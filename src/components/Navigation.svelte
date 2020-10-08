<script>
    const arrowLeft = '<span class="mono"><-</span>';
    export let ref;
    export let stage;
</script>

<style lang="scss">
    @import './src/styles/theme';

    .back {

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
        background: transparentize($lightskin, .9);
        margin-bottom: 1rem;

        .top, .mid, .bottom {
            opacity: 0;
            height: 4px;
            background: $skin;
        }
    }

    .show {
        opacity: 1 !important;
    }
</style>

<nav class="container">
    <div class="bar">
        <div class="top" ></div>
        <div class="mid" class:show={stage === undefined}></div>
        <div class="bottom" class:show={stage !== undefined }></div>
    </div>
    <a rel=external href={ref} class="back">{@html arrowLeft} Zurück</a>
</nav>
