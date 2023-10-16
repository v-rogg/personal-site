<script lang="ts">
	import { locale } from "$lib/_i18n";
	import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
	import LanguageSelect from "$lib/components/LanguageSelect.svelte";

	export let noLang = false;
</script>

<header class="container container-tight">
	<h1 class="rounded-3xl px-4 py-2 -mx-4 -my-2 max-w-max backdrop-blur-[4px] font-display font-semibold text-md-lg">
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
		header {
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
</style>
