<script lang="ts">
  import EyeCatcher from "$lib/EyeCatcher.svelte";
  import ShortBio from "$lib/ShortBio.svelte";
  import { currentSignatureStore, signatureRefsStore } from "$lib/../stores";

  export let data: {lang: string, route: string, slug: string, signatureRefs: [], currentSignature};

  if (data) {
    currentSignatureStore.set(data.currentSignature)
    signatureRefsStore.set(data.signatureRefs)
  }
</script>

<svelte:head>
  <title>Valentin Rogg</title>
</svelte:head>

<section class="first">
  <EyeCatcher />
  <ShortBio />
</section>

<style>
  .first {
    /*height: 90vh;*/
    /*max-height: calc(100vh - 3rem);*/
    display: flex;
    flex-direction: column;
  }
</style>
