<script lang="ts">
  let activeTab = "Dashboard";
  const tabs = ["Home", "Dashboard", "Lend/Borrow", "Wallet"];
  function goTo(tab: string) {
    window.location.href =
      tab === "Home" ? "/" : `/${tab.replace(/\s+/g, "").toLowerCase()}`;
  }
</script>

<div class="navbar">
  <div class="logo" style="font-family: 'New Rocker', cursive;">BORROWHOOD</div>
  <nav>
    {#each tabs as tab}
      <div
        class="nav-tab {activeTab === tab ? 'active' : ''}"
        on:click={() => goTo(tab)}
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
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Jaro:opsz@6..72&family=New+Rocker&family=Outfit:wght@100..900&display=swap");
  /* Copy navbar and layout styles from App.svelte for consistency */
  main {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2.5rem 1.2rem 3rem 1.2rem;
  }
  .dashboard-title {
    font-size: 2.7rem;
    font-weight: 800;
    color: #fff;
    margin-bottom: 2.5rem;
    margin-left: 1.5rem;
    letter-spacing: -1px;
  }
  .portfolio-card {
    background: rgba(20, 15, 40, 0.93);
    border-radius: 1.3rem;
    box-shadow: 0 8px 48px 0 #0008;
    padding: 2.5rem 2.2rem 2.2rem 2.2rem;
    max-width: 1500px;
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
    color: #fff;
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
    color: #a259ff;
    border-image: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    border-image-slice: 1;
    border-bottom: 3px solid;
  }
  .portfolio-table-wrap {
    overflow-x: auto;
  }
  .portfolio-table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 1.2rem;
    background: none;
  }
  .portfolio-table th,
  .portfolio-table td {
    padding: 1.1rem 1.2rem;
    text-align: left;
    font-size: 1.13rem;
    font-weight: 500;
    color: #fff;
  }
  .portfolio-table th {
    color: #bdb8d7;
    font-size: 1.08rem;
    font-weight: 700;
    border-bottom: 2px solid #232042;
  }
  .portfolio-table tr:not(:last-child) {
    border-bottom: 1px solid #232042;
  }
  .asset-icon {
    font-size: 1.3rem;
    margin-right: 0.6rem;
    vertical-align: middle;
  }
  .apy.positive {
    color: #3ee86b;
    font-weight: 700;
  }
  .details-btn {
    background: linear-gradient(90deg, #a259ff 0%, #38b6ff 100%);
    color: #fff;
    font-weight: 700;
    border: none;
    border-radius: 0.8rem;
    padding: 0.7rem 2.1rem;
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
    .portfolio-table th,
    .portfolio-table td {
      padding: 0.7rem 0.5rem;
      font-size: 1rem;
    }
    .details-btn {
      padding: 0.5rem 1.1rem;
      font-size: 0.98rem;
    }
  }
</style>
