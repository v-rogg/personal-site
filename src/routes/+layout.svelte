<script lang="ts">
  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";
  import { dark_mode } from "$lib/../stores.ts";
  import "../app.css";
</script>

<!--<div class="background" class:light={!$dark_mode} class:dark={$dark_mode}>-->
<!--  <div class="container flex">-->
<!--&lt;!&ndash;    <Footer />&ndash;&gt;-->
<!--  </div>-->
<!--</div>-->
<div class="app" class:light={!$dark_mode} class:dark={$dark_mode}>
  <Header />
  <slot />
  <Footer />
</div>

<!--<div id="centerline" />-->
<style>
  #centerline {
    width: 2px;
    height: 100vh;
    background: red;
    position: absolute;
    left: 50%;
    transform: translate(-50%);
    top: 0;
  }

  .background {
    background: var(--c-off-white);
    color: var(--c-black) !important;
    height: 100%;
    margin: 2rem;
    padding: 2rem;
    border-radius: 4px;
  }

  .flex {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .app {
    background: var(--c-bg);
    height: 100%;
    min-height: 100vh;
  }
</style>
