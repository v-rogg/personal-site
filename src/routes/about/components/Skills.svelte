<script>
    import { lang } from '../../../stores';
</script>

<style>
    .experience {
        margin-top: 1.25rem;
    }
    .w50 {
        width: 50%
    }
</style>

<div class="w50">
    {#if $lang === "de" }
        <h2><b>Fähigkeiten</b></h2>
    {/if}
    {#if $lang === "en" }
        <h2><b>Skills</b></h2>
    {/if}
    {#if $lang === "de" }
    <div class="experience">
        <h4><b>Sprachen</b></h4>
        <p>Deutsch C2, Englisch C1, Französisch B2, Spanisch A1</p>
    </div>
    {/if}
    {#if $lang === "en" }
    <div class="experience">
        <h4><b>Languages</b></h4>
        <p>German C2, English C1, French B2, Spanish A1</p>
    </div>

    {/if}

</div>