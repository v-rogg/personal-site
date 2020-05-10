<script xmlns="http://www.w3.org/1999/html">
    import { lang } from '../../../stores';
    import Info from './Info.svelte';
</script>

<style lang="scss">
    @import "../../../../static/styles/theme";

    .flex {
        display: flex;
        justify-content: space-between;
        margin-bottom: 4rem;
        padding-top: 0rem;
    }

    @media (max-width: 991.98px) {
        .flex {
            flex-direction: column-reverse;
        }
    }

    .slogan {
        text-transform: uppercase;
        margin-top: .5rem;
        margin-bottom: 0;
        font-weight: 400;
        line-height: 1em;
        font-size: 1.5rem;
    }

    .orange {
        color: $orange;
    }

    .blue {
        color: $blue;
    }

    .red {
        color: $red;
    }
</style>

<section class="flex">
    <div class="slogan">
        {#if $lang === 'de'}
            Zur Zeit arbeite ich an<br><b><span class="orange">Anonymer</span> <span class="blue">Personen</span> <span class="red">Analyse</span></b><br><br>Ich studiere <b>Interaktive Medien B.A.</b>
        {/if}
        {#if $lang === 'en' }
            Currently working on<br><b><span class="orange">Anonymous</span> <span class="blue">Crowd</span> <span class="red">Analysis</span></b><br><br>Studying <b>Interactive Media B.A.</b>
        {/if}
    </div>

    <Info/>
</section>
