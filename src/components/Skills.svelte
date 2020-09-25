<script>
</script>

<style lang="scss">
    @import "./src/styles/theme";

    .experience {
        margin: 1.25rem 0;

        &:before {
            position: absolute;
            //content: '|>';
            //content: '~>';
            //content: '>]';
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

    div {
        font-family: $font-family-sans;
    }

    .description {
        font-family: $font-family-serif;
    }

    .red { &:before { content:'~>'; } &:hover { color: $red; &:before { background: transparentize($red, .75); }}}
    .green { &:before { content:'=>'; } &:hover { color: $green; &:before { background: transparentize($green, .75); }}}
    .blue { &:before { content:'>]'; } &:hover { color: $blue; &:before { background: transparentize($blue, .75); }}}
</style>

<section>
    <h3>Fähigkeiten</h3>

    <div class="experience red" data-tilt data-tilt-scale="1.1" data-tilt-speed="200" data-tilt-max="15">
        <h4>Sprachen</h4>
        <p class="description">Deutsch Muttersprache, Englisch C1, Französisch B1, Spanisch A1</p>
    </div>
    <div class="experience green" data-tilt data-tilt-scale="1.1" data-tilt-speed="200" data-tilt-max="15">
        <h4>Technisches Wissen</h4>
        <p class="description">
            Full-Stack Web Development (Frontend: Vue, Svelte, Angular ... / Backend: Nest, Express, Flask ...), UX/UI Design,
            Machine Learning, DevSecOps, 3D Visualisierung (WebGL), 3D Modellierung,
            Sound Design, Micro-Controller
        </p>
    </div>
    <div class="experience blue" data-tilt data-tilt-scale="1.1" data-tilt-speed="200" data-tilt-max="15">
        <h4>Soziale Kompetenzen</h4>
        <p class="description">Teamführung, Projektmanagement, Vertrieb</p>
    </div>
</section>
