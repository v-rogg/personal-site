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

    <div class="experience">
        {#if $lang === "de" }
            <h4><b>Sprachen</b></h4>
            <p>Deutsch Muttersprache, Englisch C2, Französisch B2, Spanisch A1</p>
        {/if}
        {#if $lang === "en" }
            <h4><b>Languages</b></h4>
            <p>German native, English C2, French B2, Spanish A1</p>
        {/if}
    </div>
</div>
