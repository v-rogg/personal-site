<script lang="ts">
	import { appState } from "$lib/stores.svelte";
	let { data } = $props();

	if (data.metadata?.slug) {
		appState.metadata = data.metadata;
	}
</script>

<svelte:head>
	<title>VR • {data.metadata.title}</title>
	<meta name="description" content={data.metadata.preview} />
</svelte:head>

<data.content />
