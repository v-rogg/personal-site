<script lang="ts">
	import { darkMode } from "$lib/stores";
	import { t } from "$lib/_i18n";
	import { currentSignatureStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import "../app.scss";
	import { onMount } from "svelte";
	import * as cookie from "cookie";

	export let data: { signatureRefs: []; currentSignature };

	if (data) {
		currentSignatureStore.set(data.currentSignature);
		signatureRefsStore.set(data.signatureRefs);
	}

	onMount(() => {
		if (cookie.parse(document.cookie)["darkModeEnabled"] === "true" || (!('darkModeEnabled' in cookie.parse(document.cookie)) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
			darkMode.set(true);
		} else {
			darkMode.set(false);
		}

		darkMode.subscribe((mode) => {
			if (mode) {
				document.documentElement.classList.remove("light");
				document.documentElement.classList.add("dark");
			} else {
				document.documentElement.classList.remove("dark");
				document.documentElement.classList.add("light");
			}
		})
	})

	// telemetry.subscribe((t) => {
	// 	if (t !== undefined) {
	// 		t.signal({ time: Date.now(), appVersion: "1.0.0" });
	// 	}
	// });
</script>

<svelte:head>
	<title>{$t("common.title")}</title>
	<meta name="description" content="{$t('common.meta')}" />

	{#if $darkMode}
		<link rel="icon" type="image/png" sizes="64x64" href="/favicon/dark/favicon-64x64.png" />
		<link rel="icon" type="image/png" sizes="32x32" href="/favicon/dark/favicon-32x32.png" />
		<link rel="icon" type="image/png" sizes="16x16" href="/favicon/dark/favicon-16x16.png" />
	{:else}
		<link rel="icon" type="image/png" sizes="64x64" href="/favicon/light/favicon-64x64.png" />
		<link rel="icon" type="image/png" sizes="32x32" href="/favicon/light/favicon-32x32.png" />
		<link rel="icon" type="image/png" sizes="16x16" href="/favicon/light/favicon-16x16.png" />
	{/if}
</svelte:head>

<!--<div class="app" class:light="{!$darkMode}" class:dark="{$darkMode}">-->
<div class="app bg-white dark:bg-blue-900 flex flex-col items-center">
	<!--	<Header />-->
	<slot />
	<!--	<Footer />-->
	<!--{$admin}-->
	<!--{$identifier}-->
</div>

<style>
	.app {
		color: var(--c-black);
		height: 100%;
		min-height: 100vh;
		top: 0;
		width: 100%;
	}
</style>
