<script>
    import Nav from '../components/Nav.svelte';
    import Header from "../components/Header.svelte";
    import Footer from "../components/Footer.svelte";
    import { onMount } from 'svelte';
    import { goto } from '@sapper/app';


    export let segment;

    let touchstartX = 0;
    let touchstartY = 0;
    let touchendX = 0;
    let touchendY = 0;
    let swipe = "";

    onMount(() => {

        const gestureZone = document.body;

        gestureZone.addEventListener('touchstart', function(event) {
            touchstartX = event.changedTouches[0].screenX;
            touchstartY = event.changedTouches[0].screenY;
        }, false);

        gestureZone.addEventListener('touchend', function(event) {
            touchendX = event.changedTouches[0].screenX;
            touchendY = event.changedTouches[0].screenY;
            handleGesture();
        }, false);

        function handleGesture() {
            // if (touchendX < touchstartX) {
            //     goto("/about");
            //     swipe = "Nice one :D";
            // }
            //
            // if (touchendX > touchstartX) {
            //     goto("/");
            //     swipe = "Uhh you swipped!";
            // }

            // if (touchendY < touchstartY) {
            //     console.log('Swiped up');
            //     swipe = "Up";
            // }
            //
            // if (touchendY > touchstartY) {
            //     console.log('Swiped down');
            //     swipe = "Down";
            // }

            // if (touchendY === touchstartY) {
            //     console.log('Tap');
            //     swipe = "Tap";
            // }
        }
    })

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

<main id="modalContent" class="main--mt">
    <slot/>
</main>

<footer>
    <Footer>{swipe}</Footer>
</footer>
