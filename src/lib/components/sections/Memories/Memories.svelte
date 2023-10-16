<script lang="ts">
	import { t } from "$lib/_i18n";
	import type { Content, MemoriesSettingsContent, MemoryContent } from "$lib/storyblok/schema";
	import { StoryblokComponent } from "@storyblok/svelte";

	export let settings: Content<MemoriesSettingsContent>;
	export let content: Content<MemoryContent>[];

	$: rows = settings.content.rows || 1;
	$: columns = settings.content.columns || 1;
</script>

<h2 class="text-2xl font-semibold text-center mb-10">{$t("memories.title")}</h2>
<div class="grid gap-4" style="grid-template-columns: repeat({columns}, 1fr); grid-template-rows: repeat({rows}, 1fr);">
	{#each content as memory}
		<StoryblokComponent blok="{memory.content}" />
	{/each}
</div>
<div class="flex justify-center mt-4">
	<button class="p-2 bg-white-600 rounded-[100%] w-14 h-14 text-lg">
		<i class="fa-solid fa-plus"></i>
	</button>
</div>