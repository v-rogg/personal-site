<script context="module">
    const arrowRight = '<span class="mono">-></span>';
    const arrowLeft = '<span class="mono"><|</span>';
</script>

<script>
    let hover = false;
</script>

<style lang="scss">
    @import "./src/styles/theme";

    ul {
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .experience {
        margin: 1.25rem 0;
        position: relative;
        width: max-content;

        &:before {
            position: absolute;
            //content: '|>';
            //content: '~>';
            content: '>]';
            left: -2rem;
            font-family: 'JetBrains Mono', sans-serif;
            top: 50%;
            line-height: 0;
            opacity: 0;
            transition: opacity linear 100ms;
        }

        //&:after {
        //    position: absolute;
        //    content: '';
        //    left: 0;
        //    top: 0;
        //    width: 100%;
        //    height: 100%;
        //    transition: all linear 100ms;
        //}

        &:hover {
            //color: $red;

            &:before {
                //color: $green;
                opacity: 1;
            }

            //&:after {
            //    //background: transparentize($red, .75);
            //    left: -.75rem;
            //    top: -.75rem;
            //    width: calc(100% + 1.5rem);
            //    height: calc(100% + 1.5rem);
            //    z-index: -1;
            //}
        }
    }

    .red { &:hover { color: $red; &:before { background: transparentize($red, .75); }}}
    .darkyellow { &:hover { color: $darkyellow; &:before { background: transparentize($darkyellow, .75); }}}
    .purple { &:hover { color: $purple; &:before { background: transparentize($purple, .75); }}}
    .green { &:hover { color: $green; &:before { background: transparentize($green, .75); }}}
    .blue { &:hover { color: $blue; &:before { background: transparentize($blue, .75); }}}
    .magenta { &:hover { color: $magenta; &:before { background: transparentize($magenta, .75); }}}

    section {
        width: 50%
    }

    .time {
        font-family: $font-family-sans;
    }

    .description {
        font-family: $font-family-serif;
        //transform: translateZ(-500px);
    }

</style>

<section>
    <h3>Erfahrungen</h3>

    <ul>
        <li class="experience magenta">
            <div class="time">10/2017 {@html arrowRight} vsl. 02/2021</div>
            <h4><a href="https://www.hs-augsburg.de/" target="_blank">Hochschule Augsburg</a></h4>
            <div class="description"><a href="https://www.hs-augsburg.de/Gestaltung/Interactive-Media/IA.html" target="_blank">Interaktive Medien B.A.</a></div>
        </li>
        <li class="experience green">
            <div class="time">2012 {@html arrowRight} HEUTE</div>
            <h4>Freiberuflich</h4>
            <div class="description">Webdesign, Webentwicklung, Fotografie</div>
        </li>
        <li class="experience blue">
            <div class="time">09/2019 {@html arrowRight} 02/2020</div>
            <h4><a href="https://planstack.de/" target="_blank">Planstack</a></h4>
            <div class="description">Praktikant Full-Stack Webentwicklung</div>
        </li>
        <li class="experience purple">
            <div class="time">10/2017 {@html arrowRight} 09/2019</div>
            <h4><a href="https://starkstrom-augsburg.de/" target="_blank">Starkstrom Augsburg e.V.</a></h4>
            <div class="description">Führende Position Marketing</div>
        </li>
        <li class="experience darkyellow">
            <div class="time">10/2018 {@html arrowRight} 07/2019</div>
            <h4><a href="https://www.nfq.de/#" target="_blank">.NFQ Digital Creatives</a></h4>
            <div class="description">Werkstudent</div>
        </li>
    </ul>
</section>
