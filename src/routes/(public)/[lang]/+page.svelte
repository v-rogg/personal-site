<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/stores";
	import ShortBio from "$lib/ShortBio.svelte";
	import EyeCatcher from "$lib/EyeCatcher.svelte";

	export let data: {
		lang: string;
		route: string;
		slug: string;
		signatureRefs: [];
		currentSignature;
	};

	if (data) {
		console.log(data);

		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);
	}
</script>

<svelte:head>
	<title>Valentin Rogg</title>
</svelte:head>

<section class="first">
	<EyeCatcher />
</section>

<section class="second">
	<ShortBio />
</section>

<style>
	.first {
		display: flex;
		flex-direction: column;
	}

	/*.second {*/
	/*  margin-top: 10rem;*/
	/*  margin-bottom: 4rem;*/
	/*}*/
</style>
