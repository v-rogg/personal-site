<script lang="ts">
	import "../app.css";
	import "@awesome.me/kit-fb271958c9/icons/css/all.min.css";
	import Footer from "$lib/components/globals/Footer.svelte";
	import Header from "$lib/components/globals/Header.svelte";
	import Toaster from "$lib/components/globals/Toaster.svelte";
	import { page } from "$app/state";
	import posthog from "posthog-js";
	import { browser } from "$app/environment";
	import { PUBLIC_POSTHOG_PROJECT_ID } from "$env/static/public";
	import { afterNavigate, beforeNavigate } from "$app/navigation";

	let { children } = $props();

	if (browser && PUBLIC_POSTHOG_PROJECT_ID) {
		posthog.init(PUBLIC_POSTHOG_PROJECT_ID, {
			api_host: "https://eu.i.posthog.com",
			persistence: "localStorage",
			capture_pageview: false,
			capture_pageleave: false,
			autocapture: true
		});
		beforeNavigate(() => posthog.capture("$pageleave"));
		afterNavigate(() => posthog.capture("$pageview"));
	}
</script>

<svelte:head>
	<title>VR • Valentin Rogg</title>
	{#if !page.url.hostname.includes("localhost")}
		<script src="https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit" async defer></script>
	{/if}
</svelte:head>

<Toaster />

<Header />

{#key page.url.pathname}
	<div class="font-sans text-black">
		{@render children()}
	</div>
{/key}

<div class="mt-16 flex justify-center">
	<button
		onclick={() => {
			window.scrollTo({ top: 0, behavior: "smooth" });
			posthog.capture("click.back-to-top");
		}}
		class="rounded-md px-2 py-1 hover:bg-white-600 active:bg-white-700"
		aria-label="Zurück nach oben"
	>
		<i class="fa-solid fa-arrow-up mr-2"></i>
		Nach oben
	</button>
</div>

<Footer />
