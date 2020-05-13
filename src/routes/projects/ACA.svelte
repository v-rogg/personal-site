<script>
    import { lang } from '../../stores';
</script>

<style scoped lang="scss">
    h2 {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .subline {
        /*width: 50%;*/
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
        color: #FF8000;

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
                <b>
                    {#if $lang === 'de'}
                        Anonyme Menschenmenge Analyse
                    {/if}
                    {#if $lang === 'en' }
                        Anonymous Crowd Analysis
                    {/if}
                </b>
            </h2>
            <div class="links">
                <a href="https://app.theia-tracking.de">
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
                Unser Produkt beschäftigt sich mit der anonymen Positionsefassung von Personen und Auswertung deren Verhalten.<br>
                Das System kann vor allem im Einzelhandel, Sportstudios oder dem öffentlichen Raum zur Verwendung kommen.
            {/if}
            {#if $lang === 'en' }
                Our product deals with the anonymous tracking of the position of persons and complete crowds and evaluation of their behaviour.<br>
                The system can be used mainly in retail, sports studios or public spaces.
            {/if}
        </div>
        <div class="description">
            {#if $lang === 'de'}
                Mithilfe von Sensoren wir die Postition von Personen anonym erfasst und abgespeichert.
                Diese Positionsdaten werden auf Laufwege und Aufenthaltsorte analysiert. Mithilfe diser Daten
                können Statisitken erstellt, und Aussagen über die Beliebheit und Platzierung von bestimmten
                Bezugspunkten getroffen werden. Diese Aussagen und Statisitken werden durch ein Dashboards aufbereitet präsentiert.
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
