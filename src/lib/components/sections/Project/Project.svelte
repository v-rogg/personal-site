<script lang="ts">
	import { StoryblokComponent, storyblokEditable } from "@storyblok/svelte";
	import Tag from "$lib/components/globals/Tag/Tag.svelte";

	export let blok;
</script>

<section
	use:storyblokEditable="{blok}"
	class="grid-settings flex gap-4 justify-between bg-white-600 dark:bg-blue-800 sm:rounded-2xl p-8 col-span max-sm:min-h-[25rem] md:h-[25rem] overflow-hidden relative"
	style="--sm: {blok.grid_sm}; --md: {blok.grid_md}; --lg: {blok.grid_lg}; --xl: {blok.grid_xl};">
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-2">
			<h3 class="flex items-center gap-2">
				{#if blok.icon}
					<img class="block h-7" src="{blok.icon.filename}" alt="logo"/>
				{/if}
				<span class="block h-7 text-3xl font-semibold" style="line-height: 0.8em">
					{blok.title}
				</span>
			</h3>
			<h4 class="font-[400]" style="color: {blok.subtitle_color.color}">{blok.subtitle}</h4>
		</div>
		<p class="grow font-serif max-w-[20rem]">{blok.description}</p>
		{#if blok.cta}
			<StoryblokComponent blok="{blok.cta[0]}" />
		{/if}
	</div>
	<div class="relative flex items-center">
		<img src="{blok.img.filename}" alt="{blok.img.alt}" class="block h-fit max-h-[100%] min-w-[5rem] mx-auto mr-8" />
	</div>
	<div class="absolute right-8 top-8">
		<Tag color="{blok.tag_color.color}">
			Micro SaaS
		</Tag>
	</div>
</section>

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
