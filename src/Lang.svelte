<script>
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    function setEN() {
        dispatch('language', {
            lang: "en"
        });
    }

    function setDE() {
        dispatch('language', {
            lang: "de"
        });
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
        font-weight: 700;
    }

    .lang {
        display: flex;
        justify-content: space-between;
    }

    .fixed {
        position: fixed;
        background: white;
        width: 100%;
        top: 0;
    }
</style>

<div class="fixed">
    <div class="lang container">
        <h1>Valentin Rogg</h1>
        <div>
            <button on:click={setDE}>de</button>
            /
            <button on:click={setEN}>en</button>
        </div>
    </div>
</div>
