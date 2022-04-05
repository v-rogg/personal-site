<script lang="ts">
  import { page } from '$app/stores';
</script>

<header>
  <h1 class="corner">
    <a href="/">
      Valentin Rogg
    </a>
  </h1>

  <nav>
    <ul>
      <li class:active={$page.url.pathname === '/'}><a sveltekit:prefetch href="/">Home</a></li>
      <li class:active={$page.url.pathname === '/blog'} class="strikethrough">
        <a sveltekit:prefetch href="/" aria-disabled="true">Blog</a>
      </li>
      <li class:active={$page.url.pathname === '/projects'} class="strikethrough">
        <a sveltekit:prefetch href="/" aria-disabled="true">Projects</a>
      </li>
    </ul>
  </nav>

  <div class="options">
    Dark/Light Mode, Language
  </div>
</header>

<style>
  .strikethrough {
    text-decoration: line-through;
  }
</style>
