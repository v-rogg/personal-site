<script>
    import Header from "./Header.svelte";
    import Lang from "./Lang.svelte";
    import Car from "./Car.svelte";

    export let lang = "de";

    function changeLang(event) {
        lang = event.detail.lang
    }
</script>

<style>

    @import url('https://fonts.googleapis.com/css?family=Source+Sans+Pro:300,400,600,700&display=swap');

    main {
        font-family: 'Source Sans Pro', sans-serif;
    }

    :global(.container){
        max-width: 1200px;
        margin: 0 auto;
    }

    /* Box sizing rules */
    :global(*,
    *::before,
    *::after) {
        box-sizing: border-box;
    }

    /* Remove default padding */
    :global(ul[class],
    ol[class]) {
        padding: 0;
    }

    /* Remove default margin */
    :global(body,
    h1,
    h2,
    h3,
    h4,
    p,
    ul[class],
    ol[class],
    li,
    figure,
    figcaption,
    blockquote,
    dl,
    dd) {
        margin: 0;
        padding: 0;
    }

    /* Set core body defaults */
    :global(body) {
        min-height: 100vh;
        scroll-behavior: smooth;
        text-rendering: optimizeSpeed;
        line-height: 1.5;
    }

    :global(canvas) {
        display: block;
    }

    /* Remove list styles on ul, ol elements with a class attribute */
    :global(ul[class],
    ol[class]) {
        list-style: none;
    }

    /* A elements that don't have a class get default styles */
    :global(a:not([class])) {
        text-decoration-skip-ink: auto;
    }

    /* Make images easier to work with */
    :global(img) {
        max-width: 100%;
        display: block;
    }

    /* Inherit fonts for inputs and buttons */
    :global(input,
    button,
    textarea,
    select) {
        font: inherit;
    }

    /* Remove all animations and transitions for people that prefer not to see them */
    @media (prefers-reduced-motion: reduce) {
        * {
            animation-duration: 0.01ms !important;
            animation-iteration-count: 1 !important;
            transition-duration: 0.01ms !important;
            scroll-behavior: auto !important;
        }
    }

</style>

<svelte:head>
    <title>Valentin Rogg</title>
</svelte:head>

<main>
    <Lang on:language={changeLang}/>
    <Header {lang}/>
    <Car />
</main>
