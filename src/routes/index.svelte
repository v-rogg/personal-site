<script>
    import CV from "./main/CV.svelte";
    import Skills from "./main/Skills.svelte";
    import Projects from "./main/Projects.svelte";
</script>

<style lang="scss">
    .split {
        display: grid;
        grid-template-columns: 40% 60%;

        .left {
            width: 40%;
        }

        .right {
            width: 60%;
        }
    }

    @media (max-width: 991.98px) {
        .split {
            grid-template-columns: 100%;
        }
    }
</style>

<div class="split container">
    <CV />
    <Projects />
    <Skills />
</div>

<div class="container">
    <div class="calendar">

    </div>
</div>
