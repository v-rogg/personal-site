<script lang="ts">
	import { onMount } from "svelte";
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
	import { loadDelta } from "$lib/components/sections/EyeCatcher/signature.helpers";
	import { sendClientEvent } from "$lib/posthog";
	import type { Signature, SignatureData } from "$drizzle/types";

	let canvas: HTMLCanvasElement;
	let pad: HTMLDivElement;
	let signaturePad: SignaturePad;
	let currentSignature: Signature;
	let drawModeActive = false;
	let eraseModeActive = false;

	function clearCanvas(sendEvent = true) {
		switchToDraw(sendEvent)
		if (sendEvent) sendClientEvent('signature-clear-canvas', document);
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

	async function loadSignature(delta) {
		drawModeActive = false;
		await loadDelta(delta);
		sendClientEvent('signature-load-delta', document);
	}

	function resizeCanvas() {
		canvas.width = pad.offsetWidth;
		canvas.height = pad.offsetHeight;

		clearCanvas(false);

		if ($currentSignatureStore) {
			const currentSignature = uncenterSignature($currentSignatureStore.signature);
			signaturePad.fromData(currentSignature);
		}
	}

	function newCanvas() {
		drawModeActive = true;
		switchToDraw(false);

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

	function switchToDraw(sendEvent = true) {
		eraseModeActive = false;
		signaturePad.compositeOperation = 'source-over';
		signaturePad.minWidth = 0.5
		signaturePad.maxWidth = 2.5
		if (sendEvent) sendClientEvent('signature-switch-to-draw', document);
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
</script>

<div id="pad">
	<canvas id="signature" bind:this="{canvas}" class:dark="{$darkMode}"></canvas>
	<div class="container container-tight">
		<div class="overlay">
			{#if !drawModeActive && $signatureRefsStore}
				{#if $signatureRefsStore.length - $refIndexStore - 1 > 0}
					<button
						id="next"
						class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
						on:click="{() => loadSignature(1)}"
						aria-label="{$t('signature.next')}"
						transition:fade="{{ duration: 150 }}">
						<i class="fa-solid fa-angle-right"></i>
					</button>
				{/if}
				{#if $refIndexStore > 0}
					<button
						id="prev"
						class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
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

								switchToDraw(false);
								drawModeActive = false;
								signaturePad.off();
							}
						};
					}}"
					style="pointer-events: all">
					<button id="save" class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700" aria-label="{$t('signature.save')}" transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-floppy-disk"></i>
					</button>
				</form>
				<button
					id="clear"
					class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => clearCanvas()}"
					aria-label="{$t('signature.clear')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-solid fa-trash"></i>
				</button>
				{#if !eraseModeActive}
				<button
					id="erase"
					class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => switchToErase()}"
					aria-label="{$t('signature.erase')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-solid fa-eraser"></i>
				</button>
				{:else}
					<button
						id="draw"
						class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
						on:click="{() => switchToDraw()}"
						aria-label="{$t('signature.draw')}"
						transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-pen"></i>
					</button>
				{/if}
			{:else if !$admin}
				<button
					id="new"
					class="bg-white-500 hover:bg-white-700 dark:bg-blue-900 dark:hover:bg-blue-700"
					on:click="{() => newCanvas()}"
					aria-label="{$t('signature.new')}"
					transition:fade="{{ duration: 250 }}">
					<span class="text-2xl z-50 absolute w-max -top-10 right-6 text-right font-handwriting -rotate-3 pointer-events-none">
						{$t("signature.tutorial")}
						<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="absolute right-14 translate-y-2 -scale-x-150 scale-150 -rotate-45" xmlns="http://www.w3.org/2000/svg">
							<path
								d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
								fill="var(--c-black)"></path>
						</svg>
					</span>
					<i class="fa-solid fa-pen"></i>
				</button>
			{/if}
		</div>
	</div>
</div>

<style lang="postcss">

	#signature {
		z-index: 100;
		position: absolute;
		padding: 0;
		margin: 0;
		opacity: 1;
		//animation: 250ms in ease-in-out;
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
