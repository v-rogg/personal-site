<script>
    import Nav from '../components/Nav.svelte';
    import Header from "../components/Header.svelte";
    import Footer from "../components/Footer.svelte";

    export let segment;
</script>

<style>
    /*main {*/
    /*	position: relative;*/
    /*	max-width: 56em;*/
    /*	background-color: white;*/
    /*	padding: 2em;*/
    /*	margin: 0 auto;*/
    /*	box-sizing: border-box;*/
    /*}*/

    .fixed {
        position: fixed;
        background: white;
        width: 100%;
        /*width: 98%;*/
        top: 0;
        margin: 0 auto;
    }

    header {
        z-index: 1090;
    }

    .main--mt {
        padding-top: 12rem;
    }

    @media (max-width: 991.98px) {
        .main--mt {
            padding-top: 16rem;
        }
    }
</style>

<header class="fixed">
    <Header/>
    <Nav {segment}/>
</header>

<main class="main--mt">
    <slot/>
</main>

<footer>
    <Footer/>
</footer>
