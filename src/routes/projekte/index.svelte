<script context="module">
    const arrowRight = '<span class="mono">-></span>';
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
            padding-left: 1rem;
            border-left: 1px solid black;
            height: 5rem;
            position: relative;
            //transition: border-left-color linear 50ms;

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

        a {
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: space-between;
            align-items: start;
            //text-align: left;
            transition: box-shadow ease 250ms;
            flex-direction: column;
            flex-grow: 1;
            margin-right: 2rem;


            &:hover {
                //border: 2px solid black;
                //transform: translateY(-2px) scale(1.1) rotateX(10deg);
                //box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
                //box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            }
        }
    }

    li:nth-child(3n - 2) {
        margin: 2rem 3rem 2rem 0;
    }

    li:nth-child(3n - 1) {
        margin: 2rem 1.5rem;
    }

    li:nth-child(3n) {
        margin: 2rem 0 2rem 3rem;
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

    .inProgress {
        color: $lightskin;
    }

    //.done {
    //    color: $green;
    //}

    .inProgress, .done {
        display: flex;
        width: 100%;
        justify-content: space-between;
    }

    .darkyellow { &:before { content:'>='; } &:hover { *, &:before { color: $darkyellow; } border-color: $darkyellow; }}
    .blue { &:before { content:'>]'; } &:hover { *, &:before { color: $blue; } border-color: $blue; }}
    .purple { &:before { content:'|>'; } &:hover { *, &:before { color: $purple; } border-color: $purple; }}
    .green { &:before { content:'=>'; } &:hover { *, &:before { color: $green; } border-color: $green; }}
    .magenta { &:before { content:'>:'; } &:hover { *, &:before { color: $magenta; } border-color: $magenta; }}
    .anotherblue { &:before { content:'<>'; } &:hover { *, &:before { color: $anotherblue; } border-color: $anotherblue; }}
    .anothergreen { &:before { content:'<>'; } &:hover { *, &:before { color: $anothergreen; } border-color: $anothergreen; }}
    .red { &:before { content:'~>'; } &:hover { *, &:before { color: $red; } border-color: $red; }}
</style>

<svelte:head>
    <title>Valentin Rogg - Projekte</title>
</svelte:head>


<h3>Ein Auszug meiner bisherigen Projekte</h3>

<ul>
    <li class="tilt blue">
        <a rel=external href="projekte/anonymes-personentracking">
            <h4>Anonymes Personentracking</h4>
            <div class="inProgress">{@html inProgress} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt green">
        <a rel=external href="projekte/darterkennung-mit-lidar">
            <h4>Dart-Erkennung mithilfe eines LiDAR-Sensors</h4>
            <div class="inProgress">{@html inProgress} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt red">
        <a rel=external href="projekte/daniel-karrasch-website">
            <h4>Daniel Karrasch Website</h4>
            <div class="inProgress">{@html inProgress} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt magenta">
        <a rel=external href="projekte/kamerabasiertes-besuchertracking-im-museum">
            <h4>Kamerabasiertes Besuchertracking im Museum</h4>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt anotherblue">
        <a rel=external href="projekte/datenbank-fuer-ein-finanzprogramm">
            <h4>Datenbank für ein Finanzprogramm</h4>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt darkyellow">
        <a rel=external href="projekte/robert-kraus-website">
            <h4>Robert Kraus Website</h4>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt purple">
        <a rel=external href="projekte/montgolfiere">
            <h4>Montgolière</h4>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
</ul>
