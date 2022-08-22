<script>
  import Signature from "$lib/signature/Signature.svelte";
</script>

<section class="eyecatcher">
  <Signature />
  <img src="me-light.png">
</section>

<style>
  .eyecatcher {
    background: var(--c-off-white);
    color: var(--c-black) !important;
    /*height: calc(100vh - 8rem);*/
    flex-grow: 1;
    margin: 2rem;
    /*padding: 2rem;*/
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
