<script>
    import { lang } from '../stores';
</script>

<style lang="scss">
    @import "../../static/styles/theme";

    video {
        width: 100%;
        z-index: -1;
    }

    section {
        height: 15rem;
        overflow: hidden;
        margin-bottom: 5rem;
    }
</style>

<section>
    <video autoplay preload="auto" src="https://www.hs-augsburg.de/homes/rogg/host/people.mp4" loop muted>
    </video>
</section>
