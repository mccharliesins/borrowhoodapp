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
    <div class="wallet-icon">🌞</div>
    <button class="login-btn">Sign Up/Login</button>
  </div>
</div>

<main>
  <!-- Main content placeholder -->
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
    align-items: center;
    justify-content: space-between;
    padding: 0 1.2rem;
    height: 70px;
    background: linear-gradient(90deg, #1a1333 0%, #231942 100%);
    box-shadow: 0 2px 16px 0 rgba(30, 0, 60, 0.12);
    position: relative;
    z-index: 10;
    overflow-x: auto;
  }
  .logo {
    font-size: 2rem;
    color: #8ecaff;
    letter-spacing: 2px;
    text-shadow:
      0 0 8px #6f7cff,
      0 0 16px #6f7cff44;
    user-select: none;
    margin-right: 1rem;
    font-family: "New Rocker", cursive;
    white-space: nowrap;
  }
  nav {
    display: flex;
    gap: 1.2rem;
    align-items: center;
    flex: 1;
    justify-content: flex-start;
    min-width: 0;
    overflow-x: auto;
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
    gap: 1.2rem;
    min-width: 0;
  }
  .wallet-icon {
    width: 36px;
    height: 36px;
    background: rgba(255, 255, 255, 0.04);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    box-shadow: 0 2px 8px 0 #0002;
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
      padding: 0 0.5rem;
    }
    nav {
      gap: 0.7rem;
    }
    .logo {
      font-size: 1.3rem;
      margin-right: 0.5rem;
    }
    .login-btn {
      font-size: 1rem;
      padding: 0.5rem 0.8rem;
    }
  }
  @media (max-width: 600px) {
    .navbar {
      flex-direction: column;
      height: auto;
      align-items: flex-start;
      gap: 0.5rem;
    }
    nav {
      width: 100%;
      justify-content: flex-start;
      gap: 0.5rem;
    }
    .right-group {
      align-self: flex-end;
    }
  }
</style>
