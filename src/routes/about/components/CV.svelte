<script>
    import Skills from './CV/Skills.svelte';
    import Experience from "./CV/Experience.svelte";
</script>

<style>
    .flex {
        display: flex;
        justify-content: space-between;
    }

    @media (max-width: 991.98px) {
        .flex {
            flex-direction: column-reverse;
        }
    }
</style>


<section class="flex container">
    <Experience />
    <Skills />
</section>
