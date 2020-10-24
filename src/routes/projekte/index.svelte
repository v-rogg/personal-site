<script context="module" lang="ts">
    const arrowRight: string = '->';
    const inProgress: string = 'In Arbeit';
    const done: string = 'Beendet';
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

        .content {
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: space-between;
            align-items: start;
            //text-align: left;
            flex-direction: column;
            flex-grow: 1;
            //margin-right: 2rem;
            //padding-left: 1rem;
            //border-left: 1px solid black;
            position: relative;
        }
    }

    .border {
        display: block;
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

    //.back {
    //    margin-top: 5rem;
    //    display: block;
    //    color: $darkskin;
    //
    //    &:hover {
    //        font-weight: 700;
    //        color: $skin;
    //    }
    //}

    h3 {
        margin: 0 0 1.25rem;
    }

    //.check {
    //    width: 2rem;
    //}

    .inProgress, .done {
        display: flex;
        width: 100%;
        justify-self: flex-end;
        justify-content: space-between;
        flex-direction: row-reverse;
        color: black;
    }

    //.darkyellow { &:before { content:'>='; } &:hover { *, &:before { color: $darkyellow; } border-color: $darkyellow; }}
    //.blue { &:before { content:'>]'; } &:hover { *, &:before { color: $blue; } border-color: $blue; }}
    //.purple { &:before { content:'|>'; } &:hover { *, &:before { color: $purple; } border-color: $purple; }}
    //.green { &:before { content:'=>'; } &:hover { *, &:before { color: $green; } border-color: $green; }}
    //.magenta { &:before { content:'>:'; } &:hover { *, &:before { color: $magenta; } border-color: $magenta; }}
    //.anotherblue { &:before { content:'<>'; } &:hover { *, &:before { color: $anotherblue; } border-color: $anotherblue; }}
    //.anothergreen { &:before { content:'<>'; } &:hover { *, &:before { color: $anothergreen; } border-color: $anothergreen; }}
    //.red { &:before { content:'~>'; } &:hover { *, &:before { color: $red; } border-color: $red; }}

    //.a { &:before { content:'~>'; } &:hover { *, &:before { color: $darkskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .a { &:before { content:'~>'; } .bg { &:after { background: linear-gradient(#C7C700 0%, transparent 66%);}} &:hover { border-color: #C7C700 !important; *, &:before { color: black; } h5 { color: #C7C700 } .mono { color: #C7C700 } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    .b { &:before { content:'=>'; } .bg { &:after { background: linear-gradient(#9BDB1E 0%, transparent 66%);}} &:hover { border-color: #9BDB1E !important; *, &:before { color: black; } h5 { color: #9BDB1E } .mono { color: #9BDB1E } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    .c { &:before { content:'>]'; } .bg { &:after { background: linear-gradient(#c0a97d 0%, transparent 66%);}} &:hover { border-color: #c0a97d !important; *, &:before { color: black; } h5 { color: #c0a97d } .mono { color: #c0a97d } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    //.b { &:before { content:'=>'; } &:hover { *, &:before { color: $darkskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    //.c { &:before { content:'>]'; } &:hover { *, &:before { color: $darkskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .d { &:before { content:'>='; } .bg { &:after { background: linear-gradient(#f56457 0%, transparent 66%);}} &:hover { border-color: #f56457 !important; *, &:before { color: black; } h5 { color: #f56457 } .mono { color: #f56457 } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    .e { &:before { content:'|>'; } .bg { &:after { background: linear-gradient(#0f3044 0%, transparent 66%);}} &:hover { border-color: #0f3044 !important; *, &:before { color: black; } h5 { color: #0f3044 } .mono { color: #0f3044 } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    .f { &:before { content:'>:'; } .bg { &:after { background: linear-gradient(#ffd200 0%, transparent 66%);}} &:hover { border-color: #ffd200 !important; *, &:before { color: black; } h5 { color: #ffd200 } .mono { color: #ffd200 } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}
    //.g { &:before { content:'<>'; } &:hover { *, &:before { color: $darkskin; } .mono { color: $skin } * { border-color: $skin !important; }}}
    .g { &:before { content:'<>'; } .bg { &:after { background: linear-gradient(#c8d36b 0%, transparent 66%);}} &:hover { border-color: #c8d36b !important; *, &:before { color: black; } h5 { color: #c8d36b } .mono { color: #c8d36b } * { border-color: black !important; } .bg {opacity: .25; transform: scale(1.3) translateY(0)}}}

    .tilt {

        position: relative;
        box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
        transition: transform ease 200ms, box-shadow ease 200ms;
        //padding: 1rem;
        border-radius: 2px;
        overflow: hidden;

        &:hover {
            //transform: scale(1.1) perspective(80em) rotateX(10deg);
            transform: scale(1.1);
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            //border: 1px solid;
        }
    }

    .bg {
        position: absolute;
        top: 0;
        z-index: -1;
        //margin-top: -4rem;
        height: 8rem;
        width: auto;
        display: flex;
        justify-content: center;
        //margin-top: calc(100% - 50% - 4rem);
        transform: scale(1.2);
        opacity: 0;
        transition: opacity .2s ease, transform .2s ease, filter .2s ease;
        //filter: blur(.1rem);

        &:after {
            content: '';
            position: absolute;
            width: 100%;
            height: 100%;
            background: linear-gradient(white, transparent);
            opacity: .5;
        }

        > img {

        }
    }

    .mono, h5 {
        transition: color .2s ease;
    }
</style>

<svelte:head>
    <title>Valentin Rogg - Projekte</title>
</svelte:head>


<h3>Ein Auszug meiner bisherigen Projekte</h3>

<ul>
    <li class="tilt a">
        <a rel="external preload" href="projekte/anonymes-personentracking" class="border">
            <div class="content">
                <div class="title">
                    <h4>Anonymes Personentracking</h4>
                    <h5>Obacht!</h5>
                </div>
                <div class="inProgress"><span class="mono">{@html arrowRight}</span> {@html inProgress}</div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/anonymes-personentracking/preview-small.jpg" alt="Obacht! Vorschaubild">
        </div>
    </li>
    <li class="tilt b">
        <a rel="external preload" href="projekte/darterkennung-mit-lidar" class="border">
            <div class="content">
                <div class="title">
                    <h4>Dart-Punkteerkennung mithilfe eines LiDAR-Sensors</h4>
                    <h5>Robin Hood</h5>
                </div>
                <div class="inProgress"><span class="mono">{@html arrowRight}</span> {@html inProgress}</div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/darterkennung-mit-lidar/preview-small.jpg" alt="Robin Hood Vorschaubild">
        </div>
    </li>
    <li class="tilt c">
        <a rel="external preload" href="projekte/daniel-karrasch-website" class="border">
            <div class="content">
                <div class="title">
                    <h4>Gesangsstudio Augsburg Website</h4>
                    <h5>Daniel Karrasch</h5>
                </div>
                <div class="inProgress"><span class="mono">{@html arrowRight}</span> {@html inProgress}</div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/daniel-karrasch/preview-small.jpg" alt="Daniel Karrasch Vorschaubild">
        </div>
    </li>
    <li class="tilt d">
        <a rel="external preload" href="projekte/kamerabasiertes-besuchertracking-im-museum" class="border">
            <div class="content">
                <div class="title">
                    <h4>Kamerabasiertes Besuchertracking im Museum</h4>
                    <h5>Theia Tracking</h5>
                </div>
                <div class="done"><span class="mono">{@html arrowRight}</span></div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/kamerabasiertes-besuchertracking-im-museum/preview-small.jpg" alt="Theia Tracking Vorschaubild">
        </div>
    </li>
    <li class="tilt e">
        <a rel="external preload" href="projekte/datenbank-fuer-ein-finanzprogramm" class="border">
            <div class="content">
                <div class="title">
                    <h4>Datenbank für ein Finanzprogramm</h4>
                    <h5>Dedun</h5>
                </div>
                <div class="done"><span class="mono">{@html arrowRight}</span></div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/datenbank-fuer-ein-finanzprogramm/preview-small.jpg" alt="Dedun Vorschaubild">
        </div>
    </li>
    <li class="tilt f">
        <a rel="external preload" href="projekte/robert-kraus-website" class="border">
            <div class="content">
                <div class="title">
                    <h4>Alleinunterhalter Website</h4>
                    <h5>Robert Kraus</h5>
                </div>
                <div class="done"><span class="mono">{@html arrowRight}</span></div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/robert-kraus/preview.jpg" alt="Robert Kraus Vorschaubild">
        </div>
    </li>
    <li class="tilt g">
        <a rel="external preload" href="projekte/3d-schiffe-versenken" class="border">
            <div class="content">
                <div class="title">
                    <h4>3D Schiffe versenken</h4>
                    <h5>Montgolfière</h5>
                </div>
                <div class="done"><span class="mono">{@html arrowRight}</span></div>
            </div>
        </a>
        <div class="bg">
            <img src="assets/projects/3d-schiffe-versenken/preview.jpg" alt="Montgolfière Vorschaubild">
        </div>
    </li>
</ul>
