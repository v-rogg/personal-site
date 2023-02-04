<script lang="ts">
	import { currentSignatureStore } from "$lib/stores";
	import { enhance } from "$app/forms";

	const password = "pa=pZ3yoVXEAEEPNmy2G2u7WX44PimyB6Hk";
</script>

<div class="container">
	{#if $currentSignatureStore}
		<form
			class="admin-panel"
			method="post"
			use:enhance="{({ data }) => {
				data.set('id', $currentSignatureStore?._id);

				return async ({ result }) => {
					if (result.status === 200) $currentSignatureStore.status = result.data.status;
				};
			}}">
			<button
				id="approve"
				formaction="?/approve&{password}"
				class:notstatus="{$currentSignatureStore?.status !== 'approved' &&
					$currentSignatureStore?.status !== 'new'}">
				<i class="fa-regular fa-check"></i>
			</button>
			<button
				id="decline"
				formaction="?/decline&{password}"
				class:notstatus="{$currentSignatureStore?.status !== 'declined' &&
					$currentSignatureStore?.status !== 'new'}">
				<i class="fa-regular fa-xmark"></i>
			</button>
		</form>
	{/if}
</div>

<style>
	.admin-panel {
		position: absolute;
		bottom: 2rem;
		z-index: 800;
		left: 50%;
		transform: translateX(-50%);
		display: grid;
		grid-template-columns: repeat(2, 3rem);
		align-items: center;
		gap: 2rem;
	}

	.notstatus:before {
		opacity: 0.6;
		transform: scale(0.8);
	}

	button {
		border: none;
		font-size: 1rem;
		z-index: 250;
		width: 3rem;
		height: 3rem;
		border-radius: 100%;
		position: relative;
		color: var(--r-white);
		background: none;
		justify-self: center;
	}

	button:before {
		content: "";
		position: absolute;
		transition: 100ms ease-in-out;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border-radius: 100%;
		z-index: -1;
	}

	button:hover {
		opacity: 1;
		cursor: pointer;
	}

	#approve:before {
		background: var(--c-green);
	}

	#approve:hover:before {
		background: var(--c-green-hover);
	}

	#approve:active:before {
		background: var(--c-green-active);
	}

	#decline:before {
		background: var(--c-red);
	}

	#decline:hover:before {
		background: var(--c-red-hover);
	}

	#decline:active:before {
		background: var(--c-red-active);
	}
</style>
