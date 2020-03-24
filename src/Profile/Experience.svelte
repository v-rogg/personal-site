<script>
    export let currentRoute;
</script>

<style>
    .experience {
        margin-top: 1.25rem;
    }
    .time {

    }
    .w50 {
        width: 50%
    }
</style>

<div class="w50">
    {#if currentRoute.language === "de" }
        <h2><b>Erfahrungen</b></h2>
    {/if}

    {#if currentRoute.language === "en" }
        <h2><b>Experience</b></h2>
    {/if}

    <div class="experience">
        {#if currentRoute.language === "de"}
            <div class="time">10/2017 <span class="mono">-></span> vsl. 02/2021</div>
        {/if}
        {#if currentRoute.language === "en"}
            <div class="time">10/2017 <span class="mono">-></span> planed 02/2021</div>
        {/if}
        {#if currentRoute.language === "de"}
            <h4><b>Hochschule Augsburg</b></h4>
            <div>Interaktive Medien B.Sc.</div>
        {/if}
        {#if currentRoute.language === "en"}
            <h4><b>University of Applied Science Augsburg</b></h4>
            <div>Interactive Media B.Sc.</div>
        {/if}
    </div>

    <div class="experience">
        {#if currentRoute.language === "de"}
            <div class="time">2016 <span class="mono">-></span> HEUTE</div>
        {/if}
        {#if currentRoute.language === "en"}
            <div class="time">2016 <span class="mono">-></span> TODAY</div>
        {/if}
        {#if currentRoute.language === "de"}
            <h4><b>Selbstständig</b></h4>
        {/if}
        {#if currentRoute.language === "en"}
            <h4><b>Freelance</b></h4>
        {/if}
    </div>

    <div class="experience">
        <div class="time">09/2019 <span class="mono">-></span> 02/2020</div>
        <h4><b>Planstack</b></h4>
        {#if currentRoute.language === "de"}
            <div>Praktikant Full-Stack Webentwicklung</div>
        {/if}
        {#if currentRoute.language === "en"}
            <div>Inter Full-Stack Web development</div>
        {/if}
    </div>

    <div class="experience">
        <div class="time">10/2017 <span class="mono">-></span> 09/2019</div>
        <h4><b>Starkstrom Augsburg e.V.</b></h4>
        {#if currentRoute.language === "de"}
            <div>Führende Position Marketing</div>
        {/if}
        {#if currentRoute.language === "en"}
            <div>Lead Marketing and Public Relations</div>
        {/if}
    </div>

    <div class="experience">
        <div class="time">10/2018 <span class="mono">-></span> 07/2019</div>
        <h4><b>.NFQ Digital Creatives</b></h4>
        {#if currentRoute.language === "de"}
            <div>Werkstudent</div>
        {/if}
        {#if currentRoute.language === "en"}
            <div>Working student</div>
        {/if}
    </div>
</div>