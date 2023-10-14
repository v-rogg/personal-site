<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import SignaturePad from "signature_pad";
	import { darkMode, admin } from "$lib/stores";
	import {
		currentSignatureStore,
		refIndexStore,
		signatureRefsStore
	} from "$lib/components/sections/EyeCatcher/signature.stores";
	import { t } from "$lib/_i18n";
	import xss from "xss";
	import { enhance } from "$app/forms";
	import { fade } from "svelte/transition";
	import { getPublicFaunaClient } from "$lib/fauna/fauna.public";
	import { loadDelta } from "$lib/components/sections/EyeCatcher/signature.helpers";
	import type { Signature, SignatureData } from "$lib/fauna/schema";
	import { sendClientEvent } from "$lib/posthog";

	let canvas: HTMLCanvasElement;
	let pad: HTMLDivElement;
	let signaturePad;
	let currentSignature: Signature;
	let drawModeActive = false;
	let eraseModeActive = false;

	function clearCanvas() {
		switchToDraw()
		sendClientEvent('signature-clear-canvas', document);
		signaturePad.clear();
	}

	function centerSignature(data): SignatureData[] {
		const middleH = pad.offsetWidth / 2;
		const bottomV = pad.offsetHeight;

		let centeredData = [];

		for (let signature of data) {
			let centeredSignature = JSON.parse(JSON.stringify(signature));

			centeredSignature.points.forEach((point) => {
				point.x = point.x - middleH;
				point.y = point.y - bottomV;
			});

			centeredData.push(centeredSignature);
		}

		return centeredData;
	}

	function uncenterSignature(data) {
		const middleH = pad.offsetWidth / 2;
		const bottomV = pad.offsetHeight;

		let centeredData = [];

		for (let signature of data) {
			let centeredSignature = JSON.parse(JSON.stringify(signature));

			centeredSignature.points.forEach((point) => {
				point.x = point.x + middleH;
				point.y = point.y + bottomV;
			});

			centeredData.push(centeredSignature);
		}

		return centeredData;
	}

	const fauna = getPublicFaunaClient();

	async function loadSignature(delta) {
		drawModeActive = false;
		loadDelta(delta, fauna);
		sendClientEvent('signature-load-delta', document);
	}

	function resizeCanvas() {
		canvas.width = pad.offsetWidth;
		canvas.height = pad.offsetHeight;

		clearCanvas();

		if ($currentSignatureStore) {
			const currentSignature = uncenterSignature($currentSignatureStore.signature);
			signaturePad.fromData(currentSignature);
		}
	}

	function newCanvas() {
		drawModeActive = true;
		switchToDraw();

		$currentSignatureStore = <Signature>{
			name: null,
			signature: []
		};

		signaturePad.on();
	}

	function switchToErase() {
		eraseModeActive = true;
		signaturePad.compositeOperation = 'destination-out';
		signaturePad.minWidth = 10
		signaturePad.maxWidth = 10
		sendClientEvent('signature-switch-to-erase', document);
	}

	function switchToDraw() {
		eraseModeActive = false;
		signaturePad.compositeOperation = 'source-over';
		signaturePad.minWidth = 0.5
		signaturePad.maxWidth = 2.5
		sendClientEvent('signature-switch-to-draw', document);
	}

	onMount(() => {
		signaturePad = new SignaturePad(canvas, {
			penColor: "#0f1418",
			velocityFilterWeight: 0.3,
			minDistance: 2,
			throttle: 8
		});
		signaturePad.off();
		pad = <HTMLDivElement>document.getElementById("pad");

		signaturePad.addEventListener("endStroke", () => {
			currentSignature = $currentSignatureStore;
			currentSignature.signature = centerSignature(signaturePad.toData());
			$currentSignatureStore = currentSignature;
		});

		window.onresize = resizeCanvas;
		resizeCanvas();

		currentSignatureStore.subscribe((data: Signature) => {
			if (data !== <Signature>undefined) {
				if (Object.keys(data).length > 0) {
					signaturePad.fromData(uncenterSignature(data.signature));
				}
			} else {
				signaturePad.clear();
			}
		});
	});

	onDestroy(() => {
		fauna.close();
	})
</script>

<div id="pad">
	<canvas id="signature" bind:this="{canvas}" class:dark="{$darkMode}"></canvas>
	<div class="container container-tight">
		<div class="overlay">
			{#if !drawModeActive && $signatureRefsStore}
				{#if $signatureRefsStore.length - $refIndexStore - 1 > 0}
					<button
						id="next"
						class="dark:bg-blue-900 dark:hover:bg-blue-700"
						on:click="{() => loadSignature(1)}"
						aria-label="{$t('signature.next')}"
						transition:fade="{{ duration: 150 }}">
						<i class="fa-solid fa-angle-right"></i>
					</button>
				{/if}
				{#if $refIndexStore > 0}
					<button
						id="prev"
						class="dark:bg-blue-900 dark:hover:bg-blue-700"
						on:click="{() => loadSignature(-1)}"
						aria-label="{$t('signature.prev')}"
						transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-angle-left"></i>
					</button>
				{/if}
			{/if}

			{#if drawModeActive}
				<form
					method="post"
					action="?/create"
					use:enhance="{({ formData, cancel }) => {
						const signature = centerSignature(signaturePad.toData());
						if (signature.length === 0) {
							cancel();
						} else {
							const name = xss(prompt($t('signature.prompt'), ''));
							if (!name) cancel();

							const json = JSON.stringify({
								name: name,
								signature
							});

							formData.set('data', json);
							sendClientEvent('create-signature', document, {signature: name})
						}


						return async ({ result }) => {
							if (result.status === 200) {
								alert($t('signature.submit_alert'))
								$currentSignatureStore = result.data;
								let signatureRefs = $signatureRefsStore;
								signatureRefs.splice($refIndexStore + 1, 0, ({id: result.data.id}));
								$signatureRefsStore = signatureRefs;
								$refIndexStore += 1;

								switchToDraw();
								drawModeActive = false;
								signaturePad.off();
							}
						};
					}}"
					style="pointer-events: all">
					<button id="save" class="dark:bg-blue-900 dark:hover:bg-blue-700" aria-label="{$t('signature.save')}" transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-floppy-disk"></i>
					</button>
				</form>
				<button
					id="clear"
					class="dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => clearCanvas()}"
					aria-label="{$t('signature.clear')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-duotone fa-trash"></i>
				</button>
				{#if !eraseModeActive}
				<button
					id="erase"
					class="dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => switchToErase()}"
					aria-label="{$t('signature.erase')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-solid fa-eraser"></i>
				</button>
				{:else}
					<button
						id="draw"
						class="dark:bg-blue-900 dark:hover:bg-blue-700"
						on:click="{() => switchToDraw()}"
						aria-label="{$t('signature.draw')}"
						transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-pen"></i>
					</button>
				{/if}
			{:else if !$admin}
				<button
					id="new"
					class="dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => newCanvas()}"
					aria-label="{$t('signature.new')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-solid fa-pen"></i>
				</button>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	#signature {
		z-index: 100;
		position: absolute;
		padding: 0;
		margin: 0;
		opacity: 1;
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

	#pad {
		width: 100%;
		height: 100%;
		position: absolute;
	}

	.container {
		position: absolute;
		height: 100%;
		width: calc(100% + 4rem);
		left: 50%;
		transform: translateX(-50%);
		z-index: 800;
		pointer-events: none;
	}

	.overlay {
		position: relative;
		height: 100%;
		width: 100%;
		pointer-events: none;
	}

	.overlay button {
		border: none;
		font-size: 1rem;
		z-index: 250;
		width: 3rem;
		height: 3rem;
		border-radius: 100%;
		transition: opacity 500ms ease-in-out;
		position: absolute;
	}

	.dark {
		filter: brightness(100) brightness(0.87);
	}

	#next,
	#prev,
	#new,
	#save,
	#erase,
	#draw {
		pointer-events: all;

		&:hover {
			cursor: pointer;
		}

		&:active {
			background: var(--c-bg-active);
		}
	}

	#clear {
		pointer-events: all;
		&:hover {
			cursor: pointer;
		}
	}

	#next,
	#prev {
		width: 42px;
		height: 42px;
		top: 50%;
		transform: translateY(-50%);
	}

	#next {
		right: 0;
	}

	#prev {
		left: 0;
	}

	#new,
	#save {
		right: 0;
		bottom: 2rem;
		width: 64px;
		height: 64px;
		font-size: 20px;
	}

	#clear {
		right: 42px;
		bottom: calc(1.5rem + 2px);
		background: var(--c-secondary);
		width: 38px;
		height: 38px;
		font-size: 14px;
	}

	#erase, #draw {
		right: .5rem;
		bottom: calc(6rem + .5rem);
	}

	:global(#clear .fa-secondary) {
		opacity: 1 !important;
	}

	:global(#clear .fa-primary) {
		transform: translateY(-5%);
		transition: transform 200ms;
		transform-origin: right;
	}

	:global(#clear:hover .fa-primary) {
		transform: translateY(-10%) rotate(12deg);
	}

</style>
