<script lang="ts">
  import Header from "$lib/header/Header.svelte";
  import Footer from "$lib/footer/Footer.svelte";
  import { dark_mode } from "$lib/../stores.ts";
</script>

<div class="background" class:light={!$dark_mode} class:dark={$dark_mode}>
  <div class="container flex">
    <Header />
    <slot />
    <Footer />
  </div>
</div>

<style>
  .background {
    background: var(--c-off-white);
    color: var(--c-black) !important;
    height: 100%;
    margin: 2rem;
    padding: 2rem;
    border-radius: 4px;
  }

  .flex {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
</style>
