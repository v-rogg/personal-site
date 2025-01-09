<script lang="ts">
	import Tag from "./Tag.svelte";

	let { tags, cls = "" } = $props();
</script>

<ul class="m-0 my-4 flex w-max max-w-full flex-wrap gap-2 rounded-md p-0 text-black {cls}">
	{#each tags.sort() as tag}
		<Tag>{tag}</Tag>
	{/each}
</ul>
