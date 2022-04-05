<script lang="ts">
  import { page } from '$app/stores';

  let lang = 0;
  const languages = ["EN", "DE"];

  let invert = false;

  function toggle_lang() {
    if (lang + 1 < languages.length) {
      lang++
    } else {
      lang = 0;
    }
  }
</script>

<header class="container">
  <h1 class="">
    <a href="/">
      Valentin Rogg
    </a>
  </h1>

  <nav>
    <ul>
      <li class:active={$page.url.pathname === '/'}>
        <a sveltekit:prefetch href="/">Home</a>
      </li>
      <li class:active={$page.url.pathname === '/blog'} class="strikethrough">
        <a sveltekit:prefetch href="/" aria-disabled="true">Blog</a>
      </li>
      <li class:active={$page.url.pathname === '/projects'} class="strikethrough">
        <a sveltekit:prefetch href="/" aria-disabled="true">Projects</a>
      </li>
    </ul>
  </nav>

  <div class="options">
    <button on:click={() => {invert = !invert}} class:invert={invert} class="dark_switch"><i class="fak fa-moon"></i></button>
    <button on:click={() => {toggle_lang()}} class="language_switch">{languages[lang]}</button>
  </div>
</header>

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 16px;
    margin-bottom: 300px;
  }

  nav ul {
    display: flex;
    gap: 2rem;
  }

  nav ul li {
    list-style: none;
  }

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
  }

  .dark_switch:hover {
    cursor: pointer;
  }

  .language_switch {
    width: 50px;
    height: 24px;
    border-radius: 4px;
    border: none;
    background: var(--lightgrey);
  }

  .language_switch:hover {
    background: var(--mediumgrey);
    cursor: pointer;
  }

  .strikethrough {
    text-decoration: line-through;
  }

  h1 {
    font-family: "Pulp Display", sans-serif;
    font-size: 18px;
  }
</style>
