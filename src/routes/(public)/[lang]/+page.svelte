<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import ShortBio from "$lib/components/sections/ShortBio/ShortBio.svelte";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";

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

<section class="second container">
	<ShortBio />
</section>

<style>
	.first {
		display: flex;
		flex-direction: column;
	}

	.second {
		padding-bottom: 9rem;
	}
</style>
