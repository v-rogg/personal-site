<script lang="ts">
    const arrowLeft: string = '<span class="mono"><-</span>';
    export let ref;
    export let stage;
</script>

<style lang="scss" global>
    @import './src/styles/theme';

    .back {
        width: max-content;

        &:hover {
            font-weight: 700;
            color: $skin;
        }
    }

    nav {
        color: $darkskin;
        margin-top: 5rem !important;
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
            padding-top: 5px;
            padding-bottom: 5px;
            //height: 13px;
            width: 100%;
            position: relative;

            .color {
                height: 3px;
                width: 100%;
                display: block;
                position: relative;
                background: transparentize($darkskin, .9);
            }

        }

        .top:hover, .mid:hover {
            .color {
                background: $skin;
            }
        }
    }

    .show {
        .color {
            background: $skin !important;
        }
    }
</style>

<nav class="container">
    <div class="bar">
        <a rel="external" href="/" class="top">
            <div class="color"></div>
        </a>
        <a rel="external preload" href="/projekte" class="mid" class:show={stage === undefined}>
            <div class="color"></div>
        </a>
        <div class="bottom" class:show={stage !== undefined}>
            <div class="color"></div>
        </div>
    </div>
    <a rel="external" href={ref} class="back">{@html arrowLeft} Zurück</a>
</nav>
