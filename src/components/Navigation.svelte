<script>
    const arrowLeft = '<span class="mono"><-</span>';
    export let ref;
    export let stage;
</script>

<style lang="scss">
    @import './src/styles/theme';

    .back {
        width: max-content;

        &:hover {
            font-weight: 700;
            color: $skin;
        }
    }

    nav {
        color: $lightskin;
        margin-top: 5rem;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
    }

    .bar {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        width: 100%;
        position: relative;
        margin-bottom: 1rem;
        gap: 4px;

        .top, .mid, .bottom {
            height: 3px;
            width: 100%;
            background: transparentize($lightskin, .9);

        }

        .top:hover, .mid:hover {
            background: $skin;
        }
    }

    .show {
        background: $skin !important;
    }
</style>

<nav class="container">
    <div class="bar">
        <a rel="external" href="/" class="top"></a>
        <a rel="external" href="/projekte" class="mid" class:show={stage === undefined}></a>
        <div class="bottom" class:show={stage !== undefined }></div>
    </div>
    <a rel=external href={ref} class="back">{@html arrowLeft} Zurück</a>
</nav>
