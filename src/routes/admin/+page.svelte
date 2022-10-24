<script lang="ts">
  import EyeCatcher from "$lib/EyeCatcher.svelte";
  import { currentSignatureStore, signatureRefsStore } from "$lib/../stores";

  export let data: {lang: string, route: string, slug: string, signatureRefs: [], currentSignature};

  if (data) {
    currentSignatureStore.set(data.currentSignature)
    signatureRefsStore.set(data.signatureRefs)
  }
</script>

<section>
  <EyeCatcher />
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
  }
</style>

