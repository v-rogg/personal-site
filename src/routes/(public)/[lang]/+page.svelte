<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import ShortBio from "$lib/components/sections/ShortBio/ShortBio.svelte";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";
	import CV from "$lib/components/sections/CV/CV.svelte";

	export let data: {
		lang: string;
		route: string;
		slug: string;
		signatureRefs: [];
		currentSignature;
	};

	if (data) {
		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);
	}
</script>

<svelte:head>
	<title>Valentin Rogg</title>
</svelte:head>

<section class="first container">
	<EyeCatcher />
</section>

<section class="second container my-8 mb-12">
	<ShortBio />
</section>

<section class="third container">
	<CV />
</section>

<style>
	.first {
		display: flex;
		flex-direction: column;
	}
</style>
