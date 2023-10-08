<script lang="ts">
	import { goto } from "$app/navigation";
	import { slugStore } from "../stores";
	import { l, t, locale, locales } from "$lib/_i18n";
	import { get } from "svelte/store";

	// TODO: Add prefetching. Only load data not change lang when load functions run
	// async function preload_toggle_lang(lang) {
	// 	let i = $locales.findIndex((e) => e === lang);
	// 	await preloadData(`/${$locales[i]}${get(l)($locales[i], $slugStore)}`);
	// }

	function toggle_lang(lang) {
		let i = $locales.findIndex((e) => e === lang);
		goto(`/${$locales[i]}${get(l)($locales[i], $slugStore)}`, { noScroll: true });
	}
</script>

<div class="languages">
	{#each $locales as providedLocale}
		<button
			on:click="{() => {
				toggle_lang(providedLocale);
			}}"
			class="language_switch"
			class:active="{providedLocale === $locale}"
			disabled="{providedLocale === $locale}"><span>{$t(`lang.${providedLocale}`)}</span></button>
	{/each}
</div>

<style lang="scss">
	.languages {
		display: flex;
		backdrop-filter: blur(8px);
		border-radius: 4px;
	}

	.language_switch {
		width: 3rem;
		height: 1.5rem;
		border: none;
		border-radius: 4px;
		background: none;
		font-size: var(--fs-08);
		display: flex;
		justify-content: center;
		align-items: center;
		position: relative;

		&:before {
			position: absolute;
			content: "";
			border-radius: 4px;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			z-index: -1;
		}
	}

	@keyframes in-right {
		0% {
			left: 100%;
		}
		100% {
			left: 0;
		}
	}

	@keyframes in-left {
		0% {
			left: -100%;
		}
		100% {
			left: 0;
		}
	}

	.language_switch > span {
		//transform: translateY(-1px);
	}

	.active {
		font-weight: 700;
		&:before {
			background: var(--c-bg);
			animation: in-right 250ms ease-in-out;
		}
	}

	.active:last-of-type:before {
		animation: in-left 250ms ease-in-out;
	}

	.language_switch:hover:not(.active) {
		//background: var(--c-bg-hover);
		cursor: pointer;
	}

	.language_switch:active:not(.active) {
		//background: var(--c-bg-active);
		cursor: default;
	}
</style>
