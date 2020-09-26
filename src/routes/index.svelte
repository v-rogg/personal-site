<script>
    import {onMount} from 'svelte';
    import CV from "./main/CV.svelte";
    import Skills from "./main/Skills.svelte";
    import Projects from "./main/Projects.svelte";

    let Tilt;

    onMount(async () => {
        const module = await import('../components/Tilt.svelte');
        Tilt = module.default;
    });
</script>

<style lang="scss">
    .split {
        display: flex;

        .left {
            width: 40%;
        }

        .right {
            width: 60%;
        }
    }
</style>

<svelte:head>
    <title>Valentin Rogg</title>
    <meta name="description" content="Valentin Rogg ist Entrepreneur aus Augsburg, Deutschland und Mitgründer der Rogg'n'Schunck GmbH" />
    <meta name="keywords" content="Valentin Rogg, Entrepreneur, Startup, Anonymes Personentracking, Svelte, JavaScript, Rust">
    <meta name="author" content="Valentin Rogg">
</svelte:head>

<div class="split container">
    <div class="left">
        <CV />
    </div>
    <div class="right">
        <Projects />
        <Skills />
    </div>
</div>

<svelte:component this={Tilt} />
