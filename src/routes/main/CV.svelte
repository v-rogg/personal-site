<script context="module" lang="ts">
    const arrowRight: string = '<span class="mono">-></span>';
</script>

<style lang="scss">
    @import "../../styles/theme";

    section {
        grid-area: 1 / 1 / 3 / 2;
    }

    @media (max-width: 991.98px) {
        section {
            grid-area: 2 / 1 / 3 / 2;
        }
    }

    ul {
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .experience {
        margin: 2rem 0;
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

    //.a { &:before { content:'~>'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    //.b { &:before { content:'=>'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    //.c { &:before { content:'>]'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    .d { &:before { content:'>='; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    .e { &:before { content:'|>'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    .f { &:before { content:'>:'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}
    //.g { &:before { content:'<>'; } &:hover { a { color: $skin; } &:before { color: $lightskin; }}}

    .time {
        font-family: $font-family-sans;
        color: $lightskin;
    }

</style>

<section>
    <h3>Erfahrungen</h3>

    <ul>
        <li class="experience c">
            <div class="time">10/2017 {@html arrowRight} vsl. 02/2021</div>
            <h4><a href="https://www.hs-augsburg.de/" target="_blank">Hochschule Augsburg </a></h4>
            <p class="description"><a href="https://www.hs-augsburg.de/Gestaltung/Interactive-Media.html" target="_blank">Interaktive Medien B.A.</a></p>
        </li>
        <li class="experience b">
            <div class="time">2013 {@html arrowRight} HEUTE</div>
            <h4><a href="projekte" rel="external">Freiberuflich</a></h4>
            <p class="description">Webdesign, Webentwicklung, Fotografie</p>
        </li>
        <li class="experience d">
            <div class="time">09/2019 {@html arrowRight} 02/2020</div>
            <h4><a href="https://planstack.de/" target="_blank">Planstack</a></h4>
            <p class="description">Praktikant Full-Stack Webentwicklung</p>
        </li>
        <li class="experience f">
            <div class="time">10/2017 {@html arrowRight} 09/2019</div>
            <h4><a href="https://starkstrom-augsburg.de/" target="_blank">Starkstrom Augsburg e.V.</a></h4>
            <p class="description">Führende Position Marketing</p>
        </li>
        <li class="experience e">
            <div class="time">10/2018 {@html arrowRight} 07/2019</div>
            <h4><a href="https://www.nfq.de/" target="_blank">.NFQ Digital Creatives</a></h4>
            <p class="description">Werkstudent</p>
        </li>
        <li class="experience a">
            <div class="time">04/2011 {@html arrowRight} 05/2011</div>
            <h4><a href="https://www.mvv.de/de/ueber-uns/unternehmensgruppe/mvv-enamic/mvv-industriepark-gersthofen/industriepark-gersthofen/bilfinger-maintenance-gmbh/" target="_blank">Bilfinger Maintenance GmbH</a></h4>
            <p class="description">Betriebspraktikum</p>
        </li>
    </ul>
</section>
