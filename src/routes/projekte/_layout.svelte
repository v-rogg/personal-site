<script>
    import Navigation from "../../components/Navigation.svelte";
    export let segment;
</script>

<style lang="sass" global>
    @import './src/styles/theme'

    section
        margin-top: 5rem !important

    .pt
        margin: 1.25rem 0

    p
        margin: 1.25rem 0

    .asset
        margin: 3rem 5rem 0
        display: block

    .caption
        margin: 0 5rem 3rem 0
        text-align: right
        font-size: $font-size-small

    @media (max-width: 991.98px)
        .caption
            text-align: center
            margin: 0 0 3rem

</style>

<Navigation ref={segment === undefined ? '/' : '/projekte'} stage={segment} />

<section class="container">
    <slot></slot>
</section>

