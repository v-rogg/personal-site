<script>
    import Navigation from "../../components/Navigation.svelte";
    export let segment;
</script>

<style lang="sass">
    section
        margin-top: 5rem
</style>

<section class="container">
    <slot></slot>
</section>

<Navigation ref={segment === undefined ? '/' : '/projekte'} />
