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
    <img src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/02a117d6-3c80-4950-7cf8-e6713f71b300/100"
         srcset="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/02a117d6-3c80-4950-7cf8-e6713f71b300/25 342w,
                 https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/02a117d6-3c80-4950-7cf8-e6713f71b300/50 683w,
                 https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/02a117d6-3c80-4950-7cf8-e6713f71b300/75 1025w,
                 https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/02a117d6-3c80-4950-7cf8-e6713f71b300/100 1366w"
         sizes="100vw"
         alt="Sprinter vanlife camper"/>
  </picture>
</section>

<style>
  section {
    width: 100%;
    max-height: 100%;
    overflow: hidden;
    flex-grow: 1;
  }

  .sprinter {
    /*height: calc(100vh - 60px - 60px - 120px - 60px - 200px);*/
    height: 100%;
    /*max-height: 100%;*/
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
      height: 100%;
      /*display: none !important;*/
    }
  }
</style>
