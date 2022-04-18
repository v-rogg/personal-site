<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { dark_mode } from "../../stores";
  import { t, locale, locales } from "../i18n";

  function toggle_lang() {
    let i = $locales.findIndex((e) => e === $locale) + 1;
    if (i >= $locales.length) i = 0;
    goto(`/${$locales[i]}${$page.stuff.route}`);
  }
</script>

<header class="container">
  <h1 class="">
    <a href="/"> Valentin Rogg </a>
  </h1>

  <nav>
    <ul>
      <li class:active={$page.url.pathname === "/"}>
        <a sveltekit:prefetch href="/">Home</a>
      </li>
      <li class:active={$page.url.pathname === "/blog"}>
        <a sveltekit:prefetch href="/" aria-disabled="true" class="strikethrough disabled">{$t("common.blog")}</a>
        <span class="coming_soon">{$t("common.coming_soon")}</span>
        <!--        <span class="new"><i class="fa-solid fa-circle-small"></i></span>-->
      </li>
      <li class:active={$page.url.pathname === "/projects"}>
        <a sveltekit:prefetch href="/" aria-disabled="true" class="strikethrough disabled">{$t("common.projects")}</a>
        <span class="coming_soon">{$t("common.coming_soon")}</span>
      </li>
    </ul>
  </nav>

  <div class="options">
    <button
      on:click={() => {
        $dark_mode = !$dark_mode;
      }}
      class="dark_switch"><i class="fak fa-moon" /></button
    >
    <button
      on:click={() => {
        toggle_lang();
      }}
      class="language_switch">{$t(`lang.${$locale}`)}</button
    >
  </div>
</header>

<style>
  header {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    justify-content: space-between;
    align-items: center;
    font-size: 16px;
  }

  .options {
    justify-self: flex-end;
  }

  nav {
    justify-self: center;
  }

  nav ul {
    margin: 0;
    padding: 0;
    display: flex;
    gap: 2rem;
  }

  nav ul li {
    list-style: none;
    display: flex;
    flex-direction: column;
    position: relative;
  }

  .coming_soon {
    font-size: 10px;
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    width: 100px;
    text-align: center;
    color: var(--c-orange);
  }

  /*.new {*/
  /*  font-size: 8px;*/
  /*  position: absolute;*/
  /*  bottom: 70%;*/
  /*  right: 0;*/
  /*  transform: translateX(8px);*/
  /*  color: var(--c-green);*/
  /*}*/

  .options {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .dark_switch {
    border: none;
    background: none;
    margin: 0;
    padding: 0;
    font-size: 16px;
    color: var(--c-black);
  }

  .dark_switch:hover {
    cursor: pointer;
  }

  .language_switch {
    width: 50px;
    height: 24px;
    border-radius: 4px;
    border: none;
    background: var(--c-grey-10);
  }

  .language_switch:hover {
    background: var(--c-grey-30);
    cursor: pointer;
  }

  .strikethrough {
    text-decoration: line-through var(--c-orange);
  }

  h1 {
    font-family: "Obacht Display", sans-serif;
    font-size: 18px;
  }
</style>
