<script lang="ts">
	import { currentSignatureStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import { locale } from "$lib/_i18n";
	import { browser } from "$app/environment";
</script>

<div class="container container-tight">
	<div class="signature-number">
		{#if $currentSignatureStore && browser}
			{#if Object.keys($currentSignatureStore).length > 0}
				<div class="name">
					{#if $currentSignatureStore?.name}
						<svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512" class="mr-1"><path d="M0 80V229.5c0 17 6.7 33.3 18.7 45.3l176 176c25 25 65.5 25 90.5 0L418.7 317.3c25-25 25-65.5 0-90.5l-176-176c-12-12-28.3-18.7-45.3-18.7H48C21.5 32 0 53.5 0 80zm112 32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z" fill="currentColor"/></svg>
						{$currentSignatureStore?.name}
					{/if}
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
