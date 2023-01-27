<script lang="ts">
	import { darkMode, telemetry } from "$lib/stores";
	import { t } from "$lib/_i18n";
	import { currentSignatureStore, signatureRefsStore } from "$lib/stores";

	export let data: {signatureRefs: [], currentSignature};

	if (data) {
		currentSignatureStore.set(data.currentSignature)
		signatureRefsStore.set(data.signatureRefs)
	}

	telemetry.subscribe(t => {
		if (t !== undefined) { t.signal({time: Date.now(), appVersion: "1.0.0"}) }
	})
</script>

<svelte:head>
	<title>{$t('common.title')}</title>
	<meta name="description" content="{$t('common.meta')}" />

	{#if $darkMode}
		<link rel="icon" type="image/png" sizes="64x64" href="/favicon/dark/favicon-64x64.png">
		<link rel="icon" type="image/png" sizes="32x32" href="/favicon/dark/favicon-32x32.png">
		<link rel="icon" type="image/png" sizes="16x16" href="/favicon/dark/favicon-16x16.png">
	{:else}
		<link rel="icon" type="image/png" sizes="64x64" href="/favicon/light/favicon-64x64.png">
		<link rel="icon" type="image/png" sizes="32x32" href="/favicon/light/favicon-32x32.png">
		<link rel="icon" type="image/png" sizes="16x16" href="/favicon/light/favicon-16x16.png">
	{/if}
</svelte:head>

<div class="app" class:light={!$darkMode} class:dark={$darkMode}>
<!--	<Header />-->
	<slot />
<!--	<Footer />-->
	<!--{$admin}-->
	<!--{$identifier}-->
</div>

<style>
  .app {
    color: var(--c-black);
    background: var(--c-bg);
    height: 100%;
    min-height: 100vh;
    position: absolute;
    top: 0;
    width: 100%;
    padding-bottom: 200px;
  }
</style>
