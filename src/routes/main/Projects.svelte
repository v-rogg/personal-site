<script>
</script>

<style lang="scss">
    @import "../../styles/theme";

    .experience {
        margin: 1.25rem 0;
        width: max-content;
        max-width: 100%;
        position: relative;

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

    .w50 {
        width: 50%
    }

    @media (max-width: 991.98px) {
        .w50 {
            margin-bottom: 4rem;
        }
    }

    div {
        font-family: $font-family-sans;
    }

    .description {
        font-family: $font-family-serif;
    }

    .red { &:before { content:'~>'; } &:hover { color: $red; &:before { background: transparentize($red, .75); }}}
    .blue { &:before { content:'>]'; } &:hover { color: $blue; &:before { background: transparentize($blue, .75); }}}
    .purple { &:before { content:'|>'; } &:hover { color: $purple; &:before { background: transparentize($purple, .75); }}}
    .green { &:before { content:'=>'; } &:hover { color: $green; &:before { background: transparentize($green, .75); }}}
</style>

<section>
    <h3>Letzte Projekte</h3>

    <div class="experience purple tilt">
        <h4>Anonymes Personentracking</h4>
        <p class="description">
            Unser Produkt beschäftigt sich mit der anonymen Positionserfassung von Personen und Auswertung deren Verhaltens.
            Das System kann vor allem im Einzelhandel, Sportstudios oder dem öffentlichen Raum zur Verwendung kommen.
        </p>
    </div>
    <div class="experience blue tilt">
        <h4>Kamerabasiertes Personentracking</h4>
        <p class="description">
            Als Prototyp für das anonyme Personentracking wurde ein bis auf die Sensoren gleiches System mit Kameras aufgebaut.
            Die Technologie wurde in Zusammenarbeit mit den Kunstsammlungen Augsburg in der Dauerausstellung des Schaezlerpalais getestet.
        </p>
    </div>
    <div class="experience green tilt">
        <h4>Datenbank für ein Finanzprogramnm</h4>
        <p class="description">
            Dieses Projekt handelt von einem Datenbankmodell für eine Finanz WebApp, vergleichbar mit FinanzGuru.
            Der Nutzer ist in der Lage Konten anzulegen, Transaktionen und Daueraufträge zu hinterlegen
            und Rechnungen zu verwalten. Außerdem werde alle Daten für eine intuitive Analyse aufbereitet.
        </p>
    </div>
</section>
