<script lang="ts">
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import { t } from "$lib/_i18n";
	import EyeCatcher from "$lib/components/sections/EyeCatcher/EyeCatcher.svelte";
	import Header from "$lib/components/globals/Header.svelte";
	import { beforeUpdate } from "svelte";

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

	beforeUpdate(() => {
		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);
	})
</script>

<svelte:head>
	<title>{$t("common.title")} · {"Admin"}</title>
</svelte:head>

<Header noLang />

<section class="first container">
	<EyeCatcher />
</section>

<style>
	.first {
		display: flex;
	}
</style>
