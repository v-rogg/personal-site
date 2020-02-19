<script>
    import Header from "./Header.svelte";
    import Lang from "./Lang.svelte";

    export let lang = "de";

    function changeLang(event) {
        lang = event.detail.lang
    }
</script>

<style>

    @import url('https://fonts.googleapis.com/css?family=Source+Sans+Pro:300,400,600,700&display=swap');

    main {
        max-width: 1200px;
        margin: 0 auto;

        font-family: 'Source Sans Pro', sans-serif;
    }

</style>

<main>
    <Lang on:language={changeLang}/>
    <Header {lang}/>
</main>
