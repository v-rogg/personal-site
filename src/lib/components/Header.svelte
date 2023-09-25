<script lang="ts">
	import { locale } from "$lib/_i18n";
	import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
	import LanguageSelect from "$lib/components/LanguageSelect.svelte";

	export let noLang = false;
</script>

<header class="container container-tight">
	<h1 class="overlay">
		<a href="/{$locale}"> Valentin Rogg </a>
	</h1>

	<div class="options overlay">
		<DarkModeToggle />
		{#if !noLang}
			<LanguageSelect />
		{/if}
	</div>
</header>

<style>
	header {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		justify-content: space-between;
		align-items: center;
		position: fixed;
		top: 4rem;
		z-index: 900;
	}

	@media (max-width: 540px) {
		h1 {
			/*display: none;*/
		}

		header {
			/*top: 2rem;*/
			display: flex;
			flex-direction: column;
			gap: 1rem;
		}
	}

	.options {
		justify-self: flex-end;
	}

	.options {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	h1 {
		font-family: var(--ff-display);
		font-size: 18px;
		font-weight: 700;
		white-space: nowrap;
		padding: 0;
		margin: 0;
		border-radius: 4px;
		backdrop-filter: blur(4px);
		width: max-content;
	}
</style>
