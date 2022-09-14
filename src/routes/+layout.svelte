<script lang="ts">
  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";
  import { dark_mode } from "$lib/../stores";
  import { t } from "$lib/_i18n";
  import { admin, currentSignatureStore, signatureRefsStore } from "$lib/../stores";

  // export let data: {lang: string, route: string, slug: string, signatureRefs: [], currentSignature};

  export let data: {signatureRefs: [], currentSignature};

  if (data) {
    currentSignatureStore.set(data.currentSignature)
    signatureRefsStore.set(data.signatureRefs)
  }
</script>

<svelte:head>
  <meta name="description" content="{$t('common.meta')}" />
</svelte:head>

<div class="app" class:light={!$dark_mode} class:dark={$dark_mode}>
  <Header />
  <slot />
  <Footer />
  {$admin}
</div>

<style>
  .app {
    color: var(--c-black);
    background: var(--c-bg);
    height: 100%;
    min-height: 100vh;
    position: absolute;
    top: 0;
    width: 100%;
    padding-bottom: 200px;
  }
</style>
