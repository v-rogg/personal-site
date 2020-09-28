<style lang="sass">
    @import "./src/styles/theme"

    picture
        margin: 3rem 0 0

    .mockup
        margin: -2rem 0 0
        z-index: -1
        position: relative

    .side-by-side
        display: grid
        grid-template-columns: repeat(4, 1fr)
        margin: 0 0 2rem
        overflow: hidden

        picture
            margin-top: -3rem

    @media (max-width: 991.98px)
        .side-by-side
            grid-template-columns: 1fr 1fr

            picture
                margin-top: 0

    @media (max-width: 576px)
        .side-by-side
            grid-template-columns: 1fr

</style>

<div class="pt">
    <h3>
        3D Schiffe versenken
    </h3>
    <h4>
        Montgolfière
    </h4>
</div>

<p>
    Bei dem Projekt Montgolfière handelt es sich um ein Konzept und Prototypen für ein webbasiertes Spiel.
</p>

<picture class="mockup">
    <source type="image/webp" srcset="assets/projects/3d-schiffe-versenken/montgolfiere_moc.webp">
    <img src="assets/projects/3d-schiffe-versenken/montgolfiere_moc.png" alt="Montgolfière Mockup">
</picture>
<div class="caption">
    Montgolfière Mockup
</div>

<p>
    Das grundsätzliche Spielprinzip basiert auf dem allgemein bekannten Spiel „Schiffe versenken“.<br/>
    Es gibt zwei Spielfelder. Das Eigene und das des Gegners. Am Anfang jeder Runde platziert der Spieler seine Schiffe auf seinem Spielfeld. Sobald alle Schiffe platziert wurden, beginnt das eigentliche Spiel. Immer abwechselnd dürfen die Spieler auf das gegnerische Spielfeld schießen. Ob es sich bei dem Schuss um einen Treffer handelt oder nicht, wird mit farblich unterschiedlichen Markern gemerkt. Ziel des Spiels ist es alle Schiffe des Gegners zu versenken. Wem dies zuerst gelingt, gewinnt das Spiel.<br/>
    Um dem Spiel einen neuen Spin zu verleihen und interessanter zu gestalten, wird das Spielfeld von 10x10 auf 6x6 verkleinert. Es werden jedoch zwei neue Ebenen hinzugefügt. Nach dieser Veränderung hat das Spielfeld nur geringfügig mehr Felder, wodurch eine ähnlich lange Spieldauer gewährleistet wird. Durch die Unterteilung auf drei Spielebenen entstehen jedoch neue Möglichkeiten für die visuelle Umsetzung, das Thema sowie für die Spielmechaniken.<br/>
    Inspiriert von dem Film „Die drei Musketiere“ entstand die Idee, die klassischen Schiffe durch Luftschiffe zu ersetzen.
</p>



<div class="side-by-side">

    <picture>
        <source type="image/webp" srcset="assets/projects/3d-schiffe-versenken/c2.webp">
        <img src="assets/projects/3d-schiffe-versenken/c2.png" alt="Landschaftskonzept">
    </picture>
    <picture>
        <source type="image/webp" srcset="assets/projects/3d-schiffe-versenken/land1.webp">
        <img src="assets/projects/3d-schiffe-versenken/land1.png" alt="Landschaftskonzept">
    </picture>
    <picture>
        <source type="image/webp" srcset="assets/projects/3d-schiffe-versenken/land2.webp">
        <img src="assets/projects/3d-schiffe-versenken/land2.png" alt="Landschaftskonzept">
    </picture>
    <picture>
        <source type="image/webp" srcset="assets/projects/3d-schiffe-versenken/land3.webp">
        <img src="assets/projects/3d-schiffe-versenken/land3.png" alt="Landschaftskonzept">
    </picture>
</div>
<div class="caption">
    Landschaftskonzepte
</div>
