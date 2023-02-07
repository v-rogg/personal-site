<script lang="ts">
	import { currentSignatureStore } from "$lib/stores";
	import { locale } from "$lib/_i18n";
	import { browser } from "$app/environment";
</script>

<div class="container">
	<div class="signature-number">
		{#if $currentSignatureStore && browser}
			{#if Object.keys($currentSignatureStore).length > 0}
				<div class="name">
					<i class="fa-solid fa-tag"></i>
					{$currentSignatureStore?.name ? $currentSignatureStore?.name : ""}
				</div>
				<div id="date" class="small">
					{$currentSignatureStore.ts_created
						? new Date($currentSignatureStore.ts_created / 1000).toLocaleDateString($locale)
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
		animation: 250ms in ease-in-out;
	}

	@keyframes in {
		0% {
			opacity: 0;
		}

		100% {
			opacity: 1;
		}
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
