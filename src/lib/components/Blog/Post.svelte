<script lang="ts">
	import type { Metadata } from "$lib/types";
	import TagList from "$lib/components/globals/TagList.svelte";
	import { blur } from "svelte/transition";
	import PublishBadge from "./PublishBadge.svelte";

	let { url, metadata, cls = "" }: { url: string; metadata: Metadata; cls: string } = $props();
</script>

<a
	href={url}
	class="{cls} project relative flex flex-col overflow-hidden bg-white-600 pl-10 pr-10 pt-9 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl"
	in:blur={{ duration: 500, amount: 10, delay: 400 }}
>
	<!-- class:max-h-[500px]={metadata?.previewImageUrl !== undefined} -->
	<div class="flex justify-between">
		<h3 class="relative z-20 text-xl font-semibold text-black">{metadata?.title}</h3>
		{#if metadata?.date}
			<div class="leading-7 text-black">
				{new Date(metadata.date).toLocaleDateString("de-DE", { day: "numeric", month: "2-digit", year: "numeric" })}
			</div>
		{/if}
	</div>
	<h4 class="text-md relative z-20 text-black">{metadata?.subtitle}</h4>
	{#if metadata.tags}
		<TagList tags={metadata?.tags} cls="bg-fade relative z-20 mt-4 rounded-md" />
	{/if}
	<div
		class="relative mt-4 flex flex-col items-start gap-8 sm:flex-row sm:items-start sm:justify-between xl:h-full xl:items-end"
	>
		<p
			class="max-lg:preview-text h-full w-full overflow-ellipsis whitespace-normal text-pretty text-justify max-sm:hyphens-auto sm:pb-10"
		>
			{metadata?.preview}
		</p>
		{#if metadata?.previewImageUrl}
			<img
				src={metadata.previewImageUrl}
				alt={metadata.title}
				class="fade -mr-10 -mt-4 ml-auto h-auto max-w-72 opacity-90 xl:max-w-48 2xl:max-w-72"
			/>
		{/if}
	</div>
	<div
		class="btn absolute bottom-10 right-10 z-20 block w-max rounded-lg bg-black p-2 text-white opacity-0 shadow-lg transition duration-500"
	>
		Mehr lesen <i class="fa-regular fa-book-open ml-1"></i>
	</div>
	<span
		class="overlay bg-gradient-radial pointer-events-none absolute bottom-0 right-0 z-10 h-full w-full bg-gradient-to-br from-transparent from-20% to-white-600/90 opacity-0 transition duration-500"
	>
	</span>
	<PublishBadge published={metadata?.published || false} />
</a>

<style lang="postcss">
	.bg-fade {
		box-shadow: 0 0 40px theme(colors.white.600);
		@apply bg-white-600/30;
	}

	.project:hover {
		.overlay {
			opacity: 1;
		}

		.btn {
			opacity: 1;
		}
	}

	@layer {
		.preview-text {
			display: -webkit-box;
			-webkit-line-clamp: 5;
			-webkit-box-orient: vertical;
		}
	}

	.tag {
		font-family: "JetBrains Mono", serif;
	}
</style>
