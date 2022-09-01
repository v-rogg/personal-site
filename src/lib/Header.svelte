<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { dark_mode, slugStore } from "$lib/../stores";
  import { l, t, locale, locales } from "$lib/_i18n";

  function toggle_lang() {
    let i = $locales.findIndex((e) => e === $locale) + 1;
    if (i >= $locales.length) i = 0;
    goto(`/${$locales[i]}${$l($locales[i], $slugStore)}`);
  }

  function toggle_dark_mode() {
    $dark_mode = !$dark_mode;
    document.cookie = `darkModeEnabled=${$dark_mode};expires=Thu, 18 Dec 2300 12:00:00 UTC; path=/; SameSite=Strict`
    // localStorage.setItem("darkModeEnabled", $dark_mode)
  }
</script>

<header class="container">
  <h1 class="overlay">
    <a href="/{$locale}"> Valentin Rogg </a>
  </h1>


  <nav class="overlay">
    <ul>


      <li class:active={$page.url.pathname === "/posts"}>
        <a href="/{$locale}" aria-disabled="true" class="strikethrough disabled"
          >{$t("common.posts")}</a
        >
        <span class="coming_soon">{$t("common.coming_soon")}</span>
      </li>
      <li class:active={$page.url.pathname === "/projects"}>
        <a href="/{$locale}" aria-disabled="true" class="strikethrough disabled"
          >{$t("common.projects")}</a
        >
        <span class="coming_soon">{$t("common.coming_soon")}</span>
      </li>
    </ul>
  </nav>

  <div class="options overlay">
    <button on:click={() => toggle_dark_mode()} class="dark_switch">
      <span style="display: {$dark_mode ? 'none' : 'initial'}">
        <i class="fa-solid fa-moon"></i>
      </span>
      <span style="display: {!$dark_mode ? 'none' : 'initial'}">
        <i class="fa-solid fa-sun"></i>
      </span>
    </button>
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
    /*padding: 10px 2rem 60px;*/
    width: 100vw;
    position: fixed;
    /*padding: 2rem;*/
    /*margin: 2rem;*/
    left: 50%;
    transform: translate(-50%);
    top: 4rem;
    z-index: 900;
  }

  @media (max-width: 575.98px) {
    header {
      grid-template-columns: 1fr 1fr;
      grid-template-rows: 1fr auto;
    }

    nav {
      grid-area: 2/1/3/4;
    }

    nav ul {
      flex-direction: column;
      align-items: center;
      font-size: 2.5rem;
      margin-top: 60px !important;
      margin-bottom: 1em !important;
      gap: 2em !important;
    }
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
    gap: 4em;
  }

  nav ul li {
    list-style: none;
    display: flex;
    position: relative;
  }

  .coming_soon {
    font-size: 0.625em;
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    width: max-content;
    text-align: center;
    color: var(--c-primary);
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
    background: var(--c-white);
  }

  .language_switch:hover {
    background: var(--c-grey-10);
    cursor: pointer;
  }

  .language_switch:active {
    background: var(--c-grey-30);
    cursor: default;
  }

  .strikethrough:after {
    content: "";
    position: absolute;
    width: calc(100% + 16px);
    height: 2px;
    left: -8px;
    bottom: 40%;
    background: var(--c-primary);
  }

  h1 {
    font-family: "Display", sans-serif;
    font-size: 18px;
    font-weight: 700;
    white-space: nowrap;
    padding: 0;
    margin: 0;
  }
</style>
