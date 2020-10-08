<script context="module">
    const arrowRight = '->';
    const inProgress = 'In Arbeit';
    const done = 'Beendet';
</script>

<style lang="scss">
    @import '../../styles/theme';

    ul {
        list-style: none;
        padding: 0;
        display: grid;
        grid-template-columns: repeat(3, 1fr);

        li {
            //padding-left: 1rem;
            //border-left: 1px solid black;
            height: 8rem;
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
    }

    .border {
        padding: .75rem 1rem;
        height: 100%;
        width: 100%;
    }

    li:nth-child(3n - 2) {
        margin: 1rem 1.5rem 1rem 0;
    }

    li:nth-child(3n - 1) {
        margin: 1rem .75rem;
    }

    li:nth-child(3n) {
        margin: 1rem 0 1rem 1.5rem;
    }

    @media (max-width: 991.98px) {
        ul {
            grid-template-columns: 100%;
        }

        li:nth-child(3n - 2) {
            margin: 2rem 0;
        }

        li:nth-child(3n - 1) {
            margin: 2rem 0;
        }

        li:nth-child(3n) {
            margin: 2rem 0;
        }
    }

    .back {
        margin-top: 5rem;
        display: block;
        color: $lightskin;

        &:hover {
            font-weight: 700;
            color: $skin;
        }
    }

    h3 {
        margin: 0 0 1.25rem;
    }

    .check {
        width: 2rem;
    }

    .inProgress, .done {
        display: flex;
        width: 100%;
        justify-content: space-between;
        justify-self: flex-end;
        color: $lightskin;
    }

    //.darkyellow { &:before { content:'>='; } &:hover { *, &:before { color: $darkyellow; } border-color: $darkyellow; }}
    //.blue { &:before { content:'>]'; } &:hover { *, &:before { color: $blue; } border-color: $blue; }}
    //.purple { &:before { content:'|>'; } &:hover { *, &:before { color: $purple; } border-color: $purple; }}
    //.green { &:before { content:'=>'; } &:hover { *, &:before { color: $green; } border-color: $green; }}
    //.magenta { &:before { content:'>:'; } &:hover { *, &:before { color: $magenta; } border-color: $magenta; }}
    //.anotherblue { &:before { content:'<>'; } &:hover { *, &:before { color: $anotherblue; } border-color: $anotherblue; }}
    //.anothergreen { &:before { content:'<>'; } &:hover { *, &:before { color: $anothergreen; } border-color: $anothergreen; }}
    //.red { &:before { content:'~>'; } &:hover { *, &:before { color: $red; } border-color: $red; }}

    .a { &:before { content:'~>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .b { &:before { content:'=>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .c { &:before { content:'>]'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .d { &:before { content:'>='; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .e { &:before { content:'|>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .f { &:before { content:'>:'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .g { &:before { content:'<>'; } &:hover { *, &:before { color: $lightskin; } .mono { color: $skin } * { border-color: $skin !important; }}}

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
</style>

<svelte:head>
    <title>Valentin Rogg - Projekte</title>
</svelte:head>


<h3>Ein Auszug meiner bisherigen Projekte</h3>

<ul>
    <li class="tilt a">
        <div class="border">
            <a rel=external href="projekte/anonymes-personentracking" class="link">
                <div class="title">
                    <h4>Anonymes Personentracking</h4>
                    <h5>Obacht!</h5>
                </div>
                <div class="inProgress">{@html inProgress} <span class="mono">{@html arrowRight}</span></div>
            </a>
        </div>
    </li>
    <li class="">
<!--        <a rel=external href="projekte/darterkennung-mit-lidar">-->
        <div class="border">
            <div class="link">
                <h4>Dart-Erkennung mithilfe eines LiDAR-Sensors</h4>
                <div class="inProgress">{@html inProgress}</div>
            </div>
        </div>
    </li>
    <li class="">
<!--        <a rel=external href="projekte/daniel-karrasch-website" class="link">-->
        <div class="border">
            <div class="link">
                <h4>Daniel Karrasch Website</h4>
                <div class="inProgress">{@html inProgress}</div>
            </div>
        </div>
    </li>
    <li class="tilt d">
        <div class="border">
            <a rel=external href="projekte/kamerabasiertes-besuchertracking-im-museum" class="link">
                <div class="title">
                    <h4>Kamerabasiertes Besuchertracking im Museum</h4>
                    <h5>Theia Tracking</h5>
                </div>
                <div class="done">{@html done} <span class="mono">{@html arrowRight}</span></div>
            </a>
        </div>
    </li>
    <li class="tilt e">
        <div class="border">
            <a rel=external href="projekte/datenbank-fuer-ein-finanzprogramm" class="link">
                <div class="title">
                    <h4>Datenbank für ein Finanzprogramm</h4>
                    <h5>Dedun</h5>
                </div>
                <div class="done">{@html done} <span class="mono">{@html arrowRight}</span></div>
            </a>
        </div>
    </li>
    <li class="">
<!--        <a rel=external href="projekte/robert-kraus-website" class="link">-->
        <div class="border">
            <div class="link">
                <h4>Robert Kraus Website</h4>
                <div class="done">{@html done}</div>
            </div>
        </div>
    </li>
    <li class="tilt g">
        <div class="border">
            <a rel=external href="projekte/3d-schiffe-versenken" class="link">
                <div class="title">
                    <h4>3D Schiffe versenken</h4>
                    <h5>Montgolfière</h5>
                </div>
                <div class="done">{@html done} <span class="mono">{@html arrowRight}</span></div>
            </a>
        </div>
    </li>
</ul>
