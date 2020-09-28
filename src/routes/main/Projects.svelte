<script context="module">
    const arrowRight = '<span class="mono">-></span>';
</script>

<style lang="scss">
    @import "../../styles/theme";

    section {
        grid-area: 1 / 2 / 2 / 3;
    }

    @media (max-width: 991.98px) {
        section {
            grid-area: 1 / 1 / 2 / 2;
        }
    }

    .experience {
        margin: 2rem 0;
        width: max-content;
        max-width: 100%;
        position: relative;
        display: block;

        //&:first-of-type {
        //    margin: 1.25rem 0 2rem;
        //}

        &:before {
            position: absolute;
            left: -2rem;
            font-family: 'JetBrains Mono', sans-serif;
            top: 50%;
            line-height: 0;
            opacity: 0;
            transition: opacity linear 100ms;
        }

        &:hover {

            &:before {
                opacity: 1;
            }
        }
    }

    .description {
        font-family: $font-family-serif;
    }

    .title {
        display: flex;
        justify-content: space-between;
    }

    .more {
        color: $lightskin;

        &:hover {
            font-weight: 700;
            color: $skin;
        }
    }

    .red { &:before { content:'~>'; } &:hover { color: $red; &:before { background: transparentize($red, .75); }}}
    .blue { &:before { content:'>]'; } &:hover { color: $blue; &:before { background: transparentize($blue, .75); }}}
    .purple { &:before { content:'|>'; } &:hover { color: $purple; &:before { background: transparentize($purple, .75); }}}
    .green { &:before { content:'=>'; } &:hover { color: $green; &:before { background: transparentize($green, .75); }}}
</style>

<section>
    <div class="title">
        <h3>Neueste Projekte</h3>
        <a rel=external href="projekte" class="more">Mehr {@html arrowRight}</a>
    </div>

    <a rel=external href="projekte/anonymes-personentracking" class="experience purple tilt">
        <h4>Anonymes Personentracking</h4>
        <p class="description">
            Unser Produkt beschäftigt sich mit der anonymen Positionserfassung von Personen und Auswertung deren Verhaltens.
            Das System kann vor allem im Einzelhandel, Sportstudios oder dem öffentlichen Raum zur Verwendung kommen.
        </p>
    </a>
    <a rel=external href="projekte/kamerabasiertes-besuchertracking-im-museum" class="experience blue tilt">
        <h4>Kamerabasiertes Besuchertracking im Museum</h4>
        <h5>Theia Tracking</h5>
        <p class="description">
            Als Prototyp für das anonyme Personentracking wurde ein bis auf die Sensoren gleiches System mit Kameras aufgebaut.
            Die Technologie wurde in Zusammenarbeit mit den Kunstsammlungen Augsburg in der Dauerausstellung des Schaezlerpalais getestet.
        </p>
    </a>
    <a rel=external href="projekte/datenbank-fuer-ein-finanzprogramm" class="experience green tilt">
        <h4>Datenbank für ein Finanzprogramnm</h4>
        <h5>Dedun</h5>
        <p class="description">
            Dieses Projekt handelt von einem Datenbankmodell für eine Finanz WebApp, vergleichbar mit FinanzGuru.
            Der Nutzer ist in der Lage Konten anzulegen, Transaktionen und Daueraufträge zu hinterlegen
            und Rechnungen zu verwalten. Außerdem werde alle Daten für eine intuitive Analyse aufbereitet.
        </p>
    </a>
</section>
