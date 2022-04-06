<script context="module" lang="ts">
  // export const prerender = true;
</script>

<script lang="ts">
</script>

<svelte:head>
  <title>Valentin Rogg</title>
</svelte:head>

<section>
  <picture class="sprinter">
    <img src="sprinter.png" alt="Sprinter vanlife camper" class="container" />
  </picture>
</section>

<style>
  section {
    width: 100%;
    height: 100%;
  }

  .sprinter {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .sprinter img {
    display: block;
    width: 40%;
    transform: scaleX(-1) rotateZ(1deg);
  }
</style>
