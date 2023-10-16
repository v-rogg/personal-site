<script lang="ts">
	import { storyblokEditable } from "@storyblok/svelte";
	import type { MemoryContent } from "$lib/storyblok/schema";
	export let blok: MemoryContent;
</script>

<div
	use:storyblokEditable="{blok}"
	class="rounded-xl overflow-hidden relative h-[25rem]"
	style="grid-column: span {blok.col_span} / span {blok.col_span}; grid-row: span {blok.row_span} / span {blok.row_span};">
	{#if blok.img}
		<img class="object-cover h-[100%] w-[100%] hover:scale-y-105 transition-transform duration-1000" src="{blok.img.filename}/75" alt="{blok.img.alt}" class:-scale-x-100={blok.flip} class:hover:-scale-x-105={blok.flip} class:hover:scale-x-105={!blok.flip}/>
	{/if}
	<div class="absolute z-10 pointer-events-none bottom-4 right-4 text-white-500 text-lg backdrop-blur-sm px-2 py-1 rounded-xl border border-[rgba(255,255,255,0.3)] flex gap-2 items-center">
		<div>{blok.title}</div>
		{#if blok.flag}
			{@const flag = blok.flag}
			<div class="relative">
				<img class="h-[1.2em]" src="{flag.filename}" alt="{flag.alt}"/>
<!--				<div class="absolute -bottom-2 -right-1 text-md font-handwriting text-white-500">{new Date(blok.date).getFullYear().toString().substring(2)}</div>-->
			</div>
		{/if}
	</div>
</div>

