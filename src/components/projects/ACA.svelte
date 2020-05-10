<script>
    import { lang } from '../../stores';
</script>

<style scoped lang="scss">
    h2 {
        margin-top: .5rem;
        margin-bottom: 1rem;
    }

    .description {
        margin-top: 1rem;
    }

    .header {
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
</style>

<section class="aca">
    <div class="header">
        <h2>
            {#if $lang === 'de'}
                <b>Anonymous Crowd Analysis</b>
            {/if}
            {#if $lang === 'en' }
                <b>Anonyme Menschenmenge Analyse</b>
            {/if}
        </h2>
        <div class="links">
            <a href="https://www.theia-tracking.de">Zur Website</a>
        </div>
    </div>
    <div>
        Unser Produkt beschäftigt sich mit der anonymen Positionsefassung von Personen (Besuchern) in abgesteckten Gebieten, wie z.B. Freizeitparks, Museen und Sportstudios, mit besonderen Bezugspunkten, wie z.B. Achterbahnen, Ausstellungsstücken oder Fitnessgeräten.
    </div>
    <div class="description">
        Mithilfe von Sensoren wir die Postition von Personen anonym erfasst und abgespeichert.
        Diese Positionsdaten werden auf Laufwege und Aufenthaltsorte analysiert. Mithilfe diser Daten
        können Statisitken erstellt und Aussagen über die Beliebheit und Platzierung von bestimmten
        Bezugspunkten getroffen werden. Diese Aussagen und Statisitken werden durch ein Dashboards aufbereitet präsentiert.
    </div>
</section>
