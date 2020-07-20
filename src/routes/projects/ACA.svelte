<script>
    import { lang } from '../../stores';
</script>

<style lang="scss">
    h2 {
        display: flex;
        flex-direction: column;
        justify-content: center;
        font-weight: 900 !important;
    }

    .subline {
        /*width: 50%;*/
        font-weight: 600;
    }

    .description {
        margin-top: 2rem;
        /*width: 50%;*/
    }

    .header {
        margin-top: .5rem;
        margin-bottom: 1rem;
        display: flex;
        justify-content: space-between;
    }

    a {
        text-decoration: none;
        /*color: #FF8000;*/
        /*color: rgb(0, 0, 238);*/
        color: #0000EE;
        color: #551A8B;

        &:hover {
            text-decoration: underline;
        }
    }

    .links {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
</style>

<section>
    <div class="aca container">
        <div class="header">
            <h2>
                {#if $lang === 'de'}
                    Kamerabasiertes Besuchertracking
                {/if}
                {#if $lang === 'en' }
                    Camera-based visitor tracking
                {/if}
            </h2>
            <div class="links">
                <a href="https://theia-tracking.de">
                    {#if $lang === 'de'}
                        Zur Website
                    {/if}
                    {#if $lang === 'en' }
                        To website
                    {/if}
                </a>
            </div>
        </div>
        <div class="subline">
            {#if $lang === 'de'}
                Unser Produkt beschäftigt sich mit der anonymen Positionserfassung von Personen und Auswertung deren Verhaltens.<br>
                Das System kann vor allem im Einzelhandel, Sportstudios oder dem öffentlichen Raum zur Verwendung kommen.
            {/if}
            {#if $lang === 'en' }
                Our product deals with the anonymous tracking of the position of persons and complete crowds and evaluation of their behaviour.<br>
                The system can be used mainly in retail, sports studios or public spaces.
            {/if}
        </div>
        <div class="description">
            {#if $lang === 'de'}
                Mithilfe von Sensoren wird die Position von Personen anonym erfasst und abgespeichert.
                Diese Positionsdaten werden auf Laufwege und Aufenthaltsorte analysiert. Mithilfe dieser Daten
                können Statistiken erstellt, und Aussagen über die Beliebheit und Platzierung von bestimmten
                Bezugspunkten getroffen werden. Diese Aussagen und Statistiken werden durch ein Dashboards aufbereitet präsentiert.
            {/if}
            {#if $lang === 'en' }
                With the help of sensors, the position of people is anonymously tracked and stored.
                This position data is analyzed for routes and locations. With the help of this data
                we can create statistics, and make statements about the popularity and placement of certain
                reference points are hit. These statements and statistics are presented in a dashboard.
            {/if}
        </div>
    </div>
</section>
