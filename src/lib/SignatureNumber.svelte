<script lang="ts">
	import { currentSignatureStore } from "./stores";
	import { locale } from "$lib/_i18n";
	import { browser } from "$app/environment";
</script>

<div class="container">
	<div class="signature-number">
		{#if $currentSignatureStore && browser}
			{#if Object.keys($currentSignatureStore).length > 0}
				<div>{$currentSignatureStore?.data.name ? $currentSignatureStore?.data.name : ""}</div>
				<div id="date" class="small">
					{$currentSignatureStore.ts
						? new Date($currentSignatureStore.ts / 1000).toLocaleDateString($locale)
						: ""}
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.container {
		position: absolute;
		height: 100%;
		width: calc(100% + 4rem);
		left: 50%;
		transform: translateX(-50%);
		z-index: 799;
	}

	@media (max-width: 440px) {
		.container {
			width: calc(100% + 2rem);
		}
	}

	.signature-number {
		position: absolute;
		bottom: 2rem;
		z-index: 800;
	}

	.small {
		font-size: 12px;
	}
</style>
