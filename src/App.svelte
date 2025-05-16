<script lang="ts">
  import { onMount } from "svelte";
  import base64url from "base64url";
  import {
    account,
    server,
    native,
    fundPubkey,
    fundSigner,
  } from "../../demo/src/lib/common";
  import { Keypair } from "@stellar/stellar-sdk";
  // No additional logic needed for static navbar
  let activeTab = "Home";
  const tabs = ["Home", "Dashboard", "Lend/Borrow", "Wallet"];

  let showAuthModal = false;

  // Dashboard tab state
  let dashboardTab = "Lending";

  // Lending data
  const lendingAssets = [
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

  // Modal state
  let showLoanModal = false;
  let modalAsset: any = null;

  // Lend/Borrow tab state
  let lendBorrowMode: "LEND" | "BORROW" = "LEND";
  let selectedToken: "XLM" | "USDC" = "XLM";
  let amount: string = "";
  let duration: "1" | "3" | "custom" = "3";
  let customMonths: string = "";
  let price: number = 0.29; // fallback
  let balance = { XLM: 1500, USDC: 2000 };
  let isLoadingPrice = false;

  // Rates (could be fetched, but hardcoded for now)
  const rates = {
    XLM: { lend: 5.2, borrow: 6.8 },
    USDC: { lend: 8.1, borrow: 9.5 },
  };

  // Auth state
  let contractId: string = "";
  let walletPublicKey: string = "";
  let isLoggedIn = false;

  if (localStorage.hasOwnProperty("sp:keyId")) {
    connect(localStorage.getItem("sp:keyId"));
  }

  async function register() {
    const user = prompt("Give this passkey a name");
    if (!user) return;
    try {
      const {
        keyId: kid,
        contractId: cid,
        signedTx,
        publicKey,
      } = await account.createWallet("Borrowhood", user);
      const res = await server.send(signedTx);
      contractId = cid;
      walletPublicKey = publicKey;
      localStorage.setItem("sp:keyId", base64url(kid));
      isLoggedIn = true;
    } catch (err: any) {
      alert(err.message);
    }
  }
  async function connect(keyId_?: string) {
    try {
      const {
        keyId: kid,
        contractId: cid,
        publicKey,
      } = await account.connectWallet({
        keyId: keyId_,
        getContractId: (keyId) => server.getContractId({ keyId }),
      });
      contractId = cid;
      walletPublicKey = publicKey;
      localStorage.setItem("sp:keyId", base64url(kid));
      isLoggedIn = true;
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

  $: selectedToken, fetchPrice();
  onMount(fetchPrice);

  $: amountUsd = amount && !isNaN(+amount) ? (+amount * price).toFixed(2) : "";
  $: showMax = lendBorrowMode === "LEND";
  $: showBalance = lendBorrowMode === "LEND";
  $: maxAmount = balance[selectedToken];
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
</script>

<div class="navbar">
  <div class="logo" style="font-family: 'New Rocker', cursive;">BORROWHOOD</div>
  <nav>
    {#each tabs as tab}
      <div
        class="nav-tab {activeTab === tab ? 'active' : ''}"
        on:click={() => (activeTab = tab)}
      >
        {tab}
        <span class="underline {activeTab === tab ? 'active' : ''}"></span>
      </div>
    {/each}
  </nav>
  <div class="right-group">
    <button class="login-btn" on:click={openAuthModal}>Sign Up/Login</button>
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
        <button class="hero-btn primary">Start Borrowing</button>
        <button class="hero-btn secondary">Lend & Earn</button>
      </div>
    </section>

    <!-- Why Borrowhood Section -->
    <section class="why-section">
      <h2 class="section-title">
        <span class="gradient-text">Why</span> Borrowhood?
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
      <h2 class="cta-title">Ready to Experience the Future of Finance?</h2>
      <p class="cta-desc">
        Join thousands of users in third-world countries who are already
        leveraging Borrowhood for financial freedom.
      </p>
      <div class="cta-actions">
        <button
          class="cta-btn primary"
          on:click={() => (window.location.href = "/lendborrow")}
          >Start Lending</button
        >
        <button
          class="cta-btn secondary"
          on:click={() => (window.location.href = "/dashboard")}
          >View Dashboard</button
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
        </div>
      {/if}
      {#if dashboardTab === "Borrowing"}
        <div class="portfolio-table-wrap empty-borrowing">
          <div class="empty-msg">No active borrowings yet.</div>
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
            <div class="token-select">
              <select bind:value={selectedToken} class="token-dropdown">
                <option value="XLM">XLM</option>
                <option value="USDC">USDC</option>
              </select>
              <div class="token-info">
                <span class="token-apy">{rates[selectedToken].lend}% APY</span>
                <span class="token-price"
                  >{isLoadingPrice ? "..." : price.toFixed(2)} USD</span
                >
              </div>
            </div>
          </div>
          <div class="form-row">
            <div class="form-label-row">
              <span class="form-title"
                >Amount{amountUsd && ` (${amountUsd} USD)`}</span
              >
              {#if showBalance}
                <span class="form-balance"
                  >Balance: {balance[selectedToken]}
                  {selectedToken}
                  <span class="form-balance-usd"
                    >({(balance[selectedToken] * price).toFixed(2)} USD)</span
                  ></span
                >
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
                on:click={() => (duration = "custom")}>Custom</button
              >
              {#if duration === "custom"}
                <input
                  class="custom-months-input"
                  type="number"
                  min="1"
                  max="12"
                  bind:value={customMonths}
                  placeholder="6"
                />
                <span class="custom-months-label">month (max 12)</span>
              {/if}
            </div>
          </div>
          <button class="lend-btn" disabled={!isValid}
            >Lend {selectedToken}</button
          >
        </div>
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
    </section>
  {/if}
</main>

{#if showAuthModal}
  <div
    class="modal-overlay"
    on:click={closeAuthModal}
    tabindex="-1"
    on:keydown={handleKeydown}
  >
    <div class="auth-modal" on:click|stopPropagation>
      <h2 class="auth-title">Login/Sign Up</h2>
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
          <button class="auth-btn primary" on:click={connect}
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
  <div class="modal-overlay" on:click={closeLoanModal} tabindex="-1">
    <div class="loan-modal" on:click|stopPropagation>
      <div class="loan-modal-header">
        <span class="loan-modal-title">Withdraw {modalAsset.name}</span>
        <button class="loan-modal-close" on:click={closeLoanModal}
          >&times;</button
        >
      </div>
      <div class="loan-modal-asset">
        <span class="loan-modal-asset-icon">{modalAsset.icon}</span>
        <div class="loan-modal-asset-info">
          <div class="loan-modal-asset-name">{modalAsset.name}</div>
          <div class="loan-modal-asset-type">Lending Position</div>
        </div>
      </div>
      <div class="loan-modal-divider"></div>
      <div class="loan-modal-row">
        <span>Amount Deposited:</span><span class="loan-modal-value"
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
      <div class="loan-modal-row">
        <span>APY:</span><span class="loan-modal-value">{modalAsset.apy}</span>
      </div>
      <div class="loan-modal-row">
        <span>Interest Earned:</span><span
          class="loan-modal-value loan-modal-green">{modalAsset.interest}</span
        >
      </div>
      <div class="loan-modal-divider"></div>
      <div class="loan-modal-row loan-modal-total">
        <span>Total Withdrawable:</span><span class="loan-modal-value"
          >{modalAsset.withdrawable}</span
        >
      </div>
    </div>
  </div>
{/if}

<style>
  @import url("https://fonts.googleapis.com/css2?family=Jaro:opsz@6..72&family=New+Rocker&family=Outfit:wght@100..900&display=swap");
  :global(body) {
    margin: 0;
    font-family: "Outfit", sans-serif;
    background: linear-gradient(90deg, #1a1333 0%, #231942 100%);
    color: #fff;
  }
  .navbar {
    width: 100%;
    max-width: 100vw;
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    padding: 0 2.5rem;
    height: 54px;
    background: linear-gradient(90deg, #1a1333 0%, #231942 100%);
    box-shadow: 0 2px 16px 0 rgba(30, 0, 60, 0.12);
    position: relative;
    z-index: 10;
    overflow: hidden;
    min-width: 0;
    margin: 0 auto 1.5rem auto;
  }
  .logo {
    font-size: 2rem;
    color: #8ecaff;
    letter-spacing: 2px;
    text-shadow:
      0 0 8px #6f7cff,
      0 0 16px #6f7cff44;
    user-select: none;
    margin-right: 2rem;
    font-family: "New Rocker", cursive;
    white-space: nowrap;
    min-width: 0;
  }
  nav {
    display: flex;
    gap: 2rem;
    align-items: center;
    flex-shrink: 1;
    justify-content: flex-start;
    min-width: 0;
    overflow: hidden;
    margin-left: 0.5rem;
  }
  .nav-tab {
    position: relative;
    font-size: 1.1rem;
    font-weight: 700;
    cursor: pointer;
    color: #fff;
    padding: 0.5rem 0;
    transition: color 0.2s;
    font-family: "Outfit", sans-serif;
    white-space: nowrap;
    min-width: 0;
  }
  .nav-tab:hover,
  .nav-tab.active {
    color: #e0aaff;
  }
  .underline {
    display: block;
    position: absolute;
    left: 0;
    right: 0;
    bottom: -2px;
    height: 3px;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    border-radius: 2px;
    opacity: 0;
    transform: scaleX(0.5);
    transition:
      opacity 0.2s,
      transform 0.2s;
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
    font-weight: 700;
    font-size: 1.1rem;
    padding: 0.7rem 1.2rem;
    border: none;
    border-radius: 14px;
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    cursor: pointer;
    box-shadow: 0 2px 12px 0 #0003;
    transition:
      transform 0.13s,
      box-shadow 0.13s,
      background 0.2s;
    outline: none;
    white-space: nowrap;
    min-width: 0;
  }
  .login-btn:hover {
    transform: translateY(-2px) scale(1.04);
    box-shadow: 0 4px 20px 0 #a259ff55;
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
  }
  main {
    font-family: "Outfit", sans-serif;
    min-height: 80vh;
    padding: 2rem;
  }
  @media (max-width: 900px) {
    .navbar {
      padding: 0 1rem;
      height: 48px;
      margin: 0 auto 1rem auto;
    }
    nav {
      gap: 1rem;
      margin-left: 0.2rem;
    }
    .logo {
      font-size: 1.3rem;
      margin-right: 1rem;
    }
    .login-btn {
      font-size: 1rem;
      padding: 0.5rem 0.8rem;
    }
  }
  @media (max-width: 600px) {
    .navbar {
      flex-direction: column;
      height: 48px;
      align-items: flex-start;
      gap: 0.5rem;
      padding: 0 0.5rem;
    }
    nav {
      width: 100%;
      justify-content: flex-start;
      gap: 0.5rem;
      margin-left: 0;
    }
    .right-group {
      align-self: flex-end;
      margin-left: 0;
    }
  }
  .hero {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: calc(100vh - 54px - 1.5rem); /* 54px navbar + margin */
    width: 100%;
    text-align: center;
    margin: 0;
    padding: 0 1rem;
    box-sizing: border-box;
  }
  .hero-title {
    font-size: 3rem;
    font-weight: 700;
    margin-bottom: 1.2rem;
    line-height: 1.1;
    letter-spacing: -1px;
  }
  .gradient-text {
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
    display: inline-block;
  }
  .hero-title-bold {
    font-size: 3.2rem;
    font-weight: 900;
    color: #fff;
    margin-top: 0.2rem;
    display: inline-block;
  }
  .hero-subtitle {
    font-size: 1.3rem;
    color: #d1cbe7;
    margin-bottom: 2.2rem;
    margin-top: 0.5rem;
    font-weight: 400;
    line-height: 1.5;
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
    box-shadow: 0 2px 16px 0 #0002;
    margin-bottom: 0.5rem;
  }
  .hero-btn.primary {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
  }
  .hero-btn.primary:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    transform: translateY(-2px) scale(1.04);
    box-shadow: 0 4px 20px 0 #a259ff55;
  }
  .hero-btn.secondary {
    background: rgba(255, 255, 255, 0.08);
    color: #fff;
    border: 1.5px solid #a259ff;
  }
  .hero-btn.secondary:hover {
    background: rgba(162, 89, 255, 0.18);
    color: #fff;
    border-color: #38b6ff;
    transform: translateY(-2px) scale(1.04);
    box-shadow: 0 4px 20px 0 #38b6ff55;
  }
  @media (max-width: 700px) {
    .hero-title,
    .hero-title-bold {
      font-size: 2.1rem;
    }
    .hero {
      padding: 1.2rem 0.2rem 2rem 0.2rem;
      min-height: calc(100vh - 48px - 1rem);
    }
    .hero-actions {
      gap: 0.7rem;
    }
    .hero-btn {
      padding: 0.7rem 1.3rem;
      font-size: 1rem;
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
  .why-section,
  .how-section {
    margin: 3.5rem auto 0 auto;
    max-width: 1100px;
    padding: 0 1.2rem;
  }
  .section-title {
    font-size: 2.2rem;
    font-weight: 800;
    text-align: center;
    margin-bottom: 2.2rem;
    color: #fff;
    letter-spacing: -1px;
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
    background: rgba(30, 20, 60, 0.55);
    border-radius: 1.2rem;
    padding: 2.1rem 1.3rem 1.7rem 1.3rem;
    box-shadow: 0 2px 16px 0 #0002;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    color: #fff;
    min-height: 210px;
    transition:
      box-shadow 0.18s,
      background 0.18s;
  }
  .why-item:hover,
  .how-item:hover {
    box-shadow: 0 4px 32px 0 #a259ff33;
    background: linear-gradient(90deg, #a259ff33 0%, #38b6ff33 100%);
  }
  .why-icon,
  .how-icon {
    font-size: 2.2rem;
    margin-bottom: 1.1rem;
    display: block;
  }
  .why-item h3,
  .how-item h3 {
    font-size: 1.18rem;
    font-weight: 700;
    margin-bottom: 0.7rem;
    background: linear-gradient(90deg, #c471f5 0%, #38b6ff 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
  .why-item p,
  .how-item p {
    font-size: 1.05rem;
    color: #d1cbe7;
    font-weight: 400;
    margin: 0;
  }
  .cta-section {
    margin: 4.5rem auto 0 auto;
    max-width: 700px;
    padding: 0 1.2rem 4rem 1.2rem;
    text-align: center;
  }
  .cta-title {
    font-size: 2rem;
    font-weight: 800;
    color: #c471f5;
    margin-bottom: 1.1rem;
  }
  .cta-desc {
    font-size: 1.15rem;
    color: #d1cbe7;
    margin-bottom: 2.2rem;
  }
  .cta-actions {
    display: flex;
    gap: 1.5rem;
    justify-content: center;
    flex-wrap: wrap;
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
    box-shadow: 0 2px 16px 0 #0002;
    margin-bottom: 0.5rem;
  }
  .cta-btn.primary {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
  }
  .cta-btn.primary:hover {
    background: linear-gradient(90deg, #38b6ff 0%, #a259ff 100%);
    transform: translateY(-2px) scale(1.04);
    box-shadow: 0 4px 20px 0 #a259ff55;
  }
  .cta-btn.secondary {
    background: rgba(255, 255, 255, 0.08);
    color: #fff;
    border: 1.5px solid #a259ff;
  }
  .cta-btn.secondary:hover {
    background: rgba(162, 89, 255, 0.18);
    color: #fff;
    border-color: #38b6ff;
    transform: translateY(-2px) scale(1.04);
    box-shadow: 0 4px 20px 0 #38b6ff55;
  }
  @media (max-width: 700px) {
    .why-grid {
      grid-template-columns: 1fr;
      grid-template-rows: repeat(6, 1fr);
      gap: 1.2rem;
    }
    .how-grid {
      grid-template-columns: 1fr;
      grid-template-rows: repeat(4, 1fr);
      gap: 1.2rem;
    }
    .cta-section {
      padding: 0 0.2rem 2rem 0.2rem;
    }
    .cta-btn {
      padding: 0.7rem 1.3rem;
      font-size: 1rem;
    }
  }
  .portfolio-card {
    background: rgba(20, 15, 40, 0.93);
    border-radius: 1.3rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.5rem 2.2rem 2.2rem 2.2rem;
    max-width: 1100px;
    margin: 0 auto;
    color: #fff;
    margin-bottom: 2.5rem;
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
    background: #232032;
    border-radius: 2.5rem;
    box-shadow: 0 2px 16px 0 #0002;
    display: flex;
    align-items: center;
    padding: 0.3rem 0.3rem;
    gap: 0.2rem;
    min-width: 320px;
  }
  .toggle-btn {
    font-family: "Outfit", sans-serif;
    font-size: 1.2rem;
    font-weight: 700;
    border: none;
    border-radius: 2rem;
    background: none;
    color: #bdb8d7;
    padding: 0.7rem 2.5rem;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s;
  }
  .toggle-btn.active {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .lendborrow-main {
    display: flex;
    gap: 2.5rem;
    align-items: flex-start;
    flex-wrap: wrap;
  }
  .lend-form-card {
    background: #232032;
    border-radius: 1.2rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.2rem 2.2rem 1.5rem 2.2rem;
    min-width: 370px;
    max-width: 480px;
    flex: 1 1 370px;
    color: #fff;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
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
  }
  .duration-btn {
    background: none;
    border: 1.5px solid #28243a;
    color: #bdb8d7;
    font-size: 1.1rem;
    font-weight: 700;
    border-radius: 0.8rem;
    padding: 0.9rem 2.2rem;
    cursor: pointer;
    transition:
      background 0.18s,
      color 0.18s,
      border 0.18s;
  }
  .duration-btn.active {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    border: 1.5px solid #a259ff;
    box-shadow: 0 2px 12px 0 #a259ff33;
  }
  .custom-months-input {
    width: 3.5rem;
    background: #18122b;
    border: 1.5px solid #28243a;
    border-radius: 0.7rem;
    color: #fff;
    font-size: 1.1rem;
    font-weight: 600;
    padding: 0.7rem 0.7rem;
    outline: none;
    margin-left: 0.7rem;
    margin-right: 0.3rem;
  }
  .custom-months-label {
    color: #bdb8d7;
    font-size: 1.05rem;
    font-weight: 500;
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
  .rates-card {
    background: #232032;
    border-radius: 1.2rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.2rem 2.2rem 1.5rem 2.2rem;
    min-width: 370px;
    max-width: 480px;
    flex: 1 1 370px;
    color: #fff;
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
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
  @media (max-width: 900px) {
    .lendborrow-main {
      flex-direction: column;
      gap: 2.2rem;
    }
    .lend-form-card,
    .rates-card {
      min-width: 0;
      max-width: 100%;
    }
  }
</style>
