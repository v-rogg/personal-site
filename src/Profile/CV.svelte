<script>
    import Skills from './Skills.svelte';
    import {Route} from 'svelte-router-spa';
    import Experience from "./Experience.svelte";

    export let currentRoute;
</script>

<section class="container flex">
    <Experience {currentRoute}/>
    <Skills {currentRoute}/>
</section>

<style>
    .flex {
        display: flex;
        justify-content: space-between;
    }
    .w50 {
        width: 50%
    }
</style>