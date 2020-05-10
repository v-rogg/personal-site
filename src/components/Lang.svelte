<script>
    import {lang} from '../stores';

    function setEN() {
        $lang = 'en';
    }

    function setDE() {
        $lang = 'de';
    }
</script>

<style>
    .b {
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

    .lang-switches {
        height: fit-content;
        text-align: right;
        width: 30%;
    }
    @media (max-width: 991.98px) {
        .lang-switches {
            order: 3;
            width: 30%;
            /*flex: 2 0 50%;*/
        }
    }
</style>

<div class="lang-switches">
    <button on:click={setDE} class:b="{$lang === 'de'}" disabled="{$lang === 'de'}">de</button>
    /
    <button on:click={setEN} class:b="{$lang === 'en'}" disabled="{$lang === 'en'}">en</button>
</div>
