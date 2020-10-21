<script context="module">
    const arrowRight = '->';
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
        width: 100%;
        max-width: 100%;
        position: relative;
        display: block;

        //padding: .75rem 1rem;
        //box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
        //box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        //transition: transform ease 200ms, box-shadow ease 200ms;
        //border-radius: 4px;
        //
        //&:hover {
        //    transform:
        //            scale(1.1)
        //            perspective(80em)
        //            rotateX(10deg);
        //    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
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

    //.description {
    //    margin-top: 1rem;
    //    font-family: $font-family-serif;
    //}

    .title {
        display: flex;
        justify-content: space-between;
    }

    .more {
        color: $lightskin;

        &:hover {
            font-weight: 700;
            color: $skin;

            * {
                color: $skin;
            }
        }
    }

    a {
        h5 {
            color: $lightskin;
        }

        &:hover {
            h5 {
                color: $skin;
            }
        }
    }

    .border {
        padding: .75rem 1rem;
        height: 100%;
        width: 100%;
    }

    .link {
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: space-between;
        align-items: start;
        //text-align: left;
        transition: box-shadow ease 250ms;
        flex-direction: column;
        flex-grow: 1;
        //margin-right: 2rem;
        padding-left: 1rem;
        border-left: 1px solid black;
    }

    //.red { &:before { content:'~>'; } &:hover { color: $red; &:before { background: transparentize($red, .75); }}}
    //.blue { &:before { content:'>]'; } &:hover { color: $blue; &:before { background: transparentize($blue, .75); }}}
    //.purple { &:before { content:'|>'; } &:hover { color: $purple; &:before { background: transparentize($purple, .75); }}}
    //.green { &:before { content:'=>'; } &:hover { color: $green; &:before { background: transparentize($green, .75); }}}

    .a { &:before { content:'~>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .b { &:before { content:'=>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    //.c { &:before { content:'>]'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .d { &:before { content:'>='; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    //.e { &:before { content:'|>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    //.f { &:before { content:'>:'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    //.g { &:before { content:'<>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}

    .tilt {

        box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
        transition: transform ease 200ms, box-shadow ease 200ms;
        //padding: 1rem;
        border-radius: 2px;

        &:hover {
            transform: scale(1.1) perspective(80em) rotateX(10deg);
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }
    }

    .link {
        display: grid;
        grid-template-columns: auto auto;

        h4, h5 {
            grid-column: 1 / 3;
        }

        //.description {
        //    grid-column: 1 / 2;
        //}

        .mono {
            grid-column: 2 / 3;
            align-self: end;
        }
    }

    .mono {
        color: $lightskin;
    }

</style>

<section>
    <div class="title">
        <h3>Neueste Projekte</h3>
        <a rel="external preload" href="projekte" class="more">Mehr <span class="mono">{@html arrowRight}</span></a>
    </div>

    <a rel="external preload" href="projekte/anonymes-personentracking" class="experience a tilt">
        <div class="border">
            <div class="link">
                <h4>Anonymes Personentracking</h4>
                <h5>Obacht!</h5>
<!--                <p class="description">-->
<!--                    Unser Produkt beschäftigt sich mit der anonymen Positionserfassung von Personen und Auswertung deren Verhaltens.-->
<!--                    Das System kann sowohl im privaten als auch im öffentlichen Raum zur Verwendung kommen.-->
<!--                </p>-->
                <span class="mono">
                    {@html arrowRight}
                </span>
            </div>
        </div>
    </a>
<!--    <div rel=external href="projekte/kamerabasiertes-besuchertracking-im-museum" class="experience blue tilt">-->
    <a rel="external preload" href="projekte/kamerabasiertes-besuchertracking-im-museum" class="experience b tilt">
        <div class="border">
            <div class="link">
                <h4>Kamerabasiertes Besuchertracking im Museum</h4>
                <h5>Theia Tracking</h5>
<!--                <p class="description">-->
<!--                    Als Prototyp für das anonyme Personentracking wurde ein bis auf die Sensoren gleiches System mit Kameras aufgebaut.-->
<!--                    Die Technologie wurde in Zusammenarbeit mit den Kunstsammlungen Augsburg in der Dauerausstellung des Schaezlerpalais getestet.-->
<!--                </p>-->
                <span class="mono">
                    {@html arrowRight}
                </span>
            </div>
        </div>
    </a>
    <div rel="external preload" class="experience d">
        <div class="border">
            <div class="link">
                <h4>Dart-Punkteerkennung mithilfe eines LiDAR-Sensors</h4>
                <h5>Robin Hood</h5>
<!--                <p class="description">-->
<!--                    Dieses Projekt handelt von einem Datenbankmodell für eine Finanz WebApp, vergleichbar mit FinanzGuru.-->
<!--                    Der Nutzer ist in der Lage Konten anzulegen, Transaktionen und Daueraufträge zu hinterlegen-->
<!--                    und Rechnungen zu verwalten. Außerdem werde alle Daten für eine intuitive Analyse aufbereitet.-->
<!--                </p>-->
                <span class="mono">
                    {@html arrowRight}
                </span>
            </div>
        </div>
    </div>
</section>
