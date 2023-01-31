<script lang="ts">
	import { currentSignatureStore } from "./stores";
	import { locale } from "$lib/_i18n";
	import { browser } from "$app/environment";
</script>

<div class="container">
	<div class="signature-number">
		{#if $currentSignatureStore && browser}
			{#if Object.keys($currentSignatureStore).length > 0}
				<div class="name">
					<i class="fa-solid fa-tag"></i>
					{$currentSignatureStore?.data.name ? $currentSignatureStore?.data.name : ""}
				</div>
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
		pointer-events: none;
	}

	@media (max-width: 540px) {
		.container {
			width: calc(100% + 2rem);
		}
	}

	.signature-number {
		position: absolute;
		bottom: 2rem;
		z-index: 800;
		pointer-events: all;
	}

	.small {
		font-size: 12px;
	}

	.name {
		font-weight: 700;
		display: flex;
		align-items: center;
		gap: 4px;
		margin-bottom: 2px;
	}
</style>
