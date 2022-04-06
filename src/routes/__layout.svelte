<script context="module" lang="ts">
  import { loadTranslations } from "../lib/i18n";

  export const load = async ({ url }) => {
    const { pathname } = url;
    const lang = `${pathname.match(/[^/]+?(?=\/|$)/) || ""}`;
    const route = pathname.replace(new RegExp(`^/${lang}`), "");
    await loadTranslations(lang, route);
    return { stuff: { route, lang } };
  };
</script>

<script lang="ts">
  import Header from "$lib/header/Header.svelte";
  import "../app.css";
  import Footer from "$lib/footer/Footer.svelte";
  import { dark_mode } from "../stores.ts";
  import CenterLayout from "$lib/layouts/CenterLayout.svelte";
</script>

<div class="background" class:light={!$dark_mode} class:dark={$dark_mode}>
  <CenterLayout>
    <Header slot="top" />
    <slot slot="center" />
    <Footer slot="bottom" />
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

  div {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
</style>
