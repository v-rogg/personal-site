<script lang="ts">
  import Header from "$lib/header/Header.svelte";
  import "../app.css";
  import Footer from "../lib/footer/Footer.svelte";
  import { darkmode } from "../stores.ts";
  import CenterLayout from "../lib/SpecialLayouts/CenterLayout.svelte";
</script>

<div class="background" class:light={!$darkmode} class:dark={$darkmode}>
  <CenterLayout>
    <Header slot="top_mid" />
    <slot slot="center" />
    <Footer slot="bottom_mid" />
  </CenterLayout>
</div>

<style>
  .background {
    background: var(--c-white);
    color: var(--c-black) !important;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  :global(a) {
    text-decoration: none;
    color: var(--c-black);
  }

  div {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
</style>
