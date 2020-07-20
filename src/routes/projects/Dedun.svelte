<script>
    import { lang } from '../../stores';
</script>

<style lang="scss">
    h2 {
        display: flex;
        flex-direction: column;
        justify-content: center;
        font-weight: 900;
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

    .dedun {
        margin-top: 4rem;
    }
</style>

<section>
    <div class="dedun container">
        <div class="header">
            <h2>
                {#if $lang === 'de'}
                    Datenbank für ein Finanzprogramnm
                {/if}
                {#if $lang === 'en' }
                    Database for a finance software
                {/if}
            </h2>
<!--            <div class="links">-->
<!--                <a href="https://app.theia-tracking.de">-->
<!--                    {#if $lang === 'de'}-->
<!--                        Zur Website-->
<!--                    {/if}-->
<!--                    {#if $lang === 'en' }-->
<!--                        To website-->
<!--                    {/if}-->
<!--                </a>-->
<!--            </div>-->
        </div>
        <div class="subline">
            {#if $lang === 'de'}
                Dieses Projekt handelt von einem Datenbankmodell für eine Finanz WebApp, vergleichbar mit FinanzGuru.
                Der Nutzer ist in der Lage Konten anzulegen, Transaktionen und Daueraufträge zu hinterlegen
                und Rechnungen zu verwalten. Außerdem werde alle Daten für eine intuitive Analyse aufbereitet.
            {/if}
            {#if $lang === 'en' }
                This project is about a database model for a financial web application, comparable to FinanzGuru.
                The user is able to create accounts, deposit transactions and standing orders and manage invoices.
                In addition, all data is prepared for intuitive analysis.
            {/if}
        </div>
<!--        <div class="description">-->
<!--            {#if $lang === 'de'}-->
<!--                Mithilfe von Sensoren wir die Postition von Personen anonym erfasst und abgespeichert.-->
<!--                Diese Positionsdaten werden auf Laufwege und Aufenthaltsorte analysiert. Mithilfe diser Daten-->
<!--                können Statisitken erstellt, und Aussagen über die Beliebheit und Platzierung von bestimmten-->
<!--                Bezugspunkten getroffen werden. Diese Aussagen und Statisitken werden durch ein Dashboards aufbereitet präsentiert.-->
<!--            {/if}-->
<!--            {#if $lang === 'en' }-->
<!--                With the help of sensors, the position of people is anonymously tracked and stored.-->
<!--                This position data is analyzed for routes and locations. With the help of this data-->
<!--                we can create statistics, and make statements about the popularity and placement of certain-->
<!--                reference points are hit. These statements and statistics are presented in a dashboard.-->
<!--            {/if}-->
<!--        </div>-->
    </div>
</section>
