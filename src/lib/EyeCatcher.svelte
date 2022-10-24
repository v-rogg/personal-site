<script lang="ts">
  import Signature from "$lib/Signature.svelte";
  import SignatureNumber from "$lib/SignatureNumber.svelte";
  import { dark_mode, admin } from "$lib/../stores";
  import SignatureAdmin from "$lib/SignatureAdmin.svelte";
</script>

<section class="eyecatcher">
  <Signature />
  <SignatureNumber />
  <!--{#if $dark_mode}-->
    <img src="me-dark.png" alt="Handsome Portrait Valentin Rogg" style="display: {$dark_mode ? 'block' : 'none' }"/>
  <!--{:else}-->
    <img src="me-light.png" alt="Handsome Portrait Valentin Rogg" style="display: {$dark_mode ? 'none' : 'block' }"/>
  <!--{/if}-->
  {#if $admin}
    <SignatureAdmin/>
  {/if}
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
    overflow: hidden;
  }

  .eyecatcher:after {
    animation: grain 8s steps(10) infinite;
    background-image: url("/noise.png");
    content: "";
    height: 300%;
    left: -50%;
    position: absolute;
    top: -100%;
    width: 300%;
    display: block;
    mix-blend-mode: hard-light;
  }

  :global(.dark) .eyecatcher:after {
    opacity: .1;
    filter: brightness(1);
  }

  :global(.light) .eyecatcher:after {
    opacity: .05;
    filter: brightness(3);
  }

  @keyframes grain {
    0%, 100% { transform:translate(0, 0) }
    10% { transform:translate(-5%, -10%) }
    20% { transform:translate(-15%, 5%) }
    30% { transform:translate(7%, -25%) }
    40% { transform:translate(-5%, 25%) }
    50% { transform:translate(-15%, 10%) }
    60% { transform:translate(15%, 0%) }
    70% { transform:translate(5%, 15%) }
    80% { transform:translate(3%, 30%) }
    90% { transform:translate(-10%, 10%) }
  }

  img {
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translate(-50%);
  }
</style>
