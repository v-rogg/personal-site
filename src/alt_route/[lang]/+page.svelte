<script lang="ts">
  import Signature from "$lib/signature/Signature.svelte";
</script>

<svelte:head>
  <title>Valentin Rogg</title>
</svelte:head>

<section>
  <Signature/>
</section>

<style>
  section {
    width: 100%;
    max-height: 100%;
    flex-grow: 1;
  }

  .sprinter {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: none;
  }

  .sprinter img {
    display: block;
    width: 80%;
    max-width: 768px;
    transform: scaleX(-1) rotateZ(1deg);
  }

  @media (max-width: 575.98px) {
    .sprinter {
      height: 100%;
    }
  }
</style>
