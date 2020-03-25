<script>
    import Skills from './Skills.svelte';
    import Experience from "./Experience.svelte";
</script>

<style>
    .flex {
        display: flex;
        justify-content: space-between;
    }
</style>


<section class="flex">
    <Experience />
    <Skills />
</section>