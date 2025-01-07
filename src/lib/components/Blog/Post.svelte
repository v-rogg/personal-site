<script lang="ts">
	import type { Metadata } from "$lib/types";
	import { blur } from "svelte/transition";
	import PublishBadge from "./PublishBadge.svelte";

	let { url, metadata, cls = "" }: { url: string; metadata: Metadata; cls: string } = $props();
</script>

<a
	href={url}
	class="{cls} project relative flex h-80 flex-col overflow-hidden bg-white-600 pl-10 pr-10 pt-10 backdrop-blur transition duration-500 hover:shadow-xl sm:h-80 sm:rounded-xl"
	in:blur={{ duration: 500, amount: 10, delay: 400 }}
	class:h-[500px]={metadata?.previewImageUrl !== undefined}
>
	<h3 class="relative z-20 text-xl font-semibold text-black">{metadata?.title}</h3>
	<h4 class="text-md relative z-20 text-black">{metadata?.subtitle}</h4>
	{#if metadata?.date}
		<div class="text absolute right-10 top-10 z-20 leading-none text-black">
			{new Date(metadata.date).toLocaleDateString("de")}
		</div>
	{/if}
	<ul class="relative z-20 mt-4 flex flex-wrap gap-2 text-black">
		{#each metadata?.tags?.sort() || [] as tag}
			<li class="inline-flex items-center gap-0.5 rounded-md bg-white-700 px-3 py-2 text-sm font-medium leading-none">
				<!-- <i class="fa-solid fa-hashtag text-xs"></i>{tag} -->
				{tag}
			</li>
		{/each}
	</ul>
	<div class="relative mt-4 flex h-[150px] flex-col items-center gap-8 sm:h-[128px] sm:flex-row">
		<p class="preview-text h-full w-full overflow-ellipsis hyphens-auto whitespace-normal text-balance text-justify">
			{metadata?.preview}
		</p>
		{#if metadata?.previewImageUrl}
			<img src="{metadata.previewImageUrl}/h=800" alt={metadata.title} class="-mr-6 h-80 -translate-y-4" />
		{/if}
	</div>
	<div
		class="btn absolute bottom-10 right-10 z-20 block w-max rounded-lg bg-black p-2 text-white opacity-0 shadow-lg transition duration-500"
	>
		<!-- href={url} -->
		Mehr lesen <i class="fa-regular fa-book-open ml-1"></i>
	</div>
	<span
		class="overlay bg-gradient-radial pointer-events-none absolute bottom-0 right-0 z-10 h-full w-full bg-gradient-to-br from-transparent from-20% to-white-600/90 opacity-0 transition duration-500"
	>
	</span>
	<PublishBadge published={metadata?.published || false} />
</a>

<style lang="postcss">
	.project:hover {
		.overlay {
			opacity: 1;
		}

		.btn {
			opacity: 1;
		}
	}

	.preview-text {
		display: -webkit-box;
		-webkit-line-clamp: 5;
		-webkit-box-orient: vertical;
	}

	.tag {
		font-family: "JetBrains Mono", serif;
	}
</style>
