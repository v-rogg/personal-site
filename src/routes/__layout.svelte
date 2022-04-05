<script lang="ts">
  import Header from '$lib/header/Header.svelte';
  import '../app.css';
  import Footer from '../lib/footer/Footer.svelte';
</script>

<div class="dark">
  <Header />

  <main>
    <slot />
  </main>

  <Footer />
</div>

<style>
  :global(a) {
    text-decoration: none;
    color: var(--black);
  }

  div {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
</style>
