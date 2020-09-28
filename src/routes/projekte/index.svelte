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
            height: 6rem;
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

    .inProgress, .done {
        display: flex;
        width: 100%;
        justify-content: space-between;
        justify-self: flex-end;
        color: $lightskin;
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
<!--        <a rel=external href="projekte/anonymes-personentracking" class="link">-->
        <div class="link">
            <h4>Anonymes Personentracking</h4>
            <div class="inProgress">{@html inProgress}</div>
        </div>
    </li>
    <li class="tilt green">
<!--        <a rel=external href="projekte/darterkennung-mit-lidar">-->
        <div class="link">
            <h4>Dart-Erkennung mithilfe eines LiDAR-Sensors</h4>
            <div class="inProgress">{@html inProgress}</div>
        </div>
    </li>
    <li class="tilt red">
<!--        <a rel=external href="projekte/daniel-karrasch-website" class="link">-->
        <div class="link">
            <h4>Daniel Karrasch Website</h4>
            <div class="inProgress">{@html inProgress}</div>
        </div>
    </li>
    <li class="tilt magenta">
<!--        <a rel=external href="projekte/kamerabasiertes-besuchertracking-im-museum" class="link">-->
        <div class="link">
            <div class="title">
                <h4>Kamerabasiertes Besuchertracking im Museum</h4>
                <h5>Theia Tracking</h5>
            </div>
            <div class="done">{@html done}</div>
        </div>
    </li>
    <li class="tilt anotherblue">
        <a rel=external href="projekte/datenbank-fuer-ein-finanzprogramm" class="link">
            <div class="title">
                <h4>Datenbank für ein Finanzprogramm</h4>
                <h5>Dedun</h5>
            </div>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
    <li class="tilt darkyellow">
<!--        <a rel=external href="projekte/robert-kraus-website" class="link">-->
        <div class="link">
            <h4>Robert Kraus Website</h4>
            <div class="done">{@html done}</div>
        </div>
    </li>
    <li class="tilt purple">
        <a rel=external href="projekte/3d-schiffe-versenken" class="link">
            <div class="title">
                <h4>3D Schiffe versenken</h4>
                <h5>Montgolfière</h5>
            </div>
            <div class="done">{@html done} {@html arrowRight}</div>
        </a>
    </li>
</ul>
