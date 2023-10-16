<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import ShortBio from "$lib/components/sections/ShortBio/ShortBio.svelte";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";
	import CV from "$lib/components/sections/CV/CV.svelte";
	import { onMount } from "svelte"
	import { useStoryblokBridge } from "@storyblok/svelte";
	import type { Content, CvContent, BioContent, MemoriesSettingsContent, MemoryContent } from "$lib/storyblok/schema";
	import Memories from "$lib/components/sections/Memories/Memories.svelte";

	export let data: {
		lang: string;
		route: string;
		slug: string;
		bio: Content<BioContent>;
		cv: Content<CvContent>;
		memoriesSettings: Content<MemoriesSettingsContent>;
		memories: Content<MemoryContent>[];
		signatureRefs: [];
		currentSignature: object;
	};

	if (data) {
		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);
	}

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
			data.memoriesSettings.id,
			(newStory) => {data.memoriesSettings = newStory}
		)
		useStoryblokBridge(
			data.memories.id,
			(newStory) => {data.memories = newStory}
		);
	});
</script>

<div>
</div>

<svelte:head>
	<title>Valentin Rogg</title>
</svelte:head>

<section class="first container mb-8">
	<EyeCatcher />
</section>

<section class="second container mb-8">
	<ShortBio blok={data.bio}/>
</section>

<section class="third container mb-48">
	<Memories settings="{data.memoriesSettings}" content="{data.memories}"/>
</section>

<!--<section class="third container mb-12">-->
<!--	{#if data.cv}-->
<!--		<CV blok={data.cv}/>-->
<!--	{/if}-->
<!--</section>-->

<style>
	.first {
		display: flex;
		flex-direction: column;
	}
</style>
