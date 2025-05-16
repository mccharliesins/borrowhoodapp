<script lang="ts">
  // No additional logic needed for static navbar
  let activeTab = "Home";
  const tabs = ["Home", "Dashboard", "Lend/Borrow", "Wallet"];

  let showAuthModal = false;

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
        <button class="tab active">Lending</button>
        <button class="tab">Borrowing</button>
      </div>
      <div class="portfolio-table-wrap">
        <table class="portfolio-table">
          <thead>
            <tr>
              <th>Asset</th>
              <th>Amount</th>
              <th>Value (USD)</th>
              <th>APY</th>
              <th>Days Left</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td><span class="asset-icon">🌟</span> XLM</td>
              <td>1,200</td>
              <td>$480.50</td>
              <td class="apy positive">5.2%</td>
              <td>0 days left</td>
              <td><button class="details-btn">View Details</button></td>
            </tr>
            <tr>
              <td><span class="asset-icon">💲</span> USDC</td>
              <td>750</td>
              <td>$750.00</td>
              <td class="apy positive">8.1%</td>
              <td>0 days left</td>
              <td><button class="details-btn">View Details</button></td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  {/if}
  <!-- Main content placeholder for other tabs -->
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
        <button class="auth-btn primary">Login with Passkey</button>
        <button class="auth-btn secondary">Create a New Account</button>
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
</style>
