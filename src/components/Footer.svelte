<script>
    import { lang } from "../stores";

    const year = new Date().getFullYear();
</script>

<style lang="scss">
    section {
        margin-top: 10rem;
        display: flex;
        justify-content: space-between;
    }
    span {
        margin-right: .25em;
    }
</style>

<section class="container">
    <div>
        © {year} Valentin Rogg
    </div>
<!--    <div>-->
<!--        <a href="./impressum">Impressum</a>-->
<!--    </div>-->
    <div>
        <slot></slot>
    </div>
    <div>
        <span>v1.2</span> {#if $lang === 'en'}05/15/20{/if} {#if $lang === 'de'}15.05.20{/if}
    </div>
</section>
