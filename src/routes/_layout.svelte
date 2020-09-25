<script>
    import Footer from "../components/Footer.svelte";
    import Header from "../components/Header.svelte";
    // export let segment;
</script>

<style lang="scss" global>
    @import "./src/styles/global";
</style>

<Header />

<main>
	<slot></slot>
</main>

<Footer />
