<script context="module" lang="ts">
  export const prerender = true;
</script>

<script lang="ts">
</script>

<svelte:head>
  <title>Valentin Rogg</title>
</svelte:head>

<section>
  <picture class="sprinter">
    <img src="sprinter.png" alt="Sprinter vanlife camper"/>
  </picture>
</section>

<style>
  section {
    width: 100%;
  }

  .sprinter {
    height: calc(100vh - 60px - 60px - 120px - 60px - 80px);
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .sprinter img {
    display: block;
    width: 80%;
    max-width: 768px;
    transform: scaleX(-1) rotateZ(1deg);
  }

  @media (max-width: 575.98px) {
    .sprinter {
      display: none !important;
    }
  }
</style>
