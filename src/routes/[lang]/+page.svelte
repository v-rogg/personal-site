<script lang="ts">
  // import Signature from "$lib/signature/Signature.svelte";
  // import Footer from "$lib/footer/Footer.svelte";
  import EyeCatcher from "$lib/eyecatcher/EyeCatcher.svelte";
  import ShortBio from "../../lib/ShortBio.svelte";
  import { setContext } from "svelte";

  export let data: {lang: string, route: string, signatureRefs: [], currentSignature};

  setContext("signatureRefs", data.signatureRefs)
  setContext("currentSignature", data.currentSignature)
</script>

<svelte:head>
  <title>Valentin Rogg</title>
</svelte:head>

<section class="first">
  <EyeCatcher />
  <ShortBio />
</section>

<!--<section class="eye-catcher">-->
<!--  <Signature />-->
<!--  <Footer />-->

<!--</section>-->
<style>
  .first {
    height: 90vh;
    max-height: calc(100vh - 3rem);
    /*height: 100%;*/
    /*position: relative;*/
    display: flex;
    flex-direction: column;
  }
  /*.eye-catcher {*/
  /*  width: 100%;*/
  /*  height: 50vh;*/
  /*  max-height: 900px;*/
  /*  flex-grow: 1;*/
  /*}*/

  /*.sprinter {*/
  /*  height: 100%;*/
  /*  width: 100%;*/
  /*  display: flex;*/
  /*  justify-content: center;*/
  /*  align-items: center;*/
  /*  pointer-events: none;*/
  /*}*/

  /*.sprinter img {*/
  /*  display: block;*/
  /*  width: 80%;*/
  /*  max-width: 768px;*/
  /*  transform: scaleX(-1) rotateZ(1deg);*/
  /*}*/

  /*@media (max-width: 575.98px) {*/
  /*  .sprinter {*/
  /*    height: 100%;*/
  /*  }*/
  /*}*/
</style>
