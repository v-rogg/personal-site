<script lang="ts">
	import Signature from "$lib/components/sections/EyeCatcher/Signature.svelte";
	import SignatureNumber from "$lib/components/sections/EyeCatcher/SignatureNumber.svelte";
	import { darkMode, admin } from "$lib/stores";
	import SignatureAdmin from "$lib/components/sections/EyeCatcher/SignatureAdmin.svelte";
	import { browser } from "$app/environment";
	import { PUBLIC_CF_IMAGES_ENDPOINT } from "$env/static/public";

	const imageLight = "3b8293de-9bc4-4c08-623e-37635c553100";
	const imageDark = "4d394d71-c80c-4146-8671-5cbd49119800";
</script>

<svelte:head>
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/50" />
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/75" />
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/100" />
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/50" />
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/75" />
	<link rel="preload" as="image" href="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/100" />
</svelte:head>

<section class="eyecatcher">
	{#if browser}
		<Signature />
		<SignatureNumber />
		{#if $admin}
			<SignatureAdmin />
		{/if}
	{/if}
	{#if !$darkMode}
		<picture>
			<source media="(max-width: 300px)" srcset="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/50" />
			<source media="(max-width: 900px)" srcset="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/75" />
			<img
				class="image-light"
				src="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageDark}/100"
				alt="Handsome Portrait Valentin Rogg"
				width="608"
				height="575" />
		</picture>
	{:else}
		<picture>
			<source media="(max-width: 300px)" srcset="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/50" />
			<source media="(max-width: 900px)" srcset="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/75" />
			<img
				class="image-dark"
				src="{PUBLIC_CF_IMAGES_ENDPOINT}/{imageLight}/100"
				alt="Handsome Portrait Valentin Rogg"
				width="608"
				height="575" />
		</picture>
	{/if}
</section>

<style>
	.eyecatcher {
		background: var(--c-secondary);
		flex-grow: 1;
		margin: 2rem;
		border-radius: 4px;
		position: relative;
		height: 650px;
		overflow: hidden;
	}

	@media (max-width: 540px) {
		.eyecatcher {
			margin: 1rem 1rem 1rem;
		}
	}

	img {
		position: absolute;
		bottom: 0;
		left: 50%;
		transform: translate(-50%);
	}

	.image-dark {
		filter: hue-rotate(-3deg);
	}
</style>
