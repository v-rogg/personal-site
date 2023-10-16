<script lang="ts">
	import { t } from "$lib/_i18n";
	import type { Content, MemoriesSettingsContent, MemoryContent } from "$lib/storyblok/schema";
	import { StoryblokComponent } from "@storyblok/svelte";

	export let settings: Content<MemoriesSettingsContent>;
	export let content: Content<MemoryContent>[];

	let width = 0;
	$: breakpoint = getBreakpoint(width)
	$: columns = getColumns(settings.content, breakpoint)

	function getBreakpoint(width: number) {
		if (width <= 640) {
			return "sm"
		} else if (width <= 768) {
			return "md"
		} else if (width <= 1024) {
			return "lg"
		} else if (width <= 1280) {
			return "xl"
		} else if (width <= 1536) {
			return  "twoxl"
		} else {
			return "threexl"
		}
	}

	function getColumns(settings: object, breakpoint: string): number {
		return settings[breakpoint]
	}
</script>

<svelte:window bind:innerWidth={width} />

<!--<h2 class="text-2xl font-semibold text-center mb-10">{$t("memories.title")}</h2>-->
<div class="grid gap-4" style="grid-template-columns: repeat({columns}, 1fr);">
	{#each content as memory}
		<StoryblokComponent blok="{memory.content}" />
	{/each}
</div>
