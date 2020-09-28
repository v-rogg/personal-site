<script>
    const arrowLeft = '<span class="mono"><-</span>';
    export let ref;
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
        justify-content: space-between;
        align-items: flex-end;
    }
</style>

<nav class="container">
    <a rel=external href={ref} class="back">{@html arrowLeft} Zurück</a>
</nav>
