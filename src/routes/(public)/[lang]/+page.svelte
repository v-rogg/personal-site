<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import ShortBio from "$lib/components/sections/ShortBio/ShortBio.svelte";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";
	import { onMount } from "svelte"
	import { useStoryblokBridge, StoryblokComponent } from "@storyblok/svelte";
	import type { Content, CvContent, BioContent} from "$lib/storyblok";
	import { storyblokEditable } from "@storyblok/svelte";

	export let data: {
		lang: string;
		route: string;
		slug: string;
		bio: Content<BioContent>;
		cv: Content<CvContent>;
		page;
		signatureRefs: [];
		currentSignature: object;
	};

	if (data) {
		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);

		console.log(data.page);
	}

	$: page = data.page.content

	onMount(() => {
		useStoryblokBridge(
			data.cv.id,
			(newStory) => {data.cv = newStory}
		);
		useStoryblokBridge(
			data.bio.id,
			(newStory) => {data.bio = newStory}
		);
		useStoryblokBridge(
			data.page.id,
			async (newStory) => {
				data.page = newStory}
		);
	});
</script>

<svelte:head>
	<title>Valentin Rogg</title>
</svelte:head>

<section class="container mb-4">
	<EyeCatcher />
</section>

<section class="grid gap-4 container mb-4 grid-cols-1 lg:grid-cols-2">
	<ShortBio blok={data.bio}/>
	<StoryblokComponent blok="{data.page.content.spotlight[0]}" />

</section>

<section class="grid-settings grid gap-4 container mb-8" style="--sm: {page.grid_sm}; --md: {page.grid_md}; --lg: {page.grid_lg}; --xl: {page.grid_xl}" use:storyblokEditable={data.page}>
	{#each data.page.content.feed as blok}
		<StoryblokComponent {blok} />
	{/each}
</section>

<style lang="postcss">
	.grid-settings {
		@media (min-width: theme('screens.sm')) {
			grid-template-columns: repeat(var(--sm), 1fr);
		}
		@media (min-width: theme('screens.md')) {
			grid-template-columns: repeat(var(--md), 1fr);
		}
		@media (min-width: theme('screens.lg')) {
			grid-template-columns: repeat(var(--lg), 1fr);
		}
		@media (min-width: theme('screens.xl')) {
			grid-template-columns: repeat(var(--xl), 1fr);
		}
	}
</style>
