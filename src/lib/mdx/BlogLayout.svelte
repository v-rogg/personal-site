<script lang="ts">
	import TagList from "$lib/components/globals/TagList.svelte";

	let { title, subtitle, tags, date, children } = $props();
</script>

<div class="-mx-10 rounded-xl bg-white-600 px-10 py-10">
	<h1>{title}</h1>

	<h2 class="text-wrap">{subtitle}</h2>

	<TagList tags={tags} />

	<div class="mt-2 text-sm">
		<span class="mr-2">Veröffentlichung:</span>
		{new Date(date).toLocaleDateString("de-DE", { day: "numeric", month: "2-digit", year: "numeric" })}
	</div>
	<!--
<img src={previewImageUrl} alt="" class="relative -z-10 -mb-4 -mt-16 ml-auto max-h-48 opacity-30" />

<hr /> -->
</div>

<div class="mt-10">
	{@render children()}
</div>
