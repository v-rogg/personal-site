<script lang="ts">
  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";
  import { dark_mode } from "$lib/../stores";
  import { t } from "$lib/_i18n";
</script>

<svelte:head>
  <meta name="description" content="{$t('common.meta')}" />
</svelte:head>

<div class="app" class:light={!$dark_mode} class:dark={$dark_mode}>
  <Header />
  <slot />
  <Footer />
</div>

<style>
  .app {
    color: var(--c-black);
    background: var(--c-bg);
    height: 100%;
    min-height: 100vh;
  }
</style>
