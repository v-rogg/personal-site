<script lang="ts">
  import Signature from "$lib/Signature.svelte";
  import SignatureNumber from "$lib/SignatureNumber.svelte";
  import { dark_mode } from "$lib/../stores";
</script>

<section class="eyecatcher">
  <Signature />
  <SignatureNumber />
  <img src="{$dark_mode ? 'me-dark.png' : 'me-light.png'}" alt="Handsome Portrait Valentin Rogg" />
</section>

<style>
  .eyecatcher {
    background: var(--c-secondary);
    color: var(--c-black) !important;
    flex-grow: 1;
    margin: 2rem;
    border-radius: 4px;
    position: relative;
    min-height: 670px;
    max-height: 760px;
  }

  img {
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translate(-50%);
  }
</style>
