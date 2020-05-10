<script>
    import { lang } from '../../../stores';
</script>

<style>
    .flex {
        display: flex;
        justify-content: space-between;
        margin-bottom: 4rem;
        padding-top: 0rem;
    }

    .flex-column {
        display: flex;
        flex-direction: column;
        justify-content: start;
    }

    .info {
        text-align: right;
        margin-top: 0;
        margin-bottom: 0;
        font-weight: 400;
        font-size: 1.2rem;
        color: black;
        text-decoration: none;
    }

    .slogan {
        text-transform: uppercase;
        margin-top: .5rem;
        margin-bottom: 0;
        font-weight: 400;
        line-height: 1em;
        font-size: 1.5rem;
    }
</style>

<section class="flex">
    <!--        <div class="slogan"><b>Entwickler</b><br>mit Auge<br>für Design,<br><b>Macher</b></div>-->
    {#if $lang === 'de'}
        <div class="slogan">Zur Zeit arbeite ich an<br><b>Anonymer Personen Analyse</b><br><br>Ich studiere <b>Interaktive Medien B.A.</b></div>
    {/if}
    {#if $lang === 'en' }
        <div class="slogan">Currently working on<br><b>Anonymous Crowd Analysis</b><br><br>Studying <b>Interactive Media B.A.</b></div>
    {/if}
    <div class="flex-column">
        <a class="info" href="https://linkedin.com/in/vrogg">linkedin.com/in/vrogg</a>
<!--        <a class="info" href="tel:+49 (175) 7090991">+49 (175) 70 90 991</a>-->
        <a class="info" href="mailto:valentin.rogg@hs-augsburg.de">mail@valentinrogg.de</a>
    </div>

</section>
