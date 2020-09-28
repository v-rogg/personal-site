<script>
    const arrowLeft = '<span class="mono"><-</span>';
    export let ref;
</script>

<style lang="scss">
    @import './src/styles/theme';

    .back {
        margin-top: 5rem;
        display: block;
        color: $lightskin;

        &:hover {
            font-weight: 700;
            color: $skin;
        }
    }
</style>

<nav class="container">
    <a rel=external href={ref} class="back">{@html arrowLeft} Zurück</a>
</nav>
