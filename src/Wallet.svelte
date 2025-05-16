<script lang="ts">
  let activeTab = "Wallet";
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
  <h1>Wallet</h1>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Jaro:opsz@6..72&family=New+Rocker&family=Outfit:wght@100..900&display=swap");
  /* Copy navbar and layout styles from App.svelte for consistency */
</style>
