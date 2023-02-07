<script lang="ts">
	import { onMount } from "svelte";
	import SignaturePad from "signature_pad";
	import {
		currentSignatureStore,
		darkMode,
		refIndexStore,
		signatureRefsStore,
		admin,
		identifier
	} from "$lib/stores";
	import { t } from "$lib/_i18n";
	import xss from "xss";
	import type { Signature, SignatureData } from "$lib/fauna-gql/schema";
	import { gql } from "graphql-request";
	import { client } from "$lib/fauna-gql/public.client";
	import { tweened } from "svelte/motion";
	import { enhance } from "$app/forms";
	import { fade } from "svelte/transition";

	let canvas: HTMLCanvasElement;
	let pad: HTMLDivElement;
	let signaturePad: SignaturePad;
	let currentSignature: Signature;
	let empty = true;
	let drawModeActive = false;
	let progressBarTween = tweened(0, { duration: 400 });
	progressBarTween.subscribe((n) => {
		if (n >= 100) {
			progressBarTween.update(() => 0, { duration: 0 });
		}
	});

	function clearCanvas() {
		signaturePad.clear();
		empty = signaturePad.isEmpty();
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

	async function loadDelta(delta) {
		const deltaIndex = $refIndexStore + delta;

		if (deltaIndex < 0 || deltaIndex >= $signatureRefsStore.length) return;

		drawModeActive = false;

		const promises = await Promise.all([
			progressBarTween.update(() => 80, { duration: 200 }),
			client
				.request(
					gql`
						query ($id: ID!) {
							findSignatureByID(id: $id) {
								_id
								_ts
								name
								status
								ts_created
								signature {
									penColor
									minWidth
									maxWidth
									dotSize
									points {
										x
										y
										time
										pressure
									}
								}
							}
						}
					`,
					{ id: $signatureRefsStore[deltaIndex]._id }
				)
				.then((res) => res.findSignatureByID)
				.catch((err) => {
					console.log(err);
				})
		]);

		$refIndexStore = deltaIndex;
		$currentSignatureStore = promises[1];
		await progressBarTween.update(() => 100, { duration: 100 });
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

		$currentSignatureStore = <Signature>{
			name: null,
			signature: []
		};

		signaturePad.on();
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
			empty = signaturePad.isEmpty();
		});

		window.onresize = resizeCanvas;
		resizeCanvas();

		currentSignatureStore.subscribe((data: Signature) => {
			if (data !== <Signature>undefined) {
				// console.log(data);
				if (Object.keys(data).length > 0) {
					signaturePad.fromData(uncenterSignature(data.signature));
					// signaturePad.fromData(uncenterSignature(currentSignature.data.signature));
					// centeredData = currentSignature;
				}
			} else {
				signaturePad.clear();
			}
		});

		// <!--/* Rafael Nockmann, 2017-09-10 */-->
		// <!--/* Dieses Skript erkennt horizontale Touchgesten. */-->
		//
		// <!--/* START Optionen */-->
		//
		// <!--// Mindestdauer der Wischgeste in ms.-->
		// <!--var durationMin = 100;-->
		//
		// <!--// Zurückgelegte Mindestdistanz auf der X-Koordinate.-->
		// <!--var distanceTraveledMin = 100;-->
		//
		// <!--// Abweichung vom Kurs.-->
		// <!--// Beispiel: Wenn eine Wischbewegung von links nach rechts erkannt werden soll, der Wert, den die Wischgeste nach oben oder unten verrutschen darf -> Toleranz-->
		// <!--var courceTolerance = 150;-->
		//
		// <!--// Element, das auf Wischgeste geprüft werden soll.-->
		//
		// <!--var startX = null;-->
		// <!--var startY = null;-->
		// <!--var starttime = null;-->
		//
		// <!--swipePad.ontouchstart = function (e) {-->
		// <!--	startX = e.changedTouches[0].pageX;-->
		// <!--	startY = e.changedTouches[0].pageY;-->
		// <!--	starttime = new Date().getTime();-->
		// <!--};-->
		//
		// <!--swipePad.ontouchend = function (e) {-->
		// <!--	var endX = e.changedTouches[0].pageX;-->
		// <!--	var endY = e.changedTouches[0].pageY;-->
		// <!--	var endtime = new Date().getTime();-->
		//
		// <!--	verifyHorizontalSwipe(endX, endY, endtime);-->
		// <!--};-->
		//
		// <!--function verifyHorizontalSwipe(endX, endY, endtime) {-->
		// <!--	// Dauer der Touchgeste in ms:-->
		// <!--	var duration = endtime - starttime;-->
		// <!--	// Distanz der Touchgeste in Pixel:-->
		// <!--	var distanceTraveled = endX - startX;-->
		// <!--	// Abweichung der Touchgeste nach oben oder unten in Pixel:-->
		// <!--	var deviation = Math.abs(endY - startY);-->
		//
		// <!--	if (-->
		// <!--		duration >= durationMin &&-->
		// <!--		Math.abs(distanceTraveled) >= distanceTraveledMin &&-->
		// <!--		deviation <= courceTolerance-->
		// <!--	) {-->
		// <!--		if (Math.sign(distanceTraveled) == 1) {-->
		// <!--			if ($refIndexStore > 0) loadDelta(-1);-->
		// <!--		} else if (Math.sign(distanceTraveled) == -1) {-->
		// <!--			if ($signatureRefsStore.length - $refIndexStore - 1 > 0) loadDelta(1);-->
		// <!--		}-->
		// <!--	} else {-->
		// <!--		// alert(-->
		// <!--		// 	"Keine Wischgeste erkannt\n" +-->
		// <!--		// 		"Dauer: " +-->
		// <!--		// 		duration +-->
		// <!--		// 		"\nZurückgelegte Strecke: " +-->
		// <!--		// 		Math.abs(distanceTraveled) +-->
		// <!--		// 		"\nKursabweichung: " +-->
		// <!--		// 		deviation-->
		// <!--		// );-->
		// <!--	}-->
		// <!--}-->
		//
		// <!--/* Touchgeste soll nicht als Zoom oder Scrollen erkannt werden, wenn der User im entsprechenden Element auf der Seite eine Touchgeste ausführt. */-->
		// <!--swipePad.ontouchmove = function (e) {-->
		// <!--	e.preventDefault();-->
		// <!--};-->
		//
		// <!--/* Maussimulation */-->
		//
		// <!--swipePad.onmousedown = function (e) {-->
		// <!--	startX = e.pageX;-->
		// <!--	startY = e.pageY;-->
		// <!--	starttime = new Date().getTime();-->
		// <!--};-->
		// swipePad.onmouseup = function (e) {
		// 	var endX = e.pageX;
		// 	var endY = e.pageY;
		// 	var endtime = new Date().getTime();
		//
		// 	verifyHorizontalSwipe(endX, endY, endtime);
		// };
	});

	// let swipePad;
</script>

<div id="pad">
	<canvas id="signature" bind:this="{canvas}" class:dark="{$darkMode}"></canvas>
	<!--	<div class="swipe" bind:this="{swipePad}"></div>-->
	<div class="container">
		<div class="overlay">
			{#if !drawModeActive && $signatureRefsStore}
				{#if $signatureRefsStore.length - $refIndexStore - 1 > 0}
					<button
						id="next"
						on:click="{() => loadDelta(1)}"
						aria-label="{$t('signature.next')}"
						transition:fade="{{ duration: 150 }}">
						<i class="fa-solid fa-angle-right"></i>
					</button>
				{/if}
				{#if $refIndexStore > 0}
					<button
						id="prev"
						on:click="{() => loadDelta(-1)}"
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
					use:enhance="{({ data, cancel }) => {
						const signature = centerSignature(signaturePad.toData());
						if (signature.length === 0) {
							cancel();
						} else {
							const name = xss(prompt($t('signature.prompt'), ''));
							if (!name) cancel();

							const json = JSON.stringify({
								name: name,
								user_identifier: $identifier,
								signature
							});

							data.set('data', json);
						}

						return async ({ result }) => {
							if (result.status === 200) {
								$currentSignatureStore.status = result.data.status;
								let signatureRefs = $signatureRefsStore;
								signatureRefs.push(result.data._id);
								$signatureRefsStore = signatureRefs;
								$currentSignatureStore = result.data;
								$refIndexStore = signatureRefs.length - 1;

								drawModeActive = false;
								signaturePad.off();
							}
						};
					}}"
					style="pointer-events: all">
					<button
						id="save"
						aria-label="{$t('signature.save')}"
						transition:fade="{{ duration: 250 }}">
						<i class="fa-solid fa-floppy-disk"></i>
					</button>
				</form>
				<button
					id="clear"
					on:click="{() => clearCanvas()}"
					aria-label="{$t('signature.clear')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-duotone fa-trash"></i>
				</button>
				<!--      <button id="exit" on:click={() => clearCanvas()}>-->
				<!--        <i class="fa-solid fa-xmark-large"></i>-->
				<!--      </button>-->
			{:else if !$admin}
				<button
					id="new"
					on:click="{() => newCanvas()}"
					aria-label="{$t('signature.new')}"
					transition:fade="{{ duration: 250 }}">
					<i class="fa-solid fa-pen"></i>
				</button>
			{/if}
		</div>
	</div>

	{#if $progressBarTween > 0}
		<div
			class="progress-bar"
			role="progressbar"
			aria-valuenow="{$progressBarTween}"
			style="width: {$progressBarTween}%">
		</div>
	{/if}
</div>

<style lang="scss">
	.progress-bar {
		position: absolute;
		bottom: 0;
		height: 2px;
		background: var(--c-primary);
		z-index: 1;
	}

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

	.swipe {
		height: 100%;
		width: 100%;
		position: absolute;
		z-index: 750;
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

	@media (max-width: 540px) {
		.container {
			width: calc(100% + 2rem);
		}
	}

	.overlay {
		position: relative;
		height: 100%;
		width: 100%;
		pointer-events: none;
	}

	.overlay button {
		background: var(--c-bg);
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
	#save {
		pointer-events: all;

		&:hover {
			background: var(--c-bg-hover);
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

	/*#exit {*/
	/*  right: -2rem;*/
	/*  bottom: calc(1.5rem + 2px);*/
	/*  background: none;*/
	/*  width: 38px;*/
	/*  height: 38px;*/
	/*  font-size: 14px;*/
	/*}*/

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

	/*.overlay {*/
	/*  position: absolute;*/
	/*  right: 0;*/
	/*  bottom: 2rem;*/
	/*}*/
</style>
