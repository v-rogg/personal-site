<script lang="ts">
  import { currentSignatureStore } from "$lib/../stores";
  import { locale } from "$lib/_i18n";

  // let currentSignature;
  // currentSignatureStore.subscribe(d => currentSignature = d);
  // const signatureRefs = getContext("signatureRefs");

</script>

<div class="container">
  <div class="signature-number">
    {#if Object.keys($currentSignatureStore).length > 0}
      <div>{$currentSignatureStore.data.name ? $currentSignatureStore.data.name : ""}</div>
<!--      <div id="index"><i class="fa-regular fa-hashtag" style="font-size: 15px"></i> {$currentSignatureStore.ref['@ref'].id} <span class="small">/ {$signatureRefsStore.length}</span></div>-->
      <div id="date" class="small">{$currentSignatureStore.ts ? (new Date($currentSignatureStore.ts / 1000)).toLocaleDateString($locale) : ""}</div>
    {/if}
  </div>
</div>

<style>
  .signature-number {
    position: absolute;
    bottom: 2rem;
    z-index: 800;
  }

  #date, #index {
  }

  .small {
    font-size: 12px;
  }
</style>
