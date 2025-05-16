<script lang="ts">
  // No additional logic needed for static navbar
  let activeTab = "Home";
  const tabs = ["Home", "Dashboard", "Lend/Borrow", "Wallet"];
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
    <button class="login-btn">Sign Up/Login</button>
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
  {/if}
  <!-- Main content placeholder for other tabs -->
</main>

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
</style>
