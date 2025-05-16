<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import base64url from "base64url";
  import {
    account,
    server,
    native,
    fundPubkey,
    fundSigner,
    initializeWallet,
  } from "./lib/common";
  import { Keypair } from "@stellar/stellar-sdk";
  // No additional logic needed for static navbar
  let activeTab = "Home";
  const tabs = ["Home", "Dashboard", "Lend/Borrow", "Wallet"];

  let showAuthModal = false;

  // Wallet tab state
  let showSendModal = false;
  let walletBalance = "";
  let tokenBalances: { id: string; name: string; balance: string }[] = [];
  let recentTransactions: any[] = [];
  let currentPage = 0;
  let txLoading = false;
  let selectedSendToken = "";
  let recipientAddress = "";
  let sendAmount = "";
  let sendingInProgress = false;
  let sendResult: { success: boolean; message: string } | null = null;

  // --- Proximity Glow for Token Cards ---
  let mouse = { x: 0, y: 0 };
  let cardGlows: string[] = [];

  function updateGlow(e: MouseEvent) {
    mouse.x = e.clientX;
    mouse.y = e.clientY;
    // Defer calculation to after DOM update
    requestAnimationFrame(calcAllCardGlows);
  }

  function calcAllCardGlows() {
    const cards = document.querySelectorAll<HTMLElement>(".token-card");
    cardGlows = Array.from(cards).map((card) => {
      const rect = card.getBoundingClientRect();
      const left = Math.abs(mouse.x - rect.left);
      const right = Math.abs(mouse.x - rect.right);
      const top = Math.abs(mouse.y - rect.top);
      const bottom = Math.abs(mouse.y - rect.bottom);
      const minDist = Math.min(left, right, top, bottom);
      let direction = "top";
      if (minDist === left) direction = "left";
      else if (minDist === right) direction = "right";
      else if (minDist === bottom) direction = "bottom";
      const radius = 120;
      if (minDist > radius) return "";
      const intensity = 0.7 * (1 - minDist / radius); // max 0.7 opacity
      let gradient = "";
      switch (direction) {
        case "top":
          gradient = `linear-gradient(to bottom, rgba(162,89,255,${intensity}) 0%, transparent 80%)`;
          break;
        case "bottom":
          gradient = `linear-gradient(to top, rgba(162,89,255,${intensity}) 0%, transparent 80%)`;
          break;
        case "left":
          gradient = `linear-gradient(to right, rgba(162,89,255,${intensity}) 0%, transparent 80%)`;
          break;
        case "right":
          gradient = `linear-gradient(to left, rgba(162,89,255,${intensity}) 0%, transparent 80%)`;
          break;
      }
      return gradient;
    });
  }

  onMount(() => {
    window.addEventListener("mousemove", updateGlow);
  });
  onDestroy(() => {
    window.removeEventListener("mousemove", updateGlow);
  });

  // Token options (should be populated from your available tokens)
  const tokenOptions = [
    { id: "native", name: "XLM" },
    // Add other tokens here
  ];

  // Dashboard tab state
  let dashboardTab = "Lending";

  // Define types for assets
  type LendingAsset = {
    icon: string;
    name: string;
    amount: string;
    value: string;
    apy: string;
    days: string;
    duration: string;
    start: string;
    interest: string;
    withdrawable: string;
    txid?: string;
    dueDate?: string;
  };

  type BorrowingAsset = {
    icon: string;
    name: string;
    amount: string;
    value: string;
    apy: string;
    days: string;
    duration: string;
    start: string;
    interest: string;
    due_date: string;
    repayable: string;
    collateral: string;
    txid?: string;
  };

  // Lending data - initialize with example data and will be updated from localStorage
  let lendingAssets: LendingAsset[] = [
    {
      icon: "🌟",
      name: "XLM",
      amount: "1,200",
      value: "$480.50",
      apy: "5.2%",
      days: "0 days left",
      duration: "45 days",
      start: "Aug 15, 2023",
      interest: "+$32.84",
      withdrawable: "1,200 XLM",
    },
    {
      icon: "💲",
      name: "USDC",
      amount: "750",
      value: "$750.00",
      apy: "8.1%",
      days: "0 days left",
      duration: "30 days",
      start: "Sep 1, 2023",
      interest: "+$18.50",
      withdrawable: "750 USDC",
    },
  ];

  // Borrowing data - initialize with example data and will be updated from localStorage
  let borrowingAssets: BorrowingAsset[] = [
    {
      icon: "🌟",
      name: "XLM",
      amount: "500",
      value: "$200.25",
      apy: "6.8%",
      days: "15 days left",
      duration: "60 days",
      start: "Sep 10, 2023",
      interest: "-$12.45",
      due_date: "Nov 10, 2023",
      repayable: "512.45 XLM",
      collateral: "750 XLM",
    },
    {
      icon: "💲",
      name: "USDC",
      amount: "300",
      value: "$300.00",
      apy: "9.5%",
      days: "10 days left",
      duration: "30 days",
      start: "Oct 5, 2023",
      interest: "-$7.25",
      due_date: "Nov 5, 2023",
      repayable: "307.25 USDC",
      collateral: "450 USDC",
    },
  ];

  // Function to update dashboard data from localStorage
  function updateDashboardFromLocalStorage() {
    const storedLendings = loadLocalLendings();
    const storedBorrowings = loadLocalBorrowings();

    // Only use the data from local storage, no sample data
    lendingAssets = [...storedLendings];
    borrowingAssets = [...storedBorrowings];
  }

  // Modal state
  let showLoanModal = false;
  let modalAsset: any = null;
  let withdrawInProgress = false;
  let repayInProgress = false;

  // Lend/Borrow tab state
  let lendBorrowMode: "LEND" | "BORROW" = "LEND";
  let selectedToken: "XLM" | "USDC" = "XLM";
  let amount: string = "";
  let duration: "1" | "3" | "custom" = "3";
  let customMonths: string = "";
  let price: number = 0.29; // fallback
  let tokenBalanceMap = { XLM: 1500, USDC: 2000 };
  let isLoadingPrice = false;
  // Add loading state for lend/borrow
  let lendInProgress = false;

  // Rates (could be fetched, but hardcoded for now)
  const rates = {
    XLM: { lend: 5.2, borrow: 6.8 },
    USDC: { lend: 8.1, borrow: 9.5 },
  };

  // Auth state
  let contractId: string = "";
  let walletPublicKey: string = "";
  let isLoggedIn = false;
  let walletInitialized = false;

  // Success popup state
  let showSuccessPopup = false;
  let successMessage = "";
  let successIcon = "";
  let successTimer: any = null;

  // Add a computed APY color for dropdown and selected token
  $: apyColor = lendBorrowMode === "BORROW" ? "#ffb84d" : "#3ee86b";

  onMount(async () => {
    // Initialize the wallet system
    try {
      await initializeWallet();
      walletInitialized = true;

      // Check if user is already logged in
      const storedKeyId = localStorage.getItem("sp:keyId");
      if (storedKeyId) {
        connect(storedKeyId);
      }

      // Fetch initial price
      fetchPrice();

      // Load any existing lend/borrow data from localStorage
      updateDashboardFromLocalStorage();
    } catch (err) {
      console.error("Failed to initialize wallet:", err);
    }
  });

  async function register() {
    if (!walletInitialized) {
      alert(
        "Wallet system is still initializing. Please try again in a moment."
      );
      return;
    }

    const user = prompt("Give this passkey a name");
    if (!user) return;
    try {
      const result = await account.createWallet("Borrowhood", user);
      const { keyId: kid, contractId: cid, signedTx } = result;
      const res = await server.send(signedTx);
      contractId = cid;
      if ("publicKey" in result) walletPublicKey = result.publicKey as string;
      localStorage.setItem("sp:keyId", base64url(kid));
      isLoggedIn = true;

      // Update wallet info
      await getWalletBalance();
      await fetchTokenBalances();
      await fetchRecentTransactions();
    } catch (err: any) {
      alert(err.message);
    }
  }

  async function connect(keyId_?: string) {
    if (!walletInitialized) {
      console.warn(
        "Wallet system is still initializing. Connection attempt will be delayed."
      );
      return;
    }

    try {
      const result = await account.connectWallet({
        keyId: keyId_,
        getContractId: (keyId) => server.getContractId({ keyId }),
      });
      const { keyId: kid, contractId: cid } = result;
      contractId = cid;
      if ("publicKey" in result) walletPublicKey = result.publicKey as string;
      localStorage.setItem("sp:keyId", base64url(kid));
      isLoggedIn = true;

      // Update wallet info
      await getWalletBalance();
      await fetchTokenBalances();
      await fetchRecentTransactions();
    } catch (err: any) {
      alert(err.message);
    }
  }

  function reset() {
    localStorage.removeItem("sp:keyId");
    contractId = "";
    walletPublicKey = "";
    isLoggedIn = false;
  }

  async function fetchPrice() {
    isLoadingPrice = true;
    let id = selectedToken === "XLM" ? "stellar" : "usd-coin";
    try {
      const res = await fetch(
        `https://api.coingecko.com/api/v3/simple/price?ids=${id}&vs_currencies=usd`
      );
      const data = await res.json();
      price = selectedToken === "XLM" ? data.stellar.usd : data["usd-coin"].usd;
    } catch (e) {
      price = selectedToken === "XLM" ? 0.29 : 1;
    }
    isLoadingPrice = false;
  }

  // Wallet functions
  async function getWalletBalance() {
    if (!contractId) return;
    try {
      const { result } = await native.balance({ id: contractId });
      walletBalance = result.toString();
    } catch (error) {
      console.error("Error fetching wallet balance:", error);
      walletBalance = "0";
    }
  }

  async function fetchTokenBalances() {
    if (!contractId) return;

    tokenBalances = [
      { id: "native", name: "XLM", balance: walletBalance || "0" },
      // Add other tokens as they're available
    ];

    // For demonstration purposes - in real app would be fetched from blockchain
    if (activeTab === "Wallet") {
      await getWalletBalance();
      // Update XLM balance
      if (tokenBalances.length > 0) {
        tokenBalances[0].balance = walletBalance;
      }
    }
  }

  async function fetchRecentTransactions() {
    if (!contractId) return;

    txLoading = true;

    try {
      // In a real implementation, you would fetch transactions from the server
      // For this demo, we'll create mock data
      recentTransactions = [
        {
          type: "PAYMENT",
          amount: "100",
          asset_code: "XLM",
          from: contractId,
          to: "GBVQMKYWGELU6IKLK2U6EIIHTNW5LIUYJE7FUQPG4FAB3QQ3KAINFVYS",
          created_at: new Date().toISOString(),
        },
        {
          type: "PAYMENT",
          amount: "50",
          asset_code: "XLM",
          from: "GBVQMKYWGELU6IKLK2U6EIIHTNW5LIUYJE7FUQPG4FAB3QQ3KAINFVYS",
          to: contractId,
          created_at: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
        },
        {
          type: "PAYMENT",
          amount: "25",
          asset_code: "XLM",
          from: contractId,
          to: "GDVP6GXFUZUCL4DWI5HLF2P2YRTJ5AUJINV2Q3ZVUHJLZVOAIECOGG5V",
          created_at: new Date(Date.now() - 172800000).toISOString(), // 2 days ago
        },
      ];

      // In production, use something like:
      // if (server.getTransactions) {
      //   recentTransactions = await server.getTransactions(contractId, { limit: 10 });
      // }
    } catch (error) {
      console.error("Error fetching transactions:", error);
      recentTransactions = [];
    } finally {
      txLoading = false;
    }
  }

  async function handleSendTokenSubmit(e: Event) {
    e.preventDefault();

    if (!isLoggedIn || !contractId) {
      sendResult = { success: false, message: "Please login first" };
      return;
    }

    if (!recipientAddress || !sendAmount || Number(sendAmount) <= 0) {
      sendResult = {
        success: false,
        message: "Please fill in all fields with valid values",
      };
      return;
    }

    // Validate recipient address format for Stellar
    if (!recipientAddress.startsWith("G") || recipientAddress.length !== 56) {
      sendResult = {
        success: false,
        message: "Invalid Stellar address format. Please check and try again.",
      };
      return;
    }

    // Validate amount doesn't exceed available balance
    const currentBalance = walletBalance
      ? Number(walletBalance) / 10_000_000
      : 0;
    if (Number(sendAmount) > currentBalance) {
      sendResult = {
        success: false,
        message: `Insufficient balance. You have ${currentBalance.toFixed(7)} XLM available.`,
      };
      return;
    }

    sendingInProgress = true;
    sendResult = null;

    try {
      // Convert amount to proper units (Stellar uses 7 decimal places)
      const amountInStroops = BigInt(
        Math.floor(Number(sendAmount) * 10_000_000)
      );

      // For XLM (native asset)
      if (selectedSendToken === "native" || selectedSendToken === "") {
        const transferOp = await native.transfer({
          from: contractId,
          to: recipientAddress,
          amount: amountInStroops,
        });

        // Sign the transaction using the user's passkey
        const storedKeyId = localStorage.getItem("sp:keyId");
        if (storedKeyId) {
          await account.sign(transferOp, { keyId: storedKeyId });

          // Send the transaction
          const result = await server.send(transferOp.built!);

          // Success
          sendResult = {
            success: true,
            message: "Transaction successful! Tokens have been sent.",
          };

          // Update balances and transactions
          await getWalletBalance();
          await fetchTokenBalances();
          await fetchRecentTransactions();

          // Clear form
          setTimeout(() => {
            recipientAddress = "";
            sendAmount = "";

            // Close modal after success
            setTimeout(() => {
              showSendModal = false;
              sendResult = null;
            }, 2000);
          }, 1000);
        } else {
          throw new Error("Login session expired. Please login again.");
        }
      } else {
        // For other tokens - would implement token-specific logic here
        sendResult = {
          success: false,
          message: "Only XLM transfers are supported in this demo",
        };
      }
    } catch (error: any) {
      console.error("Send transaction error:", error);

      // Provide user-friendly error messages
      let errorMessage = "Transaction failed. Please try again.";

      if (error.message) {
        if (error.message.includes("insufficient")) {
          errorMessage = "Insufficient balance to complete this transaction.";
        } else if (error.message.includes("not found")) {
          errorMessage =
            "Recipient address not found. Please verify and try again.";
        } else if (error.message.includes("timeout")) {
          errorMessage =
            "Network timeout. Please check your connection and try again.";
        } else {
          errorMessage = error.message;
        }
      }

      sendResult = {
        success: false,
        message: errorMessage,
      };
    } finally {
      sendingInProgress = false;
    }
  }

  // When the active tab changes to Wallet, fetch the wallet data
  $: if (activeTab === "Wallet" && isLoggedIn) {
    getWalletBalance();
    fetchTokenBalances();
    fetchRecentTransactions();
  }

  // When the active tab changes to Dashboard, update data from localStorage
  $: if (activeTab === "Dashboard") {
    updateDashboardFromLocalStorage();
  }

  $: selectedToken, fetchPrice();

  $: amountUsd = amount && !isNaN(+amount) ? (+amount * price).toFixed(2) : "";
  $: showMax = lendBorrowMode === "LEND";
  $: showBalance = lendBorrowMode === "LEND";
  $: maxAmount =
    selectedToken === "XLM" && tokenBalances.length > 0
      ? Number(tokenBalances.find((t) => t.name === "XLM")?.balance || "0") /
        10_000_000
      : tokenBalanceMap[selectedToken];
  $: isValid =
    !!amount &&
    +amount > 0 &&
    (!showMax || +amount <= maxAmount) &&
    (duration === "1" ||
      duration === "3" ||
      (duration === "custom" && +customMonths > 0 && +customMonths <= 12));

  function setMax() {
    amount = maxAmount.toString();
  }

  function openAuthModal() {
    showAuthModal = true;
    document.body.style.overflow = "hidden";
  }
  function closeAuthModal() {
    showAuthModal = false;
    document.body.style.overflow = "";
  }
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") closeAuthModal();
  }
  function openLoanModal(asset: any) {
    modalAsset = asset;
    showLoanModal = true;
    document.body.style.overflow = "hidden";
  }
  function closeLoanModal() {
    showLoanModal = false;
    modalAsset = null;
    document.body.style.overflow = "";
  }

  // Function to show success popup
  function showSuccess(message: string, icon: string = "✅") {
    // Clear any existing timer
    if (successTimer) {
      clearTimeout(successTimer);
    }

    // Set popup content
    successMessage = message;
    successIcon = icon;
    showSuccessPopup = true;

    // Auto-hide after 4 seconds
    successTimer = setTimeout(() => {
      showSuccessPopup = false;
    }, 4000);
  }

  // Local storage functions for simulation
  function saveLocalLending(asset: any) {
    const storedLendings = JSON.parse(
      localStorage.getItem("borrowhood_lendings") || "[]"
    );
    storedLendings.push(asset);
    localStorage.setItem("borrowhood_lendings", JSON.stringify(storedLendings));
  }

  function saveLocalBorrowing(asset: any) {
    const storedBorrowings = JSON.parse(
      localStorage.getItem("borrowhood_borrowings") || "[]"
    );
    storedBorrowings.push(asset);
    localStorage.setItem(
      "borrowhood_borrowings",
      JSON.stringify(storedBorrowings)
    );
  }

  function loadLocalLendings() {
    const storedLendings = JSON.parse(
      localStorage.getItem("borrowhood_lendings") || "[]"
    );
    return storedLendings;
  }

  function loadLocalBorrowings() {
    const storedBorrowings = JSON.parse(
      localStorage.getItem("borrowhood_borrowings") || "[]"
    );
    return storedBorrowings;
  }

  // Handler for Lend/Borrow button
  async function handleLendBorrow() {
    if (!isValid) return;

    // Check if user is logged in
    if (!isLoggedIn || !contractId) {
      alert("Please login first");
      openAuthModal();
      return;
    }

    lendInProgress = true;

    try {
      // Set the destination wallet for simulation
      const destinationWallet =
        "GCZT3FLZ4SUUJYWKC5KSHHNG63IGHJK475QYNYA75OHJQ577A5AD5MXE";

      if (lendBorrowMode === "LEND") {
        // Calculate the amount in Stroops (Stellar's smallest unit)
        const amountInStroops = BigInt(Math.floor(Number(amount) * 10_000_000));

        // Create the transfer operation
        const transferOp = await native.transfer({
          from: contractId,
          to: destinationWallet,
          amount: amountInStroops,
        });

        // Sign the transaction using user's passkey
        const storedKeyId = localStorage.getItem("sp:keyId");
        if (!storedKeyId) {
          throw new Error("Login session expired. Please login again.");
        }

        await account.sign(transferOp, { keyId: storedKeyId });

        // Send the transaction
        const result = await server.send(transferOp.built!);

        // Create a record of the lending
        const currentDate = new Date();
        const durationMonths =
          duration === "custom" ? Number(customMonths) : Number(duration);
        const dueDate = new Date(currentDate);
        dueDate.setMonth(dueDate.getMonth() + durationMonths);

        // Calculate interest
        const principal = Number(amount);
        const interestRate = rates[selectedToken].lend;
        const interestAmount =
          principal * (interestRate / 100) * (durationMonths / 12);

        // Create the lending asset record
        const lendingAsset = {
          icon: tokenMeta[selectedToken].icon,
          name: selectedToken,
          amount: amount,
          value: `$${(principal * price).toFixed(2)}`,
          apy: `${interestRate}%`,
          days: `${durationMonths * 30} days left`,
          duration: `${durationMonths} month${durationMonths > 1 ? "s" : ""}`,
          start: currentDate.toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
          }),
          interest: `+$${interestAmount.toFixed(2)}`,
          withdrawable: `${(principal + interestAmount).toFixed(2)} ${selectedToken}`,
          txid: result.hash || "simulated-tx",
          dueDate: dueDate.toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
          }),
        };

        // Save to local storage
        saveLocalLending(lendingAsset);

        // Update the dashboard data
        updateDashboardFromLocalStorage();

        // Update wallet balance
        await getWalletBalance();
        await fetchTokenBalances();

        // Show success message
        showSuccess(
          `Successfully lent ${amount} ${selectedToken}. You will earn ${interestAmount.toFixed(2)} ${selectedToken} in interest.`,
          "💰"
        );

        // Reset form
        amount = "";
      } else if (lendBorrowMode === "BORROW") {
        // For borrowing simulation, we don't actually transfer funds out
        // but we create a record of the borrowing to display on dashboard

        const currentDate = new Date();
        const durationMonths =
          duration === "custom" ? Number(customMonths) : Number(duration);
        const dueDate = new Date(currentDate);
        dueDate.setMonth(dueDate.getMonth() + durationMonths);

        // Calculate interest
        const principal = Number(amount);
        const interestRate = rates[selectedToken].borrow;
        const interestAmount =
          principal * (interestRate / 100) * (durationMonths / 12);

        // Create the borrowing asset record
        const borrowingAsset = {
          icon: tokenMeta[selectedToken].icon,
          name: selectedToken,
          amount: amount,
          value: `$${(principal * price).toFixed(2)}`,
          apy: `${interestRate}%`,
          days: `${durationMonths * 30} days left`,
          duration: `${durationMonths} month${durationMonths > 1 ? "s" : ""}`,
          start: currentDate.toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
          }),
          interest: `-$${interestAmount.toFixed(2)}`,
          due_date: dueDate.toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
          }),
          repayable: `${(principal + interestAmount).toFixed(2)} ${selectedToken}`,
          collateral: `${(principal * 1.5).toFixed(2)} ${selectedToken}`,
          txid: "simulated-borrow-tx",
        };

        // Save to local storage
        saveLocalBorrowing(borrowingAsset);

        // Update the dashboard data
        updateDashboardFromLocalStorage();

        // Simulate adding funds to the user's wallet
        await simulateAddFunds(Number(amount));

        // Show success message
        showSuccess(
          `Successfully borrowed ${amount} ${selectedToken}. You will need to repay ${(principal + interestAmount).toFixed(2)} ${selectedToken} by ${dueDate.toLocaleDateString()}.`,
          "🏦"
        );

        // Reset form
        amount = "";
      }
    } catch (error: any) {
      console.error("Transaction error:", error);
      showSuccess(`Error: ${error.message || "Unknown error occurred"}`, "❌");
    } finally {
      lendInProgress = false;
    }
  }

  // Function to simulate adding funds for borrowing
  async function simulateAddFunds(amountToAdd: number) {
    try {
      // Convert to Stroops
      const amountInStroops = BigInt(Math.floor(amountToAdd * 10_000_000));

      // Use the demo's fundWallet approach to add funds to the user's wallet
      const { built, ...transfer } = await native.transfer({
        to: contractId,
        from: fundPubkey,
        amount: amountInStroops,
      });

      await transfer.signAuthEntries({
        address: fundPubkey,
        signAuthEntry: fundSigner.signAuthEntry,
      });

      const res = await server.send(built!);
      console.log("Fund transaction result:", res);

      // Update wallet balance
      await getWalletBalance();
      await fetchTokenBalances();
    } catch (error) {
      console.error("Error simulating add funds:", error);
    }
  }

  // Handle withdrawing from a lending position
  async function handleWithdrawLoan(asset: any) {
    if (!isLoggedIn || !contractId) {
      alert("Please login first");
      openAuthModal();
      return;
    }

    withdrawInProgress = true;

    try {
      // Parse the withdrawable amount from the asset
      const withdrawableText = asset.withdrawable;
      const withdrawableMatch = withdrawableText.match(/^([\d,.]+)\s+(\w+)$/);

      if (!withdrawableMatch) {
        throw new Error("Could not parse withdrawable amount");
      }

      const withdrawableAmount = parseFloat(
        withdrawableMatch[1].replace(/,/g, "")
      );
      const withdrawableToken = withdrawableMatch[2];

      // Add a transaction to the recent transactions list for visualization
      recentTransactions = [
        {
          type: "WITHDRAW",
          amount: withdrawableAmount.toString(),
          asset_code: withdrawableToken,
          from: "Lending Pool",
          to: contractId,
          created_at: new Date().toISOString(),
        },
        ...recentTransactions,
      ];

      // Show a transaction initiation message
      showSuccess(
        `Processing withdrawal of ${withdrawableAmount} ${withdrawableToken}...`,
        "⏳"
      );

      // Simulate adding the funds to the user's wallet
      await simulateAddFunds(withdrawableAmount);

      // Add a small delay to make the loading state visible (simulating blockchain confirmation)
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Remove the lending position from local storage
      const storedLendings = loadLocalLendings();
      const updatedLendings = storedLendings.filter(
        (lending: any) => lending.txid !== asset.txid
      );
      localStorage.setItem(
        "borrowhood_lendings",
        JSON.stringify(updatedLendings)
      );

      // Update the dashboard data
      updateDashboardFromLocalStorage();

      // Update wallet balance to show new funds
      await getWalletBalance();
      await fetchTokenBalances();

      // Close the modal
      closeLoanModal();

      // Show success message
      showSuccess(
        `Successfully withdrawn ${withdrawableAmount} ${withdrawableToken} from your lending position`,
        "💸"
      );
    } catch (error: any) {
      console.error("Error withdrawing loan:", error);
      showSuccess(`Error: ${error.message || "Unknown error occurred"}`, "❌");
    } finally {
      withdrawInProgress = false;
    }
  }

  // Handle repaying a loan
  async function handleRepayLoan(asset: any) {
    if (!isLoggedIn || !contractId) {
      alert("Please login first");
      openAuthModal();
      return;
    }

    repayInProgress = true;

    try {
      // Parse the repayable amount from the asset
      const repayableText = asset.repayable;
      const repayableMatch = repayableText.match(/^([\d,.]+)\s+(\w+)$/);

      if (!repayableMatch) {
        throw new Error("Could not parse repayable amount");
      }

      const repayableAmount = parseFloat(repayableMatch[1].replace(/,/g, ""));
      const repayableToken = repayableMatch[2];

      // Check if user has enough balance
      const currentBalance = walletBalance
        ? Number(walletBalance) / 10_000_000
        : 0;

      if (repayableToken === "XLM" && repayableAmount > currentBalance) {
        throw new Error(
          `Insufficient balance. You have ${currentBalance.toFixed(7)} XLM, but need ${repayableAmount.toFixed(7)} XLM.`
        );
      }

      // Set the destination wallet for simulation
      const destinationWallet =
        "GCZT3FLZ4SUUJYWKC5KSHHNG63IGHJK475QYNYA75OHJQ577A5AD5MXE";

      // For XLM, perform an actual transaction
      if (repayableToken === "XLM") {
        // Calculate the amount in Stroops (Stellar's smallest unit)
        const amountInStroops = BigInt(
          Math.floor(repayableAmount * 10_000_000)
        );

        // Create the transfer operation
        const transferOp = await native.transfer({
          from: contractId,
          to: destinationWallet,
          amount: amountInStroops,
        });

        // Sign the transaction using user's passkey
        const storedKeyId = localStorage.getItem("sp:keyId");
        if (!storedKeyId) {
          throw new Error("Login session expired. Please login again.");
        }

        await account.sign(transferOp, { keyId: storedKeyId });

        // Send the transaction
        const result = await server.send(transferOp.built!);
      }

      // Add a small delay to make the loading state visible (simulating blockchain confirmation)
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Remove the borrowing position from local storage
      const storedBorrowings = loadLocalBorrowings();
      const updatedBorrowings = storedBorrowings.filter(
        (borrowing: any) => borrowing.txid !== asset.txid
      );
      localStorage.setItem(
        "borrowhood_borrowings",
        JSON.stringify(updatedBorrowings)
      );

      // Update the dashboard data
      updateDashboardFromLocalStorage();

      // Close the modal
      closeLoanModal();

      // Show success message
      showSuccess(
        `Successfully repaid ${repayableAmount} ${repayableToken} loan`,
        "✅"
      );

      // Update wallet balance
      await getWalletBalance();
      await fetchTokenBalances();
    } catch (error: any) {
      console.error("Error repaying loan:", error);
      showSuccess(`Error: ${error.message || "Unknown error occurred"}`, "❌");
    } finally {
      repayInProgress = false;
    }
  }

  // Add live interest and total calculation
  $: months = duration === "custom" ? +customMonths : +duration;
  $: principal = +amount || 0;
  $: apy = rates[selectedToken].lend;
  $: interest = principal * (apy / 100) * (months / 12);
  $: total = principal + interest;
  $: interestUsd = (interest * price).toFixed(2);
  $: totalUsd = (total * price).toFixed(2);

  // Add token meta for display
  type TokenKey = "XLM" | "USDC";
  const tokenMeta: Record<
    TokenKey,
    {
      icon: string;
      name: string;
      apy: number;
      price: () => string;
    }
  > = {
    XLM: {
      icon: "🌟",
      name: "Stellar Lumens",
      apy: rates.XLM.lend,
      price: () => (isLoadingPrice ? "..." : price.toFixed(2)),
    },
    USDC: {
      icon: "💲",
      name: "USD Coin",
      apy: rates.USDC.lend,
      price: () => (isLoadingPrice ? "..." : "1.00"),
    },
  };
  let showTokenDropdown = false;
  function selectToken(t: TokenKey) {
    selectedToken = t;
    showTokenDropdown = false;
  }

  $: interestLabel =
    lendBorrowMode === "BORROW" ? "You will pay:" : "You will earn:";
  $: interestColor = lendBorrowMode === "BORROW" ? "#ffb84d" : "#3ee86b";
</script>

<div class="background-container">
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>
  <div class="blob blob-4"></div>
</div>

<div class="navbar">
  <div class="logo" style="font-family: 'New Rocker', cursive;">BORROWHOOD</div>
  <nav>
    {#each tabs as tab}
      <div
        class="nav-tab {activeTab === tab ? 'active' : ''}"
        on:click={() => (activeTab = tab)}
        on:keydown={(e) => e.key === "Enter" && (activeTab = tab)}
        tabindex="0"
        role="tab"
        aria-selected={activeTab === tab}
      >
        {tab}
        <span class="underline {activeTab === tab ? 'active' : ''}"></span>
      </div>
    {/each}
  </nav>
  <div class="right-group">
    {#if isLoggedIn}
      <button class="login-btn" on:click={reset}>Logout</button>
    {:else}
      <button class="login-btn" on:click={openAuthModal}>Sign Up/Login</button>
    {/if}
  </div>
</div>

<main>
  {#if activeTab === "Home"}
    <section class="hero">
      <h1 class="hero-title">
        <span class="gradient-text">Decentralized Lending</span> for <br />
        <span class="hero-title-bold">Everyone</span>
      </h1>
      <p class="hero-subtitle">
        A simple, secure platform built on Stellar to lend and borrow<br />
        cryptocurrency with no banks required.
      </p>
      <div class="hero-actions">
        <button
          class="hero-btn primary"
          on:click={() => (activeTab = "Lend/Borrow")}>Start Borrowing</button
        >
        <button
          class="hero-btn secondary"
          on:click={() => (activeTab = "LEND/Borrow")}>Lend & Earn</button
        >
      </div>
    </section>

    <!-- Why Borrowhood Section -->
    <section class="why-section">
      <h2 class="section-title">
        Why <span class="gradient-text">Borrowhood</span>?
      </h2>
      <div class="why-grid">
        <div class="why-item">
          <span class="why-icon">🌎</span>
          <h3>Everywhere</h3>
          <p>
            Access financial services from anywhere in the world with just an
            internet connection.
          </p>
        </div>
        <div class="why-item">
          <span class="why-icon">⚡</span>
          <h3>Instant Access</h3>
          <p>
            Get funded immediately after approval with no lengthy processing
            times.
          </p>
        </div>
        <div class="why-item">
          <span class="why-icon">🏦</span>
          <h3>No Bank Required</h3>
          <p>
            Skip the traditional banking requirements and access loans directly.
          </p>
        </div>
        <div class="why-item">
          <span class="why-icon">💰</span>
          <h3>Micro Loans</h3>
          <p>
            Borrow as little as $10 - perfect for small, urgent needs without
            the overhead.
          </p>
        </div>
        <div class="why-item">
          <span class="why-icon">🚀</span>
          <h3>Fast & Efficient</h3>
          <p>
            Our blockchain-based system processes transactions in seconds, not
            days.
          </p>
        </div>
        <div class="why-item">
          <span class="why-icon">📈</span>
          <h3>Earn While Borrowing</h3>
          <p>
            Your collateral earns interest even while you have an active loan.
          </p>
        </div>
      </div>
    </section>

    <!-- How Borrowhood Works Section -->
    <section class="how-section">
      <h2 class="section-title">
        How <span class="gradient-text">Borrowhood</span> Works
      </h2>
      <div class="how-grid">
        <div class="how-item">
          <span class="how-icon">🔐</span>
          <h3>Simple Secure Access</h3>
          <p>
            Sign in with Stellar passkey authentication. No complicated wallet
            setup required.
          </p>
        </div>
        <div class="how-item">
          <span class="how-icon">💰</span>
          <h3>Deposit & Earn</h3>
          <p>
            Lend your cryptocurrency and earn interest automatically with
            competitive rates.
          </p>
        </div>
        <div class="how-item">
          <span class="how-icon">🔄</span>
          <h3>Borrow Assets</h3>
          <p>
            Use your crypto as collateral to borrow other cryptocurrencies when
            you need them.
          </p>
        </div>
        <div class="how-item">
          <span class="how-icon">📱</span>
          <h3>Mobile-Friendly</h3>
          <p>
            Access your assets and manage your loans from any device, especially
            optimized for mobile.
          </p>
        </div>
      </div>
    </section>

    <!-- Call to Action Section -->
    <section class="cta-section">
      <h2 class="cta-title">
        <span class="gradient-text"
          >Ready to Experience the Future of Finance?</span
        >
      </h2>
      <p class="cta-desc">
        Join thousands of users in third-world countries who are already
        leveraging Borrowhood for financial freedom.
      </p>
      <div class="cta-actions">
        <button
          class="cta-btn primary"
          on:click={() => (activeTab = "Lend/Borrow")}>Start Lending</button
        >
        <button
          class="cta-btn secondary"
          on:click={() => (activeTab = "Dashboard")}>View Dashboard</button
        >
      </div>
    </section>
  {/if}
  {#if activeTab === "Dashboard"}
    <h1 class="dashboard-title">Dashboard</h1>
    <section class="portfolio-card">
      <h2 class="portfolio-title">Your Portfolio</h2>
      <div class="portfolio-tabs">
        <button
          class="tab {dashboardTab === 'Lending' ? 'active' : ''}"
          on:click={() => (dashboardTab = "Lending")}>Lending</button
        >
        <button
          class="tab {dashboardTab === 'Borrowing' ? 'active' : ''}"
          on:click={() => (dashboardTab = "Borrowing")}>Borrowing</button
        >
      </div>
      {#if dashboardTab === "Lending"}
        <div class="portfolio-table-wrap">
          {#if lendingAssets.length > 0}
            <table class="portfolio-table minimalist">
              <thead>
                <tr>
                  <th>Asset</th>
                  <th>Amount</th>
                  <th>Value (USD)</th>
                  <th>APY</th>
                  <th>Days Left</th>
                  <th>Details</th>
                </tr>
              </thead>
              <tbody>
                {#each lendingAssets as asset}
                  <tr>
                    <td
                      ><span class="asset-icon">{asset.icon}</span>
                      <span class="asset-name">{asset.name}</span></td
                    >
                    <td>{asset.amount}</td>
                    <td>{asset.value}</td>
                    <td class="apy positive">{asset.apy}</td>
                    <td>{asset.days}</td>
                    <td
                      ><button
                        class="details-btn"
                        on:click={() => openLoanModal(asset)}>Details</button
                      ></td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else}
            <div class="empty-dashboard">
              <div class="empty-icon">💼</div>
              <h3>No Lending Activity Yet</h3>
              <p>Start lending your assets to earn interest</p>
              <button
                class="empty-action-btn"
                on:click={() => {
                  activeTab = "Lend/Borrow";
                  lendBorrowMode = "LEND";
                }}
              >
                Start Lending
              </button>
            </div>
          {/if}
        </div>
      {/if}
      {#if dashboardTab === "Borrowing"}
        <div class="portfolio-table-wrap">
          {#if borrowingAssets.length > 0}
            <table class="portfolio-table minimalist">
              <thead>
                <tr>
                  <th>Asset</th>
                  <th>Amount</th>
                  <th>Value (USD)</th>
                  <th>APY</th>
                  <th>Days Left</th>
                  <th>Details</th>
                </tr>
              </thead>
              <tbody>
                {#each borrowingAssets as asset}
                  <tr>
                    <td
                      ><span class="asset-icon">{asset.icon}</span>
                      <span class="asset-name">{asset.name}</span></td
                    >
                    <td>{asset.amount}</td>
                    <td>{asset.value}</td>
                    <td class="apy negative">{asset.apy}</td>
                    <td>{asset.days}</td>
                    <td
                      ><button
                        class="details-btn"
                        on:click={() => openLoanModal(asset)}>Details</button
                      ></td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else}
            <div class="empty-dashboard">
              <div class="empty-icon">🏦</div>
              <h3>No Borrowing Activity Yet</h3>
              <p>Borrow assets with your crypto as collateral</p>
              <button
                class="empty-action-btn"
                on:click={() => {
                  activeTab = "Lend/Borrow";
                  lendBorrowMode = "BORROW";
                }}
              >
                Start Borrowing
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </section>
  {/if}
  {#if activeTab === "Lend/Borrow"}
    <section class="lendborrow-section">
      <div class="lendborrow-toggle-row">
        <div class="lendborrow-toggle">
          <button
            class="toggle-btn {lendBorrowMode === 'LEND' ? 'active' : ''}"
            on:click={() => (lendBorrowMode = "LEND")}>LEND</button
          >
          <button
            class="toggle-btn {lendBorrowMode === 'BORROW' ? 'active' : ''}"
            on:click={() => (lendBorrowMode = "BORROW")}>BORROW</button
          >
        </div>
      </div>
      <div class="lendborrow-main">
        <div class="lend-form-card">
          <div class="form-row form-title-row">
            <span class="form-title">Select Token</span>
            <span class="form-help">?</span>
          </div>
          <div class="form-row">
            <div
              class="token-select custom-token-select"
              tabindex="0"
              on:click={() => (showTokenDropdown = !showTokenDropdown)}
              on:blur={() => (showTokenDropdown = false)}
            >
              <div class="token-option-selected token-option-2col">
                <div class="token-row">
                  <span class="token-icon">{tokenMeta[selectedToken].icon}</span
                  >
                  <span class="token-ticker">{selectedToken}</span>
                  <span class="token-name">{tokenMeta[selectedToken].name}</span
                  >
                  <span class="token-apy" style="color: {apyColor}"
                    >{tokenMeta[selectedToken].apy}% APY</span
                  >
                  <span class="token-caret">▼</span>
                </div>
              </div>
              {#if showTokenDropdown}
                <div class="token-dropdown-list">
                  {#each Object.keys(tokenMeta) as t}
                    <div
                      class="token-option token-option-2col {selectedToken === t
                        ? 'active'
                        : ''}"
                      on:click={() => selectToken(t as TokenKey)}
                    >
                      <div class="token-row">
                        <span class="token-icon"
                          >{tokenMeta[t as TokenKey].icon}</span
                        >
                        <span class="token-ticker">{t}</span>
                        <span class="token-name"
                          >{tokenMeta[t as TokenKey].name}</span
                        >
                        <span
                          class="token-apy"
                          style="color: {lendBorrowMode === 'BORROW'
                            ? '#ffb84d'
                            : '#3ee86b'}"
                          >{tokenMeta[t as TokenKey].apy}% APY</span
                        >
                        <span class="token-caret">▼</span>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </div>
          <div class="form-row">
            <div class="form-label-row">
              <span class="form-title">
                Amount{amountUsd && ` (${amountUsd} USD)`}
              </span>
              {#if showBalance && lendBorrowMode === "LEND"}
                <span class="form-balance">
                  Balance: {parseFloat(
                    (
                      Number(
                        tokenBalances.find((t) => t.name === selectedToken)
                          ?.balance || "0"
                      ) / 10_000_000
                    ).toFixed(7)
                  )}
                  {selectedToken}
                  <span class="form-balance-usd">
                    ({(
                      (Number(
                        tokenBalances.find((t) => t.name === selectedToken)
                          ?.balance || 0
                      ) /
                        10_000_000) *
                      price
                    ).toFixed(2)} USD)
                  </span>
                </span>
              {:else if lendBorrowMode === "BORROW"}
                <span class="form-balance form-balance-right">
                  Max Borrow Capacity: 5338.98 XLM<br />(1575.00 USD)
                </span>
              {/if}
            </div>
            <div class="amount-input-row">
              <input
                class="amount-input"
                type="number"
                min="0"
                bind:value={amount}
                placeholder="Lend amount"
              />
              {#if showMax}
                <button class="max-btn" on:click={setMax}>MAX</button>
              {/if}
            </div>
          </div>
          <div class="form-row">
            <span class="form-title">Duration (Lending Period)</span>
            <div class="duration-row">
              <button
                class="duration-btn {duration === '1' ? 'active' : ''}"
                on:click={() => {
                  duration = "1";
                  customMonths = "";
                }}>1 Month</button
              >
              <button
                class="duration-btn {duration === '3' ? 'active' : ''}"
                on:click={() => {
                  duration = "3";
                  customMonths = "";
                }}>3 Months</button
              >
              <button
                class="duration-btn {duration === 'custom' ? 'active' : ''}"
                on:click={() => (duration = "custom")}
                style="position: relative;"
              >
                Custom
              </button>
            </div>
          </div>
          {#if duration === "custom"}
            <input
              class="custom-months-input-below"
              type="number"
              min="1"
              max="12"
              bind:value={customMonths}
              placeholder="Enter Duration in Months"
              aria-label="Custom months"
              style="margin-top: 0.7rem; width: 100%;"
            />
          {/if}
          <button
            class="lend-btn"
            disabled={!isValid || lendInProgress}
            on:click={handleLendBorrow}
          >
            {#if lendInProgress}
              <span class="btn-spinner"></span> Processing...
            {:else}
              {lendBorrowMode === "LEND" ? "Lend" : "Borrow"} {selectedToken}
            {/if}
          </button>
        </div>
        <div class="right-column">
          {#if principal > 0 && months > 0}
            <div class="interest-summary">
              <div class="interest-label">{interestLabel}</div>
              <div class="interest-amount" style="color: {interestColor}">
                {lendBorrowMode === "BORROW" ? "-" : "+"}{interest.toFixed(4)}
                {selectedToken} (${interestUsd})
              </div>
              <div class="interest-divider"></div>
              <div class="interest-total-row">
                <span class="interest-total-label"
                  >Total to {lendBorrowMode === "BORROW"
                    ? "repay"
                    : "receive"}:</span
                >
                <span class="interest-total-value">
                  {total.toFixed(4)}
                  {selectedToken} (${totalUsd})
                </span>
              </div>
            </div>
          {/if}
          <div class="rates-card">
            <div class="rates-title">Current Rates</div>
            <div class="rates-row">
              <div class="rates-asset">XLM</div>
              <div class="rates-info">
                <span class="rates-label">Lending Interest</span>
                <span class="rates-value lend">{rates.XLM.lend}%</span>
                <span class="rates-label">Borrow Interest</span>
                <span class="rates-value borrow">{rates.XLM.borrow}%</span>
              </div>
            </div>
            <div class="rates-divider"></div>
            <div class="rates-row">
              <div class="rates-asset">USDC</div>
              <div class="rates-info">
                <span class="rates-label">Lending Interest</span>
                <span class="rates-value lend">{rates.USDC.lend}%</span>
                <span class="rates-label">Borrow Interest</span>
                <span class="rates-value borrow">{rates.USDC.borrow}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  {/if}
  {#if activeTab === "Wallet"}
    <section class="wallet-section">
      {#if isLoggedIn}
        <div class="wallet-container">
          <div class="wallet-header">
            <h1 class="wallet-title">
              <span class="gradient-text">Your Wallet</span>
            </h1>
            <div class="wallet-address-container">
              <div class="wallet-address-label">Wallet Address:</div>
              <div class="wallet-address">
                {walletPublicKey || contractId || "Not connected"}
                <button
                  class="copy-btn"
                  on:click={() => {
                    navigator.clipboard.writeText(
                      walletPublicKey || contractId
                    );
                    alert("Address copied to clipboard!");
                  }}
                >
                  Copy
                </button>
              </div>
            </div>
            <div class="wallet-balance">
              <div class="balance-label">XLM Balance:</div>
              <div class="balance-amount gradient-text">
                {walletBalance
                  ? parseFloat((Number(walletBalance) / 10_000_000).toFixed(7))
                  : "0"} XLM
              </div>
            </div>
            <button
              class="send-btn primary pulse-animation"
              on:click={() => (showSendModal = true)}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="send-icon"
                ><path d="M22 2L11 13"></path><path
                  d="M22 2L15 22L11 13L2 9L22 2Z"
                ></path></svg
              >
              Send Tokens
            </button>
          </div>

          <div class="wallet-holdings">
            <h2 class="section-heading">Token Holdings</h2>
            <div class="holdings-grid">
              {#if tokenBalances && tokenBalances.length > 0}
                {#each tokenBalances as token, i}
                  <div
                    class="token-card"
                    style={cardGlows[i] ? `--glow: ${cardGlows[i]}` : ""}
                  >
                    <div class="token-icon">
                      {token.name === "XLM" ? "🌟" : "💰"}
                    </div>
                    <div class="token-details">
                      <div class="token-name">{token.name}</div>
                      <div class="token-balance">
                        {parseFloat(
                          (Number(token.balance) / 10_000_000).toFixed(7)
                        )}
                      </div>
                    </div>
                  </div>
                {/each}
              {:else}
                <div class="empty-state">No tokens found in your wallet</div>
              {/if}
            </div>
          </div>
        </div>
      {:else}
        <div class="wallet-auth-prompt">
          <h2 class="auth-prompt-title">
            <span class="gradient-text">Connect Your Wallet</span>
          </h2>
          <p class="auth-prompt-message">
            Please sign in to access your wallet features and manage your
            assets.
          </p>
          <button class="auth-btn primary" on:click={openAuthModal}>
            Sign Up/Login
          </button>
        </div>
      {/if}
    </section>
  {/if}
</main>

{#if showAuthModal}
  <div
    class="modal-overlay"
    on:click={closeAuthModal}
    on:keydown={handleKeydown}
    tabindex="-1"
    role="dialog"
  >
    <div
      class="auth-modal"
      on:click|stopPropagation
      on:keydown|stopPropagation
      role="dialog"
      aria-labelledby="auth-title"
    >
      <h2 id="auth-title" class="auth-title">Login/Sign Up</h2>
      <p class="auth-desc">
        Use your Passkey for secure, passwordless access. Passkeys are safer,
        faster, and protect you from phishing. Enjoy seamless login and account
        creation with modern security.
      </p>
      <div class="auth-actions">
        {#if isLoggedIn}
          <button class="auth-btn primary" on:click={reset}>Logout</button>
        {/if}
        {#if !isLoggedIn}
          <button class="auth-btn primary" on:click={() => connect()}
            >Login with Passkey</button
          >
          <button class="auth-btn secondary" on:click={register}
            >Create a New Account</button
          >
        {/if}
      </div>
    </div>
  </div>
{/if}

{#if showLoanModal && modalAsset}
  <div
    class="modal-overlay"
    on:click={closeLoanModal}
    on:keydown={(e) => e.key === "Escape" && closeLoanModal()}
    tabindex="-1"
    role="dialog"
  >
    <div
      class="loan-modal"
      on:click|stopPropagation
      on:keydown|stopPropagation
      role="dialog"
      aria-labelledby="loan-modal-title"
    >
      <div class="loan-modal-header">
        <span id="loan-modal-title" class="loan-modal-title"
          >{dashboardTab === "Lending" ? "Withdraw" : "Repay"}
          {modalAsset.name}</span
        >
        <button class="loan-modal-close" on:click={closeLoanModal}
          >&times;</button
        >
      </div>
      <div class="loan-modal-asset">
        <span class="loan-modal-asset-icon">{modalAsset.icon}</span>
        <div class="loan-modal-asset-info">
          <div class="loan-modal-asset-name">{modalAsset.name}</div>
          <div class="loan-modal-asset-type">
            {dashboardTab === "Lending" ? "Lending" : "Borrowing"} Position
          </div>
        </div>
      </div>
      <div class="loan-modal-divider"></div>
      <div class="loan-modal-row">
        <span
          >{dashboardTab === "Lending"
            ? "Amount Deposited:"
            : "Amount Borrowed:"}</span
        >
        <span class="loan-modal-value"
          >{modalAsset.amount} {modalAsset.name}</span
        >
      </div>
      <div class="loan-modal-row">
        <span>Value:</span><span class="loan-modal-value"
          >{modalAsset.value}</span
        >
      </div>
      <div class="loan-modal-row">
        <span>Duration:</span><span class="loan-modal-value"
          >{modalAsset.duration}</span
        >
      </div>
      <div class="loan-modal-row">
        <span>Start Date:</span><span class="loan-modal-value loan-modal-bold"
          >{modalAsset.start}</span
        >
      </div>
      {#if dashboardTab === "Borrowing" && modalAsset.due_date}
        <div class="loan-modal-row">
          <span>Due Date:</span><span class="loan-modal-value loan-modal-bold"
            >{modalAsset.due_date}</span
          >
        </div>
      {/if}
      <div class="loan-modal-row">
        <span>APY:</span><span class="loan-modal-value">{modalAsset.apy}</span>
      </div>
      <div class="loan-modal-row">
        <span
          >{dashboardTab === "Lending"
            ? "Interest Earned:"
            : "Interest Cost:"}</span
        >
        <span
          class="loan-modal-value {dashboardTab === 'Lending'
            ? 'loan-modal-green'
            : 'loan-modal-orange'}">{modalAsset.interest}</span
        >
      </div>
      {#if dashboardTab === "Borrowing" && modalAsset.collateral}
        <div class="loan-modal-row">
          <span>Collateral:</span><span class="loan-modal-value"
            >{modalAsset.collateral}</span
          >
        </div>
      {/if}
      <div class="loan-modal-divider"></div>
      <div class="loan-modal-row loan-modal-total">
        <span
          >{dashboardTab === "Lending"
            ? "Total Withdrawable:"
            : "Total Repayable:"}</span
        >
        <span class="loan-modal-value"
          >{dashboardTab === "Lending"
            ? modalAsset.withdrawable
            : modalAsset.repayable}</span
        >
      </div>
      <!-- Button row at bottom -->
      <div class="loan-modal-actions">
        <button
          class="loan-modal-btn cancel"
          on:click={closeLoanModal}
          disabled={withdrawInProgress || repayInProgress}>Cancel</button
        >
        {#if dashboardTab === "Lending"}
          <button
            class="loan-modal-btn withdraw"
            disabled={withdrawInProgress}
            on:click={() => handleWithdrawLoan(modalAsset)}
          >
            {#if withdrawInProgress}
              <div class="btn-spinner"></div>
              Processing...
            {:else}
              Withdraw
            {/if}
          </button>
        {:else if dashboardTab === "Borrowing"}
          <button
            class="loan-modal-btn repay"
            disabled={repayInProgress}
            on:click={() => handleRepayLoan(modalAsset)}
          >
            {#if repayInProgress}
              <div class="btn-spinner"></div>
              Processing...
            {:else}
              Repay
            {/if}
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

{#if showSuccessPopup}
  <div
    class="success-popup"
    class:show={showSuccessPopup}
    on:click={() => (showSuccessPopup = false)}
  >
    <div class="success-icon">{successIcon}</div>
    <div class="success-message">{successMessage}</div>
  </div>
{/if}

{#if showSendModal}
  <div
    class="modal-overlay"
    on:click={() => (showSendModal = false)}
    on:keydown={(e) => e.key === "Escape" && (showSendModal = false)}
    tabindex="-1"
    role="dialog"
  >
    <div
      class="send-modal"
      on:click|stopPropagation
      on:keydown|stopPropagation
      role="dialog"
      aria-labelledby="send-modal-title"
    >
      <div class="send-modal-header">
        <h3 id="send-modal-title" class="send-modal-title">Send Tokens</h3>
        <button
          class="send-modal-close"
          on:click={() => (showSendModal = false)}>&times;</button
        >
      </div>

      <form on:submit|preventDefault={handleSendTokenSubmit} class="send-form">
        <div class="form-group">
          <label for="token-select">Select Token</label>
          <div class="styled-select-wrapper">
            <select
              id="token-select"
              bind:value={selectedSendToken}
              class="token-select styled-select"
            >
              {#each tokenOptions as token}
                <option value={token.id}>{token.name}</option>
              {/each}
            </select>
            <div class="select-arrow">▼</div>
          </div>
        </div>

        <div class="form-group">
          <label for="recipient-address">Recipient Address</label>
          <div class="input-wrapper">
            <input
              type="text"
              id="recipient-address"
              bind:value={recipientAddress}
              placeholder="Enter wallet address"
              class="form-input"
              required
            />
            <div class="input-info">Start with G and 56 characters long</div>
          </div>
        </div>

        <div class="form-group">
          <label for="send-amount">Amount</label>
          <div class="input-wrapper">
            <input
              type="number"
              id="send-amount"
              bind:value={sendAmount}
              placeholder="0.0"
              min="0"
              step="any"
              class="form-input"
              required
            />
            <div class="input-info">
              Available: {walletBalance
                ? parseFloat((Number(walletBalance) / 10_000_000).toFixed(7))
                : "0"} XLM
            </div>
          </div>
        </div>

        <button
          type="submit"
          class="send-submit-btn"
          disabled={sendingInProgress ||
            !recipientAddress ||
            !sendAmount ||
            Number(sendAmount) <= 0}
        >
          {#if sendingInProgress}
            <div class="btn-spinner"></div>
            Processing...
          {:else}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="send-icon"
              ><path d="M22 2L11 13"></path><path
                d="M22 2L15 22L11 13L2 9L22 2Z"
              ></path></svg
            >
            Send Tokens
          {/if}
        </button>

        {#if sendResult}
          <div
            class={`send-result ${sendResult.success ? "success" : "error"}`}
          >
            {sendResult.message}
          </div>
        {/if}
      </form>
    </div>
  </div>
{/if}

<style>
  @import url("https://fonts.googleapis.com/css2?family=Jaro:opsz@6..72&family=New+Rocker&family=Outfit:wght@100..900&display=swap");
  :global(body) {
    margin: 0;
    font-family: "Outfit", sans-serif;
    color: #fff;
    /* Remove old background, handled by .bg-gradient */
    background: none;
  }
  /* Fullscreen deep blue-purple-violet gradient background */
  :global(body)::before {
    content: "";
    position: fixed;
    z-index: -1;
    inset: 0;
    width: 100vw;
    height: 100vh;
    background: linear-gradient(
      120deg,
      #040613 0%,
      #120a22 40%,
      #24144a 70%,
      #43207a 100%
    );
    /* even darker deep blue, purple, violet */
    opacity: 1;
    pointer-events: none;
    background-size: 200% 200%;
    animation: gradientMove 18s ease-in-out infinite;
  }
  @keyframes gradientMove {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
  :global(#app) {
    width: 100%;
    max-width: 100%;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .navbar {
    width: 100%;
    max-width: 100%;
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    padding: 0 2.5rem;
    height: 64px;
    background: transparent;
    box-shadow: none;
    position: relative;
    z-index: 10;
    overflow: hidden;
    min-width: 0;
    margin: 0 auto 1.5rem auto;
    border-bottom: 1px solid rgba(162, 89, 255, 0.2);
    flex-shrink: 0;
  }
  .logo {
    font-size: 1.9rem;
    font-weight: 800;
    color: #8ecaff;
    letter-spacing: 2px;
    text-shadow:
      0 0 10px #a259ff,
      0 0 20px #a259ff44;
    user-select: none;
    margin-right: 2.5rem;
    font-family: "New Rocker", cursive;
    white-space: nowrap;
    min-width: 0;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
  nav {
    display: flex;
    gap: 2.5rem;
    align-items: center;
    flex-shrink: 1;
    justify-content: flex-start;
    min-width: 0;
    overflow: hidden;
    margin-left: 1rem;
  }
  .nav-tab {
    position: relative;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    color: rgba(255, 255, 255, 0.7);
    padding: 0.75rem 0;
    transition: all 0.2s ease;
    font-family: "Outfit", sans-serif;
    white-space: nowrap;
    min-width: 0;
  }
  .nav-tab:hover,
  .nav-tab.active {
    color: #fff;
  }
  .underline {
    display: block;
    position: absolute;
    left: 0;
    right: 0;
    bottom: -2px;
    height: 3px;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    border-radius: 3px;
    opacity: 0;
    transform: scaleX(0.5);
    transition:
      opacity 0.2s ease,
      transform 0.2s ease;
  }
  .nav-tab:hover .underline,
  .nav-tab.active .underline {
    opacity: 1;
    transform: scaleX(1);
  }
  .right-group {
    display: flex;
    align-items: center;
    gap: 1.8rem;
    min-width: 0;
    flex-shrink: 0;
    margin-left: 2.5rem;
  }
  .login-btn {
    font-family: "Outfit", sans-serif;
    font-weight: 600;
    font-size: 0.95rem;
    padding: 0.65rem 1.4rem;
    border: none;
    border-radius: 10px;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    cursor: pointer;
    box-shadow: 0 2px 12px 0 rgba(162, 89, 255, 0.35);
    transition:
      transform 0.15s ease,
      box-shadow 0.15s ease,
      background 0.2s ease;
    outline: none;
    white-space: nowrap;
    min-width: 0;
    letter-spacing: 0.2px;
  }
  .login-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 20px 0 rgba(162, 89, 255, 0.5);
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
  }
  main {
    font-family: "Outfit", sans-serif;
    min-height: 80vh;
    padding: 2rem;
    width: 100%;
    max-width: 100%;
    box-sizing: border-box;
  }
  .dashboard-title {
    width: 100%;
    text-align: left;
    padding: 0 1rem;
    box-sizing: border-box;
    max-width: 1100px;
    margin: 0 auto 2rem;
  }
  .portfolio-card,
  .lendborrow-section {
    width: 100%;
    box-sizing: border-box;
  }
  @media (max-width: 900px) {
    .navbar {
      padding: 0 1.5rem;
      height: 60px;
      margin: 0 auto 1rem auto;
    }
    nav {
      gap: 1.5rem;
      margin-left: 0.5rem;
    }
    .logo {
      font-size: 1.5rem;
      margin-right: 1.5rem;
    }
    .login-btn {
      font-size: 0.9rem;
      padding: 0.5rem 1rem;
    }
  }
  @media (max-width: 600px) {
    .navbar {
      flex-direction: column;
      height: auto;
      padding: 0.7rem 1rem;
      align-items: flex-start;
      gap: 0.7rem;
    }
    nav {
      width: 100%;
      justify-content: flex-start;
      gap: 1rem;
      margin-left: 0;
      overflow-x: auto;
      padding-bottom: 0.5rem;
    }
    .nav-tab {
      font-size: 0.9rem;
      padding: 0.5rem 0;
    }
    .right-group {
      align-self: flex-end;
      margin-left: 0;
      margin-top: -2.5rem;
    }
  }
  .hero {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: calc(100vh - 64px - 1.5rem);
    width: 100%;
    text-align: center;
    margin: 0;
    padding: 3rem 1rem;
    box-sizing: border-box;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    background: rgba(30, 20, 60, 0.2);
    border-radius: 2rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow:
      0 4px 30px rgba(0, 0, 0, 0.1),
      inset 0 0 60px rgba(162, 89, 255, 0.1);
  }

  .hero-title {
    font-size: 3.2rem;
    font-weight: 700;
    margin-bottom: 1.2rem;
    line-height: 1.1;
    letter-spacing: -1px;
    text-shadow: 0 2px 10px rgba(162, 89, 255, 0.6);
  }

  .gradient-text {
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
    display: inline-block;
    filter: drop-shadow(0 2px 8px rgba(150, 120, 220, 0.5));
  }

  .hero-title-bold {
    font-size: 3.4rem;
    font-weight: 900;
    color: #fff;
    margin-top: 0.2rem;
    display: inline-block;
    text-shadow: 0 2px 10px rgba(162, 89, 255, 0.6);
  }

  .hero-subtitle {
    font-size: 1.3rem;
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 2.5rem;
    margin-top: 0.5rem;
    font-weight: 400;
    line-height: 1.5;
    max-width: 700px;
  }

  .hero-actions {
    display: flex;
    gap: 1.5rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .hero-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 0.9rem 2.5rem;
    border: none;
    border-radius: 2rem;
    cursor: pointer;
    transition:
      background 0.2s,
      box-shadow 0.2s,
      transform 0.13s;
    box-shadow: 0 2px 16px 0 rgba(0, 0, 0, 0.2);
    margin-bottom: 0.5rem;
    position: relative;
    overflow: hidden;
  }

  .hero-btn.primary {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    box-shadow:
      0 5px 15px rgba(162, 89, 255, 0.3),
      inset 0 1px 1px rgba(255, 255, 255, 0.4);
  }

  .hero-btn.primary::before {
    content: "";
    position: absolute;
    top: 0;
    left: -50%;
    width: 150%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.2),
      transparent
    );
    transform: skewX(-20deg);
    transition: 0.5s;
  }

  .hero-btn.primary:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    transform: translateY(-2px) scale(1.04);
    box-shadow:
      0 8px 25px rgba(162, 89, 255, 0.5),
      inset 0 1px 1px rgba(255, 255, 255, 0.4);
  }

  .hero-btn.primary:hover::before {
    left: 100%;
  }

  .hero-btn.secondary {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: 1.5px solid rgba(162, 89, 255, 0.5);
    backdrop-filter: blur(5px);
    -webkit-backdrop-filter: blur(5px);
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.1),
      inset 0 1px 1px rgba(255, 255, 255, 0.1);
  }

  .hero-btn.secondary:hover {
    background: rgba(162, 89, 255, 0.15);
    border-color: #38b6ff;
    transform: translateY(-2px) scale(1.04);
    box-shadow:
      0 8px 25px rgba(56, 182, 255, 0.25),
      inset 0 1px 1px rgba(255, 255, 255, 0.2);
  }

  .why-section,
  .how-section {
    margin: 4.5rem auto 0 auto;
    max-width: 1100px;
    padding: 0 1.2rem;
  }

  .section-title {
    font-size: 2.2rem;
    font-weight: 800;
    text-align: center;
    margin-bottom: 2.5rem;
    color: #fff;
    letter-spacing: -1px;
    text-shadow: 0 2px 10px rgba(162, 89, 255, 0.4);
  }

  .why-grid,
  .how-grid {
    display: grid;
    justify-content: center;
    gap: 2.2rem 2.5rem;
  }

  .why-grid {
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(2, 1fr);
  }

  .how-grid {
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: 1fr;
  }

  .why-item,
  .how-item {
    background: rgba(30, 20, 60, 0.15);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 1.2rem;
    padding: 2.1rem 1.3rem 1.7rem 1.3rem;
    box-shadow:
      0 4px 20px 0 rgba(0, 0, 0, 0.2),
      inset 0 1px 1px rgba(255, 255, 255, 0.07);
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    color: #fff;
    min-height: 210px;
    transition:
      box-shadow 0.3s ease,
      transform 0.3s ease,
      background 0.3s ease,
      border-color 0.3s ease,
      opacity 0.3s ease;
    border: 1px solid rgba(255, 255, 255, 0.05);
    position: relative;
    overflow: hidden;
  }

  .why-item::before,
  .how-item::before {
    content: "";
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: radial-gradient(
      circle at center,
      rgba(162, 89, 255, 0.02),
      transparent 70%
    );
    opacity: 0;
    transition: opacity 0.3s;
  }

  .why-item:hover,
  .how-item:hover {
    box-shadow:
      0 10px 30px 0 rgba(162, 89, 255, 0.4),
      inset 0 1px 2px rgba(255, 255, 255, 0.2),
      0 0 20px 5px rgba(162, 89, 255, 0.25);
    transform: translateY(-5px);
    background: rgba(30, 20, 60, 0.25);
    border: 1px solid rgba(162, 89, 255, 0.35);
    transition: all 0.2s ease;
  }

  /* Glowing edges on hover proximity */
  .why-grid:hover .why-item:not(:hover),
  .how-grid:hover .how-item:not(:hover) {
    opacity: 0.75;
  }

  .why-item,
  .how-item {
    transition: all 0.3s ease;
  }

  .why-item:hover::before,
  .how-item:hover::before {
    opacity: 1;
  }

  .why-icon,
  .how-icon {
    font-size: 2.2rem;
    margin-bottom: 1.3rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3.5rem;
    height: 3.5rem;
    background: rgba(162, 89, 255, 0.15);
    border-radius: 50%;
    box-shadow:
      0 4px 15px rgba(162, 89, 255, 0.15),
      inset 0 1px 1px rgba(255, 255, 255, 0.1);
  }

  .why-item h3,
  .how-item h3 {
    font-size: 1.18rem;
    font-weight: 700;
    margin-bottom: 0.8rem;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
    filter: drop-shadow(0 2px 5px rgba(150, 120, 220, 0.3));
  }

  .why-item p,
  .how-item p {
    font-size: 1.05rem;
    color: rgba(255, 255, 255, 0.85);
    font-weight: 400;
    margin: 0;
    line-height: 1.5;
  }

  .cta-section {
    margin: 5.5rem auto 4rem auto;
    max-width: 800px;
    padding: 3rem 3rem 3.5rem 3rem;
    text-align: center;
    background: rgba(30, 20, 60, 0.15);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 2rem;
    box-shadow:
      0 10px 30px 0 rgba(0, 0, 0, 0.15),
      inset 0 1px 1px rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.05);
    position: relative;
    overflow: hidden;
  }

  .cta-section::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: radial-gradient(
      circle at center,
      rgba(162, 89, 255, 0.1),
      transparent 70%
    );
    opacity: 1;
  }

  .cta-title {
    font-size: 2.2rem;
    font-weight: 800;
    margin-bottom: 1.5rem;
    position: relative;
    text-shadow: 0 2px 10px rgba(162, 89, 255, 0.4);
  }

  .cta-desc {
    font-size: 1.2rem;
    color: rgba(255, 255, 255, 0.85);
    margin-bottom: 2.5rem;
    position: relative;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
    line-height: 1.6;
  }

  .cta-actions {
    display: flex;
    gap: 1.5rem;
    justify-content: center;
    flex-wrap: wrap;
    position: relative;
  }

  .cta-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 0.9rem 2.5rem;
    border: none;
    border-radius: 2rem;
    cursor: pointer;
    transition:
      background 0.2s,
      box-shadow 0.2s,
      transform 0.13s;
    margin-bottom: 0.5rem;
    position: relative;
    overflow: hidden;
  }

  .cta-btn.primary {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    box-shadow:
      0 5px 15px rgba(162, 89, 255, 0.3),
      inset 0 1px 1px rgba(255, 255, 255, 0.4);
  }

  .cta-btn.primary::before {
    content: "";
    position: absolute;
    top: 0;
    left: -50%;
    width: 150%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.2),
      transparent
    );
    transform: skewX(-20deg);
    transition: 0.5s;
  }

  .cta-btn.primary:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    transform: translateY(-2px) scale(1.04);
    box-shadow:
      0 8px 25px rgba(162, 89, 255, 0.5),
      inset 0 1px 1px rgba(255, 255, 255, 0.4);
  }

  .cta-btn.primary:hover::before {
    left: 100%;
  }

  .cta-btn.secondary {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: 1.5px solid rgba(162, 89, 255, 0.5);
    backdrop-filter: blur(5px);
    -webkit-backdrop-filter: blur(5px);
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.1),
      inset 0 1px 1px rgba(255, 255, 255, 0.1);
  }

  .cta-btn.secondary:hover {
    background: rgba(162, 89, 255, 0.15);
    border-color: #38b6ff;
    transform: translateY(-2px) scale(1.04);
    box-shadow:
      0 8px 25px rgba(56, 182, 255, 0.25),
      inset 0 1px 1px rgba(255, 255, 255, 0.2);
  }

  @media (max-width: 700px) {
    .hero-title,
    .hero-title-bold {
      font-size: 2.1rem;
    }
    .hero {
      padding: 1.5rem 1rem 2.5rem 1rem;
      min-height: calc(100vh - 60px - 1rem);
    }
    .hero-actions {
      gap: 0.7rem;
    }
    .hero-btn {
      padding: 0.7rem 1.3rem;
      font-size: 1rem;
    }
    .why-grid {
      grid-template-columns: 1fr;
      grid-template-rows: repeat(6, 1fr);
      gap: 1.5rem;
    }
    .how-grid {
      grid-template-columns: 1fr;
      grid-template-rows: repeat(4, 1fr);
      gap: 1.5rem;
    }
    .cta-section {
      padding: 2rem 1.5rem;
    }
    .cta-btn {
      padding: 0.7rem 1.3rem;
      font-size: 1rem;
    }
    .cta-title {
      font-size: 1.8rem;
    }
    .cta-desc {
      font-size: 1.1rem;
    }
  }
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(20, 10, 40, 0.65);
    backdrop-filter: blur(7px);
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  .auth-modal {
    background: #18122b;
    border-radius: 1.5rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.5rem 2.2rem 2.2rem 2.2rem;
    min-width: 340px;
    max-width: 95vw;
    color: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
    animation: popIn 0.2s;
  }
  @keyframes popIn {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
  .auth-title {
    font-size: 2rem;
    font-weight: 800;
    margin-bottom: 1.1rem;
    letter-spacing: -1px;
    color: #c471f5;
    text-align: center;
  }
  .auth-desc {
    font-size: 1.1rem;
    color: #d1cbe7;
    margin-bottom: 2.1rem;
    text-align: center;
    max-width: 350px;
  }
  .auth-actions {
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
    width: 100%;
  }
  .auth-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 1rem 0;
    border: none;
    border-radius: 1.5rem;
    cursor: pointer;
    transition:
      background 0.18s,
      box-shadow 0.18s,
      color 0.18s;
    width: 100%;
    margin-bottom: 0.2rem;
  }
  .auth-btn.primary {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .auth-btn.primary:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    color: #fff;
    box-shadow: 0 4px 20px 0 #a259ff55;
  }
  .auth-btn.secondary {
    background: rgba(255, 255, 255, 0.08);
    color: #fff;
    border: 1.5px solid #a259ff;
  }
  .auth-btn.secondary:hover {
    background: rgba(162, 89, 255, 0.18);
    color: #fff;
    border-color: #38b6ff;
  }
  @media (max-width: 500px) {
    .auth-modal {
      padding: 1.2rem 0.5rem 1.2rem 0.5rem;
      min-width: 0;
    }
    .auth-title {
      font-size: 1.3rem;
    }
    .auth-desc {
      font-size: 1rem;
    }
  }
  .portfolio-card {
    background: rgba(35, 32, 50, 0.55);
    border-radius: 1.5rem;
    box-shadow:
      0 8px 48px 0 #0008,
      0 1.5px 8px 0 #a259ff22;
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1.5px solid rgba(255, 255, 255, 0.08);
    padding: 2.5rem 2.2rem 2.2rem 2.2rem;
    max-width: 1100px;
    margin: 0 auto 2.5rem auto;
    color: #fff;
    transition:
      box-shadow 0.18s,
      background 0.18s;
  }
  .portfolio-title {
    font-size: 2rem;
    font-weight: 800;
    margin-bottom: 1.5rem;
    color: #fff;
  }
  .portfolio-tabs {
    display: flex;
    gap: 2.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #232042;
  }
  .tab {
    background: none;
    border: none;
    color: #bdb8d7;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 0.7rem 0;
    cursor: pointer;
    border-bottom: 3px solid transparent;
    transition:
      color 0.18s,
      border-color 0.18s;
    outline: none;
  }
  .tab.active {
    color: #fff;
    border-image: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    border-image-slice: 1;
    border-bottom: 3px solid;
  }
  .portfolio-table-wrap {
    overflow-x: auto;
  }
  .portfolio-table.minimalist {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 1.2rem;
    background: none;
  }
  .portfolio-table.minimalist th,
  .portfolio-table.minimalist td {
    padding: 1.1rem 1.2rem;
    text-align: left;
    font-size: 1.13rem;
    font-weight: 500;
    color: #fff;
    border: none;
  }
  .portfolio-table.minimalist th {
    color: #bdb8d7;
    font-size: 1.08rem;
    font-weight: 700;
    border-bottom: 2px solid #232042;
    background: none;
  }
  .portfolio-table.minimalist tr:not(:last-child) {
    border-bottom: 1px solid #232042;
  }
  .portfolio-table.minimalist tr {
    background: none;
    transition: background 0.18s;
  }
  .portfolio-table.minimalist tr:hover {
    background: rgba(162, 89, 255, 0.07);
  }
  .asset-icon {
    font-size: 1.3rem;
    margin-right: 0.6rem;
    vertical-align: middle;
  }
  .asset-name {
    font-weight: 700;
    font-size: 1.13rem;
    color: #fff;
  }
  .apy.positive {
    color: #3ee86b;
    font-weight: 700;
  }
  .apy.negative {
    color: #ffb84d;
    font-weight: 700;
  }
  .empty-borrowing {
    padding: 2.5rem 0;
    text-align: center;
  }
  .empty-msg {
    color: #bdb8d7;
    font-size: 1.15rem;
    font-weight: 500;
    opacity: 0.7;
  }
  @media (max-width: 900px) {
    .portfolio-card {
      padding: 1.2rem 0.5rem 1.2rem 0.5rem;
    }
    .dashboard-title {
      font-size: 2rem;
      margin-left: 0.5rem;
    }
    .portfolio-title {
      font-size: 1.3rem;
    }
    .portfolio-table.minimalist th,
    .portfolio-table.minimalist td {
      padding: 0.7rem 0.5rem;
      font-size: 1rem;
    }
  }
  .details-btn {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    font-weight: 700;
    border: none;
    border-radius: 0.8rem;
    padding: 0.5rem 1.5rem;
    cursor: pointer;
    font-size: 1.08rem;
    transition:
      background 0.18s,
      box-shadow 0.18s;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .details-btn:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    box-shadow: 0 4px 20px 0 #a259ff55;
  }
  .loan-modal {
    background: #232032;
    border-radius: 1.2rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.2rem 2.2rem 1.5rem 2.2rem;
    min-width: 340px;
    max-width: 95vw;
    color: #fff;
    display: flex;
    flex-direction: column;
    animation: popIn 0.2s;
    position: relative;
  }
  .loan-modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }
  .loan-modal-title {
    font-size: 1.5rem;
    font-weight: 800;
    color: #fff;
  }
  .loan-modal-close {
    background: none;
    border: none;
    color: #fff;
    font-size: 2rem;
    font-weight: 700;
    cursor: pointer;
    margin-left: 1rem;
    transition: color 0.18s;
  }
  .loan-modal-close:hover {
    color: #a259ff;
  }
  .loan-modal-asset {
    display: flex;
    align-items: center;
    gap: 1.1rem;
    margin-bottom: 1.2rem;
  }
  .loan-modal-asset-icon {
    font-size: 2.5rem;
    background: #18122b;
    border-radius: 50%;
    width: 3.5rem;
    height: 3.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .loan-modal-asset-info {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }
  .loan-modal-asset-name {
    font-size: 1.2rem;
    font-weight: 700;
    color: #fff;
  }
  .loan-modal-asset-type {
    font-size: 1rem;
    color: #bdb8d7;
    opacity: 0.85;
  }
  .loan-modal-divider {
    border-bottom: 1.5px solid #28243a;
    margin: 1.1rem 0 1.1rem 0;
    width: 100%;
  }
  .loan-modal-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 1.08rem;
    margin-bottom: 0.7rem;
    color: #bdb8d7;
  }
  .loan-modal-row:last-child {
    margin-bottom: 0;
  }
  .loan-modal-value {
    font-weight: 700;
    color: #fff;
    font-size: 1.08rem;
  }
  .loan-modal-bold {
    font-weight: 800;
    color: #fff;
    font-size: 1.1rem;
  }
  .loan-modal-green {
    color: #3ee86b;
  }
  .loan-modal-orange {
    color: #ffb84d;
  }
  .loan-modal-total {
    font-size: 1.13rem;
    font-weight: 800;
    margin-top: 0.7rem;
  }
  @media (max-width: 500px) {
    .loan-modal {
      padding: 1.2rem 0.5rem 1.2rem 0.5rem;
      min-width: 0;
    }
    .loan-modal-title {
      font-size: 1.1rem;
    }
    .loan-modal-row,
    .loan-modal-value {
      font-size: 0.98rem;
    }
  }
  .lendborrow-section {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2.5rem 1.2rem 3rem 1.2rem;
  }
  .lendborrow-toggle-row {
    display: flex;
    justify-content: flex-start;
    margin-bottom: 2.5rem;
  }
  .lendborrow-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(35, 32, 50, 0.55);
    border-radius: 2.5rem;
    box-shadow: 0 2px 16px 0 #0002;
    padding: 0.3rem;
    gap: 0.2rem;
    min-width: 320px;
    border: 1.5px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }
  .toggle-btn {
    flex: 1 1 0;
    text-align: center;
    font-family: "Outfit", sans-serif;
    font-size: 1.2rem;
    font-weight: 700;
    border: none;
    border-radius: 2rem;
    background: none;
    color: #bdb8d7;
    padding: 0.7rem 0;
    min-width: 0;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s;
    outline: none;
    position: relative;
    z-index: 1;
  }
  .toggle-btn.active {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    box-shadow: 0 2px 12px 0 #a259ff33;
    animation: bounceToggle 0.38s cubic-bezier(0.68, -0.55, 0.27, 1.55);
  }
  @keyframes bounceToggle {
    0% {
      transform: scale(1);
    }
    30% {
      transform: scale(1.12, 0.92);
    }
    50% {
      transform: scale(0.96, 1.08);
    }
    70% {
      transform: scale(1.04, 0.98);
    }
    100% {
      transform: scale(1);
    }
  }
  .lendborrow-main {
    display: flex;
    gap: 2.5rem;
    align-items: stretch;
    flex-wrap: wrap;
    justify-content: center;
  }
  .lend-form-card,
  .rates-card {
    background: rgba(35, 32, 50, 0.55);
    border-radius: 1.5rem;
    box-shadow:
      0 8px 48px 0 #0008,
      0 1.5px 8px 0 #a259ff22;
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1.5px solid rgba(255, 255, 255, 0.08);
    padding: 2.5rem 2.2rem 2.2rem 2.2rem;
    min-width: 370px;
    max-width: 480px;
    flex: 1 1 370px;
    color: #fff;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
    transition:
      box-shadow 0.18s,
      background 0.18s;
  }
  .lend-form-card {
    margin-right: 0.5rem;
  }
  .rates-card {
    margin-left: 0.5rem;
  }
  .lendborrow-toggle-row {
    display: flex;
    justify-content: center;
    margin-bottom: 2.5rem;
  }
  .lendborrow-toggle {
    background: rgba(35, 32, 50, 0.55);
    border-radius: 2.5rem;
    box-shadow: 0 2px 16px 0 #0002;
    display: flex;
    align-items: center;
    padding: 0.3rem 0.3rem;
    gap: 0.2rem;
    min-width: 320px;
    border: 1.5px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }
  @media (max-width: 900px) {
    .lendborrow-main {
      flex-direction: column;
      gap: 2.2rem;
    }
    .lend-form-card,
    .rates-card {
      min-width: 0;
      max-width: 100%;
      margin: 0;
    }
  }
  .form-row {
    margin-bottom: 1.1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .form-title-row {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.2rem;
  }
  .form-title {
    font-size: 1.15rem;
    font-weight: 700;
    color: #fff;
  }
  .form-help {
    font-size: 1.3rem;
    color: #bdb8d7;
    background: #18122b;
    border-radius: 50%;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    cursor: pointer;
    user-select: none;
  }
  .token-select {
    width: 100%;
    background: #232032;
    border-radius: 0.8rem;
    border: 1.5px solid #28243a;
    padding: 1.2rem 1.2rem 0.7rem 1.2rem;
    display: flex;
    flex-direction: column;
    gap: 0.7rem;
    position: relative;
  }
  .token-dropdown {
    width: 100%;
    background: none;
    border: none;
    color: #fff;
    font-size: 2rem;
    font-weight: 800;
    outline: none;
    margin-bottom: 0.2rem;
    appearance: none;
    cursor: pointer;
  }
  .token-info {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    font-size: 1.1rem;
    font-weight: 700;
  }
  .token-apy {
    color: #3ee86b;
    font-size: 1.1rem;
  }
  .token-price {
    color: #38b6ff;
    font-size: 1.1rem;
  }
  .form-label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1.2rem;
  }
  .form-balance {
    color: #bdb8d7;
    font-size: 1.05rem;
    font-weight: 500;
  }
  .form-balance-usd {
    color: #38b6ff;
    font-size: 1.05rem;
    font-weight: 500;
    margin-left: 0.2rem;
  }
  .amount-input-row {
    display: flex;
    align-items: center;
    gap: 0.7rem;
  }
  .amount-input {
    flex: 1;
    background: #18122b;
    border: 1.5px solid #28243a;
    border-radius: 0.7rem;
    color: #fff;
    font-size: 1.15rem;
    font-weight: 600;
    padding: 1rem 1.2rem;
    outline: none;
    transition: border 0.18s;
  }
  .amount-input:focus {
    border: 1.5px solid #a259ff;
  }
  .max-btn {
    background: none;
    border: none;
    color: #c471f5;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 0.5rem 1.2rem;
    border-radius: 1.2rem;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s;
  }
  .max-btn:hover {
    background: #a259ff22;
    color: #fff;
  }
  .duration-row {
    display: flex;
    gap: 1.1rem;
    align-items: center;
    margin-top: 0.7rem;
    flex-wrap: wrap;
    width: 100%;
  }
  .duration-btn {
    flex: 1 1 0;
    min-width: 0;
    background: rgba(255, 255, 255, 0.04);
    border: 2px solid rgba(162, 89, 255, 0.18);
    color: #bdb8d7;
    font-size: 1.1rem;
    font-weight: 700;
    border-radius: 0.8rem;
    padding: 0.9rem 0;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s,
      border 0.18s;
    outline: none;
    text-align: center;
  }
  .duration-btn.active {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    border: 2.5px solid #38b6ff;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .custom-months-input {
    width: 3.5rem;
    background: #18122b;
    border: 2px solid #38b6ff;
    border-radius: 0.7rem;
    color: #fff;
    font-size: 1.1rem;
    font-weight: 600;
    padding: 0.7rem 0.7rem;
    outline: none;
    margin-left: 0.7rem;
    margin-right: 0.3rem;
    text-align: center;
  }
  .lend-btn {
    width: 100%;
    margin-top: 1.7rem;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    font-size: 1.2rem;
    font-weight: 800;
    border: none;
    border-radius: 1.2rem;
    padding: 1.2rem 0;
    cursor: pointer;
    box-shadow: 0 2px 12px 0 #a259ff33;
    transition:
      background 0.18s,
      box-shadow 0.18s,
      color 0.18s;
    opacity: 1;
  }
  .lend-btn:disabled {
    background: #232042;
    color: #bdb8d7;
    cursor: not-allowed;
    opacity: 0.7;
    box-shadow: none;
  }
  .rates-title {
    font-size: 1.3rem;
    font-weight: 800;
    margin-bottom: 1.5rem;
    color: #fff;
  }
  .rates-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.2rem;
    gap: 1.2rem;
  }
  .rates-asset {
    font-size: 1.3rem;
    font-weight: 800;
    color: #fff;
    min-width: 70px;
  }
  .rates-info {
    display: flex;
    align-items: center;
    gap: 1.2rem;
  }
  .rates-label {
    color: #bdb8d7;
    font-size: 1.05rem;
    font-weight: 500;
    margin-right: 0.3rem;
  }
  .rates-value.lend {
    color: #3ee86b;
    font-size: 1.15rem;
    font-weight: 700;
    margin-right: 1.2rem;
  }
  .rates-value.borrow {
    color: #ffb84d;
    font-size: 1.15rem;
    font-weight: 700;
  }
  .rates-divider {
    border-bottom: 1.5px solid #28243a;
    margin: 1.1rem 0 1.1rem 0;
    width: 100%;
  }
  .wallet-section {
    max-width: 1100px;
    margin: 0 auto;
    padding: 0 1.2rem;
  }

  .wallet-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .wallet-header {
    background: rgba(30, 20, 60, 0.55);
    border-radius: 1.2rem;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    box-shadow: 0 4px 20px 0 rgba(15, 5, 50, 0.25);
  }

  .wallet-title {
    font-size: 2.2rem;
    font-weight: 800;
    margin: 0 0 1rem 0;
  }

  .wallet-address-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .wallet-address-label {
    font-size: 1rem;
    color: #bdb8d7;
  }

  .wallet-address {
    font-size: 1rem;
    font-family: monospace;
    background: rgba(15, 10, 30, 0.5);
    padding: 1rem;
    border-radius: 0.75rem;
    word-break: break-all;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .copy-btn {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: none;
    border-radius: 0.5rem;
    padding: 0.4rem 0.8rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
    font-family: "Outfit", sans-serif;
  }

  .copy-btn:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .wallet-balance {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .balance-label {
    font-size: 1rem;
    color: #bdb8d7;
  }

  .balance-amount {
    font-size: 2rem;
    font-weight: 700;
  }

  .send-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 0.9rem 1.2rem;
    border: none;
    border-radius: 1rem;
    cursor: pointer;
    transition:
      background 0.2s,
      box-shadow 0.2s,
      transform 0.13s;
    box-shadow: 0 2px 16px 0 rgba(162, 89, 255, 0.5);
    margin-top: 1rem;
    width: 100%;
    max-width: 15rem;
    align-self: flex-end;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    position: relative;
    overflow: hidden;
  }

  .send-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 5px 20px 0 rgba(162, 89, 255, 0.7);
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
  }

  .send-icon {
    margin-right: 0.3rem;
  }

  .pulse-animation {
    position: relative;
  }

  .pulse-animation::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border-radius: 1rem;
    box-shadow: 0 0 0 0 rgba(162, 89, 255, 0.7);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0% {
      box-shadow: 0 0 0 0 rgba(162, 89, 255, 0.7);
    }
    70% {
      box-shadow: 0 0 0 10px rgba(162, 89, 255, 0);
    }
    100% {
      box-shadow: 0 0 0 0 rgba(162, 89, 255, 0);
    }
  }

  .section-heading {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0 0 1.5rem 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .refresh-btn {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border: none;
    border-radius: 0.5rem;
    padding: 0.4rem 0.8rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .refresh-btn:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .wallet-holdings {
    background: rgba(30, 20, 60, 0.55);
    border-radius: 1.2rem;
    padding: 2rem;
    box-shadow: 0 4px 20px 0 rgba(15, 5, 50, 0.25);
  }

  .holdings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .token-card {
    position: relative;
    background: rgba(15, 10, 30, 0.5);
    border-radius: 1rem;
    padding: 1.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
    transition:
      transform 0.2s,
      box-shadow 0.2s;
    overflow: hidden;
  }
  .token-card::before {
    content: "";
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 2;
    background: var(--glow, none);
    transition: background 0.13s;
    border-radius: 1rem;
  }

  .token-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 20px 0 rgba(162, 89, 255, 0.25);
    background: linear-gradient(
      90deg,
      rgba(162, 89, 255, 0.1) 0%,
      rgba(56, 182, 255, 0.1) 100%
    );
  }

  .token-icon {
    font-size: 2rem;
    width: 3rem;
    height: 3rem;
    background: linear-gradient(
      90deg,
      rgba(162, 89, 255, 0.2) 0%,
      rgba(56, 182, 255, 0.2) 100%
    );
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .token-details {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .token-name {
    font-size: 1.1rem;
    font-weight: 700;
    color: #fff;
  }

  .token-balance {
    font-size: 1.3rem;
    font-weight: 600;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }

  .transaction-history {
    background: rgba(30, 20, 60, 0.55);
    border-radius: 1.2rem;
    padding: 2rem;
    box-shadow: 0 4px 20px 0 rgba(15, 5, 50, 0.25);
  }

  .transaction-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .transaction-item {
    background: rgba(15, 10, 30, 0.5);
    border-radius: 0.75rem;
    padding: 1.2rem;
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .tx-type {
    background: linear-gradient(
      90deg,
      rgba(162, 89, 255, 0.2) 0%,
      rgba(56, 182, 255, 0.2) 100%
    );
    padding: 0.5rem 1rem;
    border-radius: 2rem;
    font-size: 0.9rem;
    font-weight: 700;
    text-transform: uppercase;
    min-width: 80px;
    text-align: center;
  }

  .tx-details {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    flex: 1;
  }

  .tx-amount {
    font-size: 1.1rem;
    font-weight: 700;
    color: #38b6ff;
  }

  .tx-addresses {
    font-size: 0.9rem;
    color: #bdb8d7;
    word-break: break-all;
    display: flex;
    flex-direction: column;
  }

  .tx-address {
    font-family: monospace;
    color: #d1cbe7;
  }

  .tx-time {
    font-size: 0.85rem;
    color: #bdb8d7;
    margin-top: 0.5rem;
  }

  .pagination {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    margin-top: 2rem;
  }

  .pagination-btn {
    background: rgba(162, 89, 255, 0.2);
    color: #fff;
    border: none;
    border-radius: 0.5rem;
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .pagination-btn:hover:not(:disabled) {
    background: rgba(162, 89, 255, 0.4);
  }

  .pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pagination-info {
    font-size: 0.9rem;
    color: #bdb8d7;
  }

  .empty-state {
    color: #bdb8d7;
    text-align: center;
    padding: 2rem 0;
    font-size: 1.1rem;
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem 0;
    gap: 1rem;
  }

  .loading-spinner {
    width: 2.5rem;
    height: 2.5rem;
    border: 3px solid rgba(162, 89, 255, 0.2);
    border-top: 3px solid #a259ff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .loading-text {
    color: #bdb8d7;
    font-size: 1rem;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .wallet-auth-prompt {
    background: rgba(30, 20, 60, 0.55);
    border-radius: 1.2rem;
    padding: 4rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2rem;
    text-align: center;
    box-shadow: 0 4px 20px 0 rgba(15, 5, 50, 0.25);
    max-width: 600px;
    margin: 2rem auto;
  }

  .auth-prompt-title {
    font-size: 2rem;
    font-weight: 800;
    margin: 0;
  }

  .auth-prompt-message {
    font-size: 1.2rem;
    color: #d1cbe7;
    max-width: 400px;
    margin: 0;
  }

  /* Send Modal Styles */
  .send-modal {
    background: rgba(24, 18, 43, 0.95);
    border-radius: 1.5rem;
    box-shadow: 0 8px 48px 0 rgba(0, 0, 0, 0.6);
    padding: 2rem;
    min-width: 340px;
    max-width: 95vw;
    width: 450px;
    color: #fff;
    display: flex;
    flex-direction: column;
    animation: popIn 0.2s;
    border: 1px solid rgba(162, 89, 255, 0.3);
    backdrop-filter: blur(10px);
  }

  .send-modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }

  .send-modal-title {
    font-size: 1.5rem;
    font-weight: 800;
    margin: 0;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }

  .send-modal-close {
    background: none;
    border: none;
    color: #fff;
    font-size: 1.8rem;
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .send-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-size: 1rem;
    color: #d1cbe7;
  }

  .form-input,
  .token-select {
    background: rgba(15, 10, 30, 0.5);
    border: 1px solid rgba(162, 89, 255, 0.3);
    color: #fff;
    font-size: 1.1rem;
    padding: 1rem;
    border-radius: 0.75rem;
    outline: none;
    transition: all 0.2s;
    width: 100%;
  }

  .form-input:focus,
  .token-select:focus {
    border-color: #a259ff;
    box-shadow: 0 0 0 2px rgba(162, 89, 255, 0.25);
  }

  .styled-select-wrapper {
    position: relative;
    width: 100%;
  }

  .styled-select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    background: rgba(15, 10, 30, 0.8);
    border: 1px solid rgba(162, 89, 255, 0.4);
    background-image: linear-gradient(
      90deg,
      rgba(162, 89, 255, 0.1) 0%,
      rgba(56, 182, 255, 0.1) 100%
    );
    cursor: pointer;
  }

  .select-arrow {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: rgba(162, 89, 255, 0.7);
    pointer-events: none;
    font-size: 0.8rem;
  }

  .send-submit-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    padding: 1rem 0;
    border: none;
    border-radius: 0.75rem;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    cursor: pointer;
    transition:
      background 0.2s,
      box-shadow 0.2s,
      transform 0.2s;
    margin-top: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    height: 3.5rem;
    box-shadow: 0 2px 10px 0 rgba(162, 89, 255, 0.3);
    position: relative;
    overflow: hidden;
  }

  .send-submit-btn::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.2),
      transparent
    );
    transition: 0.5s;
  }

  .send-submit-btn:hover:not(:disabled) {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    box-shadow: 0 4px 20px 0 rgba(162, 89, 255, 0.6);
    transform: translateY(-2px);
  }

  .send-submit-btn:hover:not(:disabled)::before {
    left: 100%;
  }

  .send-submit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: rgba(162, 89, 255, 0.3);
  }

  .input-wrapper {
    position: relative;
    width: 100%;
  }

  .input-info {
    position: absolute;
    right: 10px;
    top: 0;
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.6);
    background: rgba(15, 10, 30, 0.7);
    padding: 0 5px;
    border-radius: 0 0 5px 5px;
    transform: translateY(-100%);
    transition: all 0.2s;
    pointer-events: none;
  }

  .form-input:focus + .input-info {
    transform: translateY(0);
    color: rgba(162, 89, 255, 0.9);
  }

  .btn-spinner {
    width: 1.2rem;
    height: 1.2rem;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .send-result {
    padding: 1rem;
    border-radius: 0.75rem;
    font-size: 1rem;
    text-align: center;
    animation: fadeIn 0.2s;
    margin-top: 1rem;
    border: 1px solid transparent;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .send-result.success {
    background: rgba(62, 232, 107, 0.1);
    color: #3ee86b;
    border-color: rgba(62, 232, 107, 0.3);
    box-shadow: 0 0 15px rgba(62, 232, 107, 0.2);
  }

  .send-result.success::before {
    content: "✓";
    margin-right: 0.5rem;
    font-weight: bold;
  }

  .send-result.error {
    background: rgba(255, 71, 71, 0.1);
    color: #ff4747;
    border-color: rgba(255, 71, 71, 0.3);
    box-shadow: 0 0 15px rgba(255, 71, 71, 0.2);
  }

  .send-result.error::before {
    content: "⚠";
    margin-right: 0.5rem;
    font-weight: bold;
  }

  @media (max-width: 768px) {
    .wallet-header {
      padding: 1.5rem;
    }

    .holdings-grid {
      grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    }

    .transaction-item {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .tx-type {
      align-self: flex-start;
    }
  }

  /* Animated Background */
  .background-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    overflow: hidden;
    opacity: 0.6;
    filter: blur(70px);
    transform: scale(1.2);
  }

  .blob {
    position: absolute;
    border-radius: 50%;
    filter: blur(40px);
    opacity: 0.4;
    mix-blend-mode: screen;
    animation: float 25s infinite alternate ease-in-out;
  }

  .blob-1 {
    width: 40vw;
    height: 40vw;
    left: 10%;
    top: 20%;
    background: radial-gradient(circle at center, #a259ff, #38047c);
    animation-delay: 0s;
  }

  .blob-2 {
    width: 35vw;
    height: 35vw;
    right: 10%;
    top: 10%;
    background: radial-gradient(circle at center, #7b5aff, #18122b);
    animation-delay: -5s;
    animation-duration: 30s;
  }

  .blob-3 {
    width: 50vw;
    height: 50vw;
    left: 0%;
    bottom: 0%;
    background: radial-gradient(circle at center, #38b6ff, #231942);
    animation-delay: -10s;
    animation-duration: 22s;
  }

  .blob-4 {
    width: 45vw;
    height: 45vw;
    right: 0%;
    bottom: 10%;
    background: radial-gradient(circle at center, #c471f5, #1a1333);
    animation-delay: -15s;
    animation-duration: 28s;
  }

  @keyframes float {
    0% {
      transform: translate(0, 0) scale(1) rotate(0deg);
    }
    25% {
      transform: translate(3%, 3%) scale(1.03) rotate(2deg);
    }
    50% {
      transform: translate(-2%, 5%) scale(0.97) rotate(-1deg);
    }
    75% {
      transform: translate(-4%, -2%) scale(0.99) rotate(-3deg);
    }
    100% {
      transform: translate(2%, -4%) scale(1.02) rotate(1deg);
    }
  }

  /* Success Popup Styles */
  .success-popup {
    position: fixed;
    top: 2rem;
    right: 2rem;
    background: rgba(24, 18, 43, 0.95);
    border-radius: 1rem;
    padding: 1.2rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
    z-index: 2000;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(162, 89, 255, 0.4);
    backdrop-filter: blur(10px);
    transform: translateX(calc(100% + 2rem));
    opacity: 0;
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    max-width: 360px;
    cursor: pointer;
    animation:
      slideIn 0.5s forwards,
      slideOut 0.5s 3.5s forwards;
  }

  .success-popup.show {
    transform: translateX(0);
    opacity: 1;
  }

  @keyframes slideIn {
    0% {
      transform: translateX(calc(100% + 2rem));
      opacity: 0;
    }
    100% {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @keyframes slideOut {
    0% {
      transform: translateX(0);
      opacity: 1;
    }
    100% {
      transform: translateX(calc(100% + 2rem));
      opacity: 0;
    }
  }

  .success-icon {
    font-size: 1.8rem;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.5rem;
  }

  .success-message {
    font-size: 1rem;
    color: #fff;
    flex: 1;
    line-height: 1.4;
  }

  .lend-form-card {
    padding: 2rem 1.5rem 1.5rem 1.5rem;
    max-width: 420px;
  }
  .lend-form-card .form-title {
    font-size: 1.08rem;
    font-weight: 700;
    margin-bottom: 0.2rem;
  }
  .lend-form-card .form-help {
    font-size: 1.1rem;
    width: 1.7rem;
    height: 1.7rem;
  }
  .lend-form-card .token-select {
    width: 100%;
    box-sizing: border-box;
    padding: 0.7rem 0rem 0.4rem 0rem;
    border-radius: 0.6rem;
    min-height: unset;
    margin-bottom: 0.5rem;
  }
  .lend-form-card .token-dropdown {
    width: 100%;
    font-size: 1.05rem;
    margin-bottom: 0.05rem;
    box-sizing: border-box;
  }
  .lend-form-card .token-info {
    width: 100%;
    font-size: 0.95rem;
    gap: 0.7rem;
    display: flex;
    justify-content: space-between;
  }
  .lend-form-card .token-apy,
  .lend-form-card .token-price {
    font-size: 0.95rem;
  }
  .lend-form-card .token-select {
    min-height: 0;
    margin-bottom: 0.5rem;
  }
  .lend-form-card .form-label-row {
    font-size: 1rem;
    gap: 0.7rem;
  }
  .lend-form-card .form-balance,
  .lend-form-card .form-balance-usd {
    font-size: 0.98rem;
  }
  .lend-form-card .amount-input-row {
    gap: 0.5rem;
  }
  .lend-form-card .amount-input {
    font-size: 1rem;
    padding: 0.8rem 1rem;
    border-radius: 0.7rem;
  }
  .lend-form-card .max-btn {
    font-size: 1rem;
    padding: 0.4rem 1rem;
    border-radius: 1rem;
  }
  .lend-form-card .duration-row {
    gap: 0.7rem;
    margin-top: 0.5rem;
  }
  .lend-form-card .duration-btn {
    font-size: 1rem;
    padding: 0.7rem 1.3rem;
    border-radius: 0.7rem;
  }
  .lend-form-card .custom-months-input {
    font-size: 0.98rem;
    padding: 0.5rem 0.7rem;
    border-radius: 0.6rem;
    width: 3rem;
  }
  .lend-form-card .custom-months-label {
    font-size: 0.95rem;
  }
  .lend-form-card .lend-btn {
    font-size: 1.08rem;
    padding: 1rem 0;
    border-radius: 1rem;
    margin-top: 1.2rem;
  }
  @media (max-width: 600px) {
    .lend-form-card {
      padding: 1.1rem 0.5rem 1rem 0.5rem;
      max-width: 100%;
    }
  }
  /* Spinner for Lend/Borrow button */
  .btn-spinner {
    display: inline-block;
    width: 1.2em;
    height: 1.2em;
    border: 2.5px solid #a259ff44;
    border-top: 2.5px solid #38b6ff;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 0.7em;
    vertical-align: middle;
  }

  /* Empty dashboard state */
  .empty-dashboard {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 4rem 1rem;
    background: rgba(30, 20, 60, 0.2);
    border-radius: 1rem;
    border: 1px solid rgba(162, 89, 255, 0.2);
    margin: 2rem 0;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1.5rem;
    background: rgba(162, 89, 255, 0.1);
    width: 6rem;
    height: 6rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 30px rgba(162, 89, 255, 0.3);
  }

  .empty-dashboard h3 {
    font-size: 1.6rem;
    font-weight: 700;
    margin: 0 0 0.8rem 0;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }

  .empty-dashboard p {
    font-size: 1.1rem;
    color: rgba(255, 255, 255, 0.7);
    margin: 0 0 2rem 0;
  }

  .empty-action-btn {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: white;
    border: none;
    border-radius: 2rem;
    padding: 0.9rem 2rem;
    font-size: 1.1rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 4px 15px rgba(162, 89, 255, 0.3);
  }

  .empty-action-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(162, 89, 255, 0.5);
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
  }
  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  /* Interest summary styles */
  .interest-summary {
    margin: 1.2rem 0 0.7rem 0;
    background: rgba(35, 32, 50, 0.55);
    border-radius: 1.2rem;
    padding: 1.2rem 1.2rem 1.1rem 1.2rem;
    box-shadow:
      0 8px 32px 0 #0008,
      0 1.5px 8px 0 #a259ff22;
    border: 1.5px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    font-size: 1.08rem;
    color: #fff;
    transition:
      box-shadow 0.18s,
      background 0.18s;
  }
  .interest-label {
    font-weight: 700;
    margin-bottom: 0.5rem;
  }
  .interest-amount {
    color: #3ee86b;
    font-size: 1.15rem;
    font-weight: 800;
    margin-bottom: 0.7rem;
  }
  .interest-divider {
    border-bottom: 1px solid #28243a;
    margin: 0.7rem 0 0.7rem 0;
  }
  .interest-total-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 1.08rem;
  }
  .interest-total-label {
    color: #bdb8d7;
    font-weight: 600;
  }
  .interest-total-value {
    color: #fff;
    font-weight: 700;
  }
  .right-column {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    flex: 1 1 370px;
    min-width: 370px;
    max-width: 480px;
  }
  @media (max-width: 900px) {
    .lendborrow-main {
      flex-direction: column;
      gap: 2.2rem;
    }
    .right-column {
      min-width: 0;
      max-width: 100%;
    }
  }
  .custom-months-input-in-btn {
    width: 2.5rem;
    background: transparent;
    border: none;
    color: #fff;
    font-size: 1.1rem;
    font-weight: 700;
    outline: none;
    margin-left: 0.5rem;
    text-align: center;
    border-bottom: 2px solid #38b6ff;
    transition: border 0.18s;
  }
  .custom-months-input-in-btn:focus {
    border-bottom: 2px solid #a259ff;
  }
  .custom-explainer {
    margin-top: 0.7rem;
    color: #bdb8d7;
    font-size: 0.98rem;
    text-align: left;
    padding-left: 0.1rem;
  }
  .custom-months-input-below {
    width: 100%;
    background: #18122b;
    border: 2px solid #38b6ff;
    border-radius: 0.7rem;
    color: #fff;
    font-size: 1.1rem;
    font-weight: 600;
    padding: 0.7rem 1rem;
    outline: none;
    text-align: center;
    box-sizing: border-box;
    transition: border 0.18s;
  }
  .custom-months-input-below:focus {
    border: 2px solid #a259ff;
  }
  .token-info-row {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 0.7rem;
    padding: 0.1rem 0.2rem;
  }
  .token-ticker {
    font-weight: 800;
    font-size: 1.08rem;
    color: #fff;
    flex: 1 1 0;
    text-align: left;
    letter-spacing: 0.5px;
  }
  .token-apy {
    color: #3ee86b;
    font-size: 1.02rem;
    font-weight: 700;
    flex: 1 1 0;
    text-align: center;
  }
  .token-price {
    color: #38b6ff;
    font-size: 1.02rem;
    font-weight: 700;
    flex: 1 1 0;
    text-align: right;
  }
  /* Custom token dropdown styles */
  .custom-token-select {
    position: relative;
    width: 100%;
    background: rgba(20, 15, 40, 0.93);
    border-radius: 0.8rem;
    border: 1.5px solid #28243a;
    box-shadow: 0 2px 12px 0 #a259ff11;
    cursor: pointer;
    margin-bottom: 0.5rem;
    min-height: 3.2rem;
    transition:
      border 0.18s,
      box-shadow 0.18s;
    outline: none;
  }
  .token-option-selected,
  .token-option {
    display: flex;
    align-items: center;
    gap: 0.7rem;
    padding: 0.9rem 1.1rem;
    font-size: 1.05rem;
    font-weight: 600;
    color: #fff;
    border-radius: 0.7rem;
    background: none;
    width: 100%;
    box-sizing: border-box;
  }
  .token-option-selected {
    justify-content: space-between;
    position: relative;
  }
  .token-option {
    background: rgba(35, 32, 50, 0.55);
    margin-bottom: 0.3rem;
    transition:
      background 0.13s,
      box-shadow 0.13s;
    cursor: pointer;
  }
  .token-option.active,
  .token-option:hover {
    background: linear-gradient(90deg, #a259ff22 0%, #38b6ff22 100%);
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .token-dropdown-list {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    z-index: 10;
    background: rgba(20, 15, 40, 0.98);
    border-radius: 0.8rem;
    box-shadow: 0 8px 32px 0 #0008;
    margin-top: 0.2rem;
    padding: 0.1rem 0.05rem;
    max-height: 180px;
    overflow-y: auto;
  }
  .token-option,
  .token-option-2col {
    min-height: 36px;
    padding: 0.4rem 0.8rem;
    font-size: 0.98rem;
    gap: 0.4rem;
  }
  .token-row {
    gap: 0.4rem;
  }
  .token-icon {
    font-size: 1.1rem;
    margin-right: 0.15rem;
  }
  .token-ticker {
    font-size: 1.01rem;
  }
  .token-name {
    font-size: 0.98rem;
  }
  .token-apy {
    font-size: 0.98rem;
  }
  .token-caret {
    margin-left: auto;
    color: #bdb8d7;
    font-size: 1.1rem;
    font-weight: 700;
    transition: transform 0.18s;
  }
  .custom-token-select:focus-within,
  .custom-token-select:focus {
    border: 1.5px solid #a259ff;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .token-option-2col {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.2rem;
    padding: 0.7rem 1.1rem;
  }
  .token-row {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 0.7rem;
  }
  .token-row .token-ticker {
    font-weight: 800;
    font-size: 1.08rem;
    color: #fff;
    letter-spacing: 0.5px;
    margin-right: 0.5rem;
  }
  .token-row .token-name {
    color: #bdb8d7;
    font-size: 1.01rem;
    font-weight: 600;
  }
  .token-row .token-apy {
    color: #3ee86b;
    font-size: 1.01rem;
    font-weight: 700;
  }
  .token-row .token-price {
    color: #38b6ff;
    font-size: 1.01rem;
    font-weight: 700;
  }
  .token-option-2col .token-caret {
    align-self: flex-end;
    margin-top: 0.2rem;
  }
  .token-row .token-caret {
    margin-left: 0.7rem;
    color: #bdb8d7;
    font-size: 1.1rem;
    font-weight: 700;
    align-self: center;
    vertical-align: middle;
    transition: transform 0.18s;
  }
  .form-label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1.2rem;
  }
  .form-balance-right {
    text-align: right;
    display: block;
    width: 100%;
  }
  .loan-modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 2.2rem;
    padding-top: 1.2rem;
    border-top: 1px solid #28243a;
  }
  .loan-modal-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.05rem;
    font-weight: 700;
    padding: 0.7rem 1.7rem;
    border: none;
    border-radius: 0.8rem;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s;
    box-shadow: 0 2px 12px 0 #a259ff22;
  }
  .loan-modal-btn.cancel {
    background: rgba(255, 255, 255, 0.08);
    color: #fff;
    border: 1.5px solid #a259ff;
  }
  .loan-modal-btn.cancel:hover {
    background: rgba(162, 89, 255, 0.18);
    color: #fff;
    border-color: #38b6ff;
  }
  .loan-modal-btn.repay {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    color: #fff;
  }
  .loan-modal-btn.repay:hover {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
  }
  .loan-modal-btn.withdraw {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
  }
  .loan-modal-btn.withdraw:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
  }
</style>
