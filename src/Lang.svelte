<script>
    import { navigateTo } from 'svelte-router-spa';
    import {createEventDispatcher} from "svelte";

    export let lang;

    const dispatch = createEventDispatcher();

    function setEN() {
        dispatch('language', {
            lang: "en"
        });
        navigateTo('lebenslauf', 'en');
    }

    function setDE() {
        dispatch('language', {
            lang: "de"
        });
        navigateTo('lebenslauf', 'de');
    }

    function navMain() {
        navigateTo('/');
    }
</script>

<style>
    h1 {
        text-transform: uppercase;
        font-size: 2rem;
        margin-top: 0;
        margin-bottom: 0;
        font-weight: 700;
    }

    button {
        border: none;
        background: none;
        margin: 0;
        cursor: pointer;
        font-weight: 400;
    }

    button:disabled {
        color: black;
        cursor: text;
    }

    .b {
        font-weight: 700;
    }

    .lang {
        display: flex;
        justify-content: space-between;
    }

    .fixed {
        position: fixed;
        /*background: white;*/
        width: 100%;
        top: 0;
    }

    .pointer {
        cursor: pointer;
    }

    /*!* Buzz Out *!*/
    /*@-webkit-keyframes hvr-buzz-out {*/
    /*    10% {*/
    /*        -webkit-transform: translateX(3px) rotate(2deg);*/
    /*        transform: translateX(3px) rotate(2deg);*/
    /*    }*/
    /*    20% {*/
    /*        -webkit-transform: translateX(-3px) rotate(-2deg);*/
    /*        transform: translateX(-3px) rotate(-2deg);*/
    /*    }*/
    /*    30% {*/
    /*        -webkit-transform: translateX(3px) rotate(2deg);*/
    /*        transform: translateX(3px) rotate(2deg);*/
    /*    }*/
    /*    40% {*/
    /*        -webkit-transform: translateX(-3px) rotate(-2deg);*/
    /*        transform: translateX(-3px) rotate(-2deg);*/
    /*    }*/
    /*    50% {*/
    /*        -webkit-transform: translateX(2px) rotate(1deg);*/
    /*        transform: translateX(2px) rotate(1deg);*/
    /*    }*/
    /*    60% {*/
    /*        -webkit-transform: translateX(-2px) rotate(-1deg);*/
    /*        transform: translateX(-2px) rotate(-1deg);*/
    /*    }*/
    /*    70% {*/
    /*        -webkit-transform: translateX(2px) rotate(1deg);*/
    /*        transform: translateX(2px) rotate(1deg);*/
    /*    }*/
    /*    80% {*/
    /*        -webkit-transform: translateX(-2px) rotate(-1deg);*/
    /*        transform: translateX(-2px) rotate(-1deg);*/
    /*    }*/
    /*    90% {*/
    /*        -webkit-transform: translateX(1px) rotate(0);*/
    /*        transform: translateX(1px) rotate(0);*/
    /*    }*/
    /*    100% {*/
    /*        -webkit-transform: translateX(-1px) rotate(0);*/
    /*        transform: translateX(-1px) rotate(0);*/
    /*    }*/
    /*}*/
    /*@keyframes hvr-buzz-out {*/
    /*    10% {*/
    /*        -webkit-transform: translateX(3px) rotate(2deg);*/
    /*        transform: translateX(3px) rotate(2deg);*/
    /*    }*/
    /*    20% {*/
    /*        -webkit-transform: translateX(-3px) rotate(-2deg);*/
    /*        transform: translateX(-3px) rotate(-2deg);*/
    /*    }*/
    /*    30% {*/
    /*        -webkit-transform: translateX(3px) rotate(2deg);*/
    /*        transform: translateX(3px) rotate(2deg);*/
    /*    }*/
    /*    40% {*/
    /*        -webkit-transform: translateX(-3px) rotate(-2deg);*/
    /*        transform: translateX(-3px) rotate(-2deg);*/
    /*    }*/
    /*    50% {*/
    /*        -webkit-transform: translateX(2px) rotate(1deg);*/
    /*        transform: translateX(2px) rotate(1deg);*/
    /*    }*/
    /*    60% {*/
    /*        -webkit-transform: translateX(-2px) rotate(-1deg);*/
    /*        transform: translateX(-2px) rotate(-1deg);*/
    /*    }*/
    /*    70% {*/
    /*        -webkit-transform: translateX(2px) rotate(1deg);*/
    /*        transform: translateX(2px) rotate(1deg);*/
    /*    }*/
    /*    80% {*/
    /*        -webkit-transform: translateX(-2px) rotate(-1deg);*/
    /*        transform: translateX(-2px) rotate(-1deg);*/
    /*    }*/
    /*    90% {*/
    /*        -webkit-transform: translateX(1px) rotate(0);*/
    /*        transform: translateX(1px) rotate(0);*/
    /*    }*/
    /*    100% {*/
    /*        -webkit-transform: translateX(-1px) rotate(0);*/
    /*        transform: translateX(-1px) rotate(0);*/
    /*    }*/
    /*}*/
    /*.hvr-buzz-out {*/
    /*    display: inline-block;*/
    /*    vertical-align: middle;*/
    /*    -webkit-transform: perspective(1px) translateZ(0);*/
    /*    transform: perspective(1px) translateZ(0);*/
    /*    box-shadow: 0 0 1px rgba(0, 0, 0, 0);*/
    /*}*/
    /*.hvr-buzz-out:hover, .hvr-buzz-out:focus, .hvr-buzz-out:active {*/
    /*    -webkit-animation-name: hvr-buzz-out;*/
    /*    animation-name: hvr-buzz-out;*/
    /*    -webkit-animation-duration: 0.75s;*/
    /*    animation-duration: 0.75s;*/
    /*    -webkit-animation-timing-function: linear;*/
    /*    animation-timing-function: linear;*/
    /*    -webkit-animation-iteration-count: 1;*/
    /*    animation-iteration-count: 1;*/
    /*}*/
</style>

<div class="fixed">
    <section class="lang container">
        <h1 on:click={navMain} class="pointer"><!--<span class="mr hvr-buzz-out">♛</span>--> Valentin Rogg</h1>
        <div>
            <button on:click={setDE} class:b="{lang === 'de'}" disabled="{lang === 'de'}">de</button>
            /
            <button on:click={setEN} class:b="{lang === 'en'}" disabled="{lang === 'en'}">en</button>
        </div>
    </section>
</div>
