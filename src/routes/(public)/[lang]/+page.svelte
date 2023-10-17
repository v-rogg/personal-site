<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import ShortBio from "$lib/components/sections/ShortBio/ShortBio.svelte";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";
	import CV from "$lib/components/sections/CV/CV.svelte";
	import { onMount } from "svelte"
	import { useStoryblokBridge } from "@storyblok/svelte";
	import type { Content, CvContent, BioContent, MemoriesSettingsContent, MemoryContent } from "$lib/storyblok/schema";
	import Memories from "$lib/components/sections/Memories/Memories.svelte";
	import { t } from "$lib/_i18n";

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

<section class="container mb-4">
	<EyeCatcher />
</section>

<section class="grid gap-4 container mb-4 grid-cols-4 lg:grid-cols-8">
	<ShortBio blok={data.bio}/>
	<div class="bg-white-600 rounded-2xl p-8 col-span-4 h-[25rem] overflow-hidden">
		<h2 class="text-2xl font-semibold text-center mb-10">{$t("cv.title")}</h2>
		<div class="hover:scale-[102%] duration-1000">
			<CV blok={data.cv}/>
		</div>
	</div>
</section>

<section class="grid gap-4 grid-cols-11 container mb-8">
	<Memories settings="{data.memoriesSettings}" content="{data.memories}"/>
</section>

<!--<section class="third container mb-12">-->
<!--	{#if data.cv}-->
<!--	{/if}-->
<!--</section>-->
