<script lang="ts">
  import { currentSignatureStore } from "./stores";
  import { locale } from "$lib/_i18n";
</script>

<div class="container">
  <div class="signature-number">
    {#if $currentSignatureStore}
      {#if Object.keys($currentSignatureStore).length > 0}
        <div>{$currentSignatureStore.data.name ? $currentSignatureStore.data.name : ""}</div>
        <div id="date" class="small">{$currentSignatureStore.ts ? (new Date($currentSignatureStore.ts / 1000)).toLocaleDateString($locale) : ""}</div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .signature-number {
    position: absolute;
    bottom: 2rem;
    z-index: 800;
  }

  .small {
    font-size: 12px;
  }
</style>
