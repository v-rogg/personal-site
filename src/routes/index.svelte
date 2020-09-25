<script>
    import Header from "../components/Header.svelte";
    import CV from "../components/CV.svelte";
    import Skills from "../components/Skills.svelte";

    import {onMount} from 'svelte';

    let Tilt;

    onMount(async () => {
        const module = await import('../components/Tilt.svelte');
        Tilt = module.default;
    });
</script>

<style lang="scss">
    .split {
        display: flex;
    }
</style>

<svelte:head>
	<title>Valentin Rogg</title>
    <meta name="description" content="Valentin Rogg ist Entrepreneur aus Augsburg, Deutschland und Mitgründer der Rogg'n'Schunck GmbH" />
    <meta name="keywords" content="Valentin Rogg, Entrepreneur, Startup, Anonymes Personentracking, Svelte, JavaScript, Rust">
    <meta name="author" content="Valentin Rogg">
</svelte:head>

<Header />

<div class="split container">
    <CV />
    <Skills />
</div>

<svelte:component this={Tilt} />
