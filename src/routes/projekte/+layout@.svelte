<script lang="ts">
	import "../../app.css";
	import "@awesome.me/kit-fb271958c9/icons/css/all.min.css";
	let { children } = $props();
</script>

<main class="container mx-auto mt-32 px-10">
	{@render children()}
</main>
