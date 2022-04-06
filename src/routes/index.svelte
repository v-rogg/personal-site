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
    <img src="sprinter.png" alt="Big picture displaying a sprinter vanlife camper" class="container" />
  </picture>
</section>

<style>
  .sprinter {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  @media (max-width: 1300px), (max-height: 650px) {
    .sprinter {
      max-height: 300px;
    }
  }

  .sprinter img {
    display: block;
    width: 40%;
  }
</style>
