<script>
    import Footer from "../components/Footer.svelte";
    import Header from "../components/Header.svelte";
    // export let segment;
</script>

<style lang="scss" global>
    @import "./src/styles/global";
</style>

<svelte:head>
    <title>Valentin Rogg</title>
    <meta name="description" content="Valentin Rogg ist Entrepreneur aus Augsburg, Deutschland und Mitgründer der Rogg'n'Schunck GmbH" />
    <meta name="keywords" content="Valentin Rogg, Entrepreneur, Startup, Anonymes Personentracking, Svelte, JavaScript, Rust">
    <meta name="author" content="Valentin Rogg">
</svelte:head>

<Header />

<main>
	<slot></slot>
</main>

<Footer />
