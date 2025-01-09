<script lang="ts">
	import TagList from "$lib/components/globals/TagList.svelte";

	let { title, subtitle, tags, date, children } = $props();
</script>

<h1>{title}</h1>

<h2>{subtitle}</h2>

<TagList tags={tags} />

<div class="mt-2 text-sm">
	<span class="mr-2">Veröffentlichung:</span>
	{new Date(date).toLocaleDateString("de")}
</div>

<hr />

<div class="mt-10">
	{@render children()}
</div>
