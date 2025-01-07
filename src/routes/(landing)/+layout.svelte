<script lang="ts">
	let { children } = $props();
	import { page } from "$app/stores";
	import { appState } from "$lib/stores.svelte";

	appState.metadata = undefined;
</script>

<main class="container relative mx-auto flex flex-col gap-4" class:m-32={$page.url.pathname !== "/"}>
	{@render children()}
</main>
