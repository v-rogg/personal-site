<script lang="ts">
	let { children } = $props();
	import { appState } from "$lib/stores.svelte";

	appState.metadata = undefined;
</script>

<main class="container relative mx-auto flex flex-col gap-4">
	{@render children()}
</main>
