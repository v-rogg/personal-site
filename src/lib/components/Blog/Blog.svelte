<script lang="ts">
	import Post from "$lib/components/Blog/Post.svelte";
	import type { Metadata } from "$lib/types";
	import { SvelteMap } from "svelte/reactivity";
	import { blur } from "svelte/transition";

	let { blog, tags } = $props();

	let filterOpen = $state(true);
	let filterTags: Map<string, boolean> = $state(new SvelteMap(tags.map((tag: string) => [tag, true])));
	const gridColSeries = [4, 5, 6, 3, 5, 4, 3, 6];

	function checkTags(metadata: Metadata) {
		if (metadata && metadata.tags) {
			for (let tag of metadata.tags) {
				if (filterTags.has(tag) && filterTags.get(tag)) {
					return true;
				}
			}
			return false;
		}
		return true;
	}
</script>

<section id="blog" class="mt-20">
	<div class="mb-4 flex flex-col items-end gap-12 lg:flex-row">
		<h2
			class="-mb-[0.3em] block w-max text-[3.5rem] font-bold leading-none tracking-tight text-white-700 max-lg:-mt-10 max-md:pr-10 sm:text-[6rem] md:-mb-[0.14em] md:text-[10rem] lg:-ml-[8px] xl:-ml-[7px] 2xl:-ml-[12px] 2xl:text-[16rem]"
		>
			Beiträge
		</h2>
		<div class="flex h-max flex-wrap items-end justify-between gap-2 max-md:px-10 lg:justify-end">
			{#each tags.sort() as tag, i}
				{#if filterOpen}
					<button
						id="i"
						class="block h-max overflow-hidden text-nowrap rounded-lg bg-white-700 px-3 py-1 text-sm font-medium opacity-30 backdrop-blur transition hover:bg-white-600 active:bg-white-500"
						in:blur={{ duration: 500, delay: 25 * i, amount: 10 }}
						out:blur={{ duration: 500, delay: 25 * (tags.length - i), amount: 10 }}
						class:active={filterTags.get(tag)}
						onclick={() => {
							const countTrue = [...filterTags.values()].filter((value) => value === true).length;
							if (countTrue === filterTags.size) {
								filterTags.forEach((value, key) => filterTags.set(key, false));
							}

							if (countTrue === 1 && filterTags.get(tag)) {
								filterTags.forEach((value, key) => filterTags.set(key, true));
							} else {
								filterTags.set(tag, !filterTags.get(tag));
							}
						}}
					>
						{tag}
					</button>
				{/if}
			{/each}
		</div>
	</div>
	<div class="grid grid-cols-1 gap-4 lg:grid-cols-9">
		{#each blog as entry, i}
			<Post
				url={entry.route}
				cls="col-span-{gridColSeries[i % 8]} max-lg:col-span-1 {checkTags(entry.metadata)
					? 'opacity-100'
					: 'opacity-50 bg-white-600/30'} transition"
				metadata={entry.metadata}
			/>
		{/each}
	</div>
</section>

<style lang="postcss">
	.active {
		@apply opacity-100;
	}
</style>
