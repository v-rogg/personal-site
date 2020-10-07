<script>
  // import { onMount } from 'svelte';
  import Footer from '../components/Footer.svelte';
  import Header from '../components/Header.svelte';

  // let Tilt;
  //
  // onMount(async () => {
  //   const module = await import('../components/Tilt.svelte');
  //   Tilt = module.default;
  // });
</script>

<style lang="scss" global>
  @import './src/styles/global';
</style>

<svelte:head>
  <title>Valentin Rogg</title>
  <meta name="description" content="Valentin Rogg ist Entrepreneur aus Augsburg, Deutschland und Mitgründer von Rogg'n'Schunck" />
  <meta name="keywords" content="Valentin Rogg, Entrepreneur, Startup, Anonymes Personentracking, Rogg'n'Schunck, Rogg & Schunck" />
  <meta name="author" content="Valentin Rogg" />
</svelte:head>

<Header />

<main>
  <slot />
</main>

<Footer />

<!--<svelte:component this={Tilt} />-->
