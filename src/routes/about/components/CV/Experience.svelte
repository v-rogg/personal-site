<script>
    import { lang } from '../../../../stores';
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
    <h2>
        <b>
        {#if $lang === "de" }
            Erfahrungen
        {/if}
        {#if $lang === "en" }
            Experience
        {/if}
        </b>
    </h2>

    <div class="experience">
        <div class="time">
            10/2017 <span class="mono">-></span>
            {#if $lang === "de"}
                vsl. 02/2021
            {/if}
            {#if $lang === "en"}
                planned 02/2021
            {/if}
        </div>
        <h4>
            <b>
                {#if $lang === "de"}
                    Hochschule Augsburg
                {/if}
                {#if $lang === "en"}
                    University of Applied Science Augsburg
                {/if}
                <div>
                    {#if $lang === "de"}
                        Interaktive Medien B.Sc.
                    {/if}
                    {#if $lang === "en"}
                        Interactive Media B.Sc.
                    {/if}
                </div>
            </b>
        </h4>
    </div>

    <div class="experience">
        <div class="time">
            2016 <span class="mono">-></span>
            {#if $lang === "de"}
                HEUTE
            {/if}
            {#if $lang === "en"}
                TODAY
            {/if}
        </div>
        <h4>
            <b>
                {#if $lang === "de"}
                    Selbstständig
                {/if}
                {#if $lang === "en"}
                    Freelance
                {/if}
            </b>
        </h4>
    </div>

    <div class="experience">
        <div class="time">
            09/2019 <span class="mono">-></span> 02/2020
        </div>
        <h4><b>Planstack</b></h4>
        <div>
            {#if $lang === "de"}
                Praktikant Full-Stack Webentwicklung
            {/if}
            {#if $lang === "en"}
                Inter Full-Stack Web development
            {/if}
        </div>
    </div>

    <div class="experience">
        <div class="time">
            10/2017 <span class="mono">-></span> 09/2019
        </div>
        <h4><b>Starkstrom Augsburg e.V.</b></h4>
        <div>
            {#if $lang === "de"}
                Führende Position Marketing
            {/if}
            {#if $lang === "en"}
                Lead Marketing and Public Relations
            {/if}
        </div>
    </div>

    <div class="experience">
        <div class="time">
            10/2018 <span class="mono">-></span> 07/2019
        </div>
        <h4><b>.NFQ Digital Creatives</b></h4>
        <div>
            {#if $lang === "de"}
                Werkstudent
            {/if}
            {#if $lang === "en"}
                Working student
            {/if}
        </div>
    </div>
</div>
