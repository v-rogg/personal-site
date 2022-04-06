<script lang="ts">
  import { page } from "$app/stores";
  import { darkmode } from "../../stores";

  let lang = 0;
  const languages = ["EN", "DE"];

  function toggle_lang() {
    if (lang + 1 < languages.length) {
      lang++;
    } else {
      lang = 0;
    }
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
        <a sveltekit:prefetch href="/" aria-disabled="true" class="strikethrough">Blog</a>
        <span class="coming_soon">Coming soon</span>
        <!--        <span class="new"><i class="fa-solid fa-circle-small"></i></span>-->
      </li>
      <li class:active={$page.url.pathname === "/projects"}>
        <a sveltekit:prefetch href="/" aria-disabled="true" class="strikethrough">Projects</a>
        <span class="coming_soon">Coming soon</span>
      </li>
    </ul>
  </nav>

  <div class="options">
    <button
      on:click={() => {
        $darkmode = !$darkmode;
      }}
      class="dark_switch"><i class="fak fa-moon" /></button
    >
    <button
      on:click={() => {
        toggle_lang();
      }}
      class="language_switch">{languages[lang]}</button
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
    text-decoration: line-through var(--c-black);
  }

  h1 {
    font-family: "Pulp Display", sans-serif;
    font-size: 18px;
  }
</style>
