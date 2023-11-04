<script lang="ts">
	import { storyblokEditable } from "@storyblok/svelte";
	import type { MemoryContent } from "$lib/storyblok";
	import Tag from "$lib/components/globals/Tag/Tag.svelte";
	export let blok: MemoryContent;
</script>

<div
	use:storyblokEditable="{blok}"
	class="grid-settings sm:rounded-2xl overflow-hidden relative h-[25rem]"
	style="--sm: {blok.grid_sm}; --md: {blok.grid_md}; --lg: {blok.grid_lg}; --xl: {blok.grid_xl};">
	{#if blok.img}
		<img
			class="object-cover h-[100%] w-[100%] hover:scale-y-105 transition-transform duration-1000"
			src="{blok.img.filename}/75"
			alt="{blok.img.alt}"
			class:-scale-x-100="{blok.flip}"
			class:hover:-scale-x-105="{blok.flip}"
			class:hover:scale-x-105="{!blok.flip}" />
	{/if}
	<div
		class="absolute z-10 pointer-events-none bottom-8 right-8 text-white-500 backdrop-blur-sm px-2 py-1 rounded-xl border border-[rgba(255,255,255,0.3)] flex gap-2 items-center">
		<div>{blok.title}</div>
		{#if blok.flag}
			{@const flag = blok.flag}
			<div class="relative">
				<img class="h-[1.2em]" src="{flag.filename}" alt="{flag.alt}" />
				<!--				<div class="absolute -bottom-2 -right-1 text-md font-handwriting text-white-500">{new Date(blok.date).getFullYear().toString().substring(2)}</div>-->
			</div>
		{/if}
	</div>
	<div class="absolute left-8 top-8">
		<Tag color="{blok.tag_color.color}" seperateDarkmode="{false}">
			Trip
		</Tag>
	</div>
</div>

<style lang="postcss">
	.grid-settings {
		@media (min-width: theme("screens.sm")) {
			grid-column: span var(--sm) / span var(--sm);
		}
		@media (min-width: theme("screens.md")) {
			grid-column: span var(--md) / span var(--md);
		}
		@media (min-width: theme("screens.lg")) {
			grid-column: span var(--lg) / span var(--lg);
		}
		@media (min-width: theme("screens.xl")) {
			grid-column: span var(--xl) / span var(--xl);
		}
	}
</style>
