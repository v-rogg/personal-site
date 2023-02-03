<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import SignaturePad from "signature_pad";
	import {
		currentSignatureStore,
		darkMode,
		refIndexStore,
		signatureRefsStore,
		admin,
		identifier,
		telemetry
	} from "../stores";
	import { t } from "$lib/_i18n";
	import xss from "xss";
	import type { signatureData } from "$lib/types";

	let canvas: HTMLCanvasElement;

	let pad: HTMLDivElement;
	let signaturePad: SignaturePad;

	let currentSignature: signatureData;

	let empty = true;

	let drawModeActive = false;

	function clearCanvas() {
		signaturePad.clear();
		empty = signaturePad.isEmpty();
	}

	function centerSignature(data) {
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
		drawModeActive = false;
		await fetch(
			`${$page.url.origin}/_api/signature?ref=${
				$signatureRefsStore[$refIndexStore + delta]["@ref"].id
			}`,
			{
				method: "GET"
			}
		)
			.then((res) => res.json())
			.then((json) => {
				$refIndexStore += delta;
				$currentSignatureStore = json;
				return json;
			});
	}

	function resizeCanvas() {
		canvas.width = pad.offsetWidth;
		canvas.height = pad.offsetHeight;

		clearCanvas();

		if ($currentSignatureStore?.data) {
			const currentSignature = uncenterSignature(
				(<signatureData>$currentSignatureStore).data.signature
			);
			signaturePad.fromData(currentSignature);
		}
	}

	async function saveCanvas() {
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore
		let name = xss(prompt($t("signature.prompt"), ""));

		const json = JSON.stringify({
			name,
			identifier: $identifier,
			signature: centerSignature(signaturePad.toData())
		});

		const newSignature = await fetch(`${$page.url.origin}/_api/signature`, {
			method: "POST",
			body: json
		})
			.then((res) => res.json())
			.then((json) => {
				return json[0];
			});

		$telemetry.signal({
			type: "signatureCreated",
			signature: newSignature.ref["@ref"].id,
			time: Date.now(),
			appVersion: "1.0.0"
		});

		let signatureRefs = $signatureRefsStore;
		signatureRefs.push(newSignature.ref);
		$signatureRefsStore = signatureRefs;
		$currentSignatureStore = newSignature;
		$refIndexStore = signatureRefs.length - 1;

		drawModeActive = false;
		signaturePad.off();
		// clearCanvas();
	}

	function newCanvas() {
		drawModeActive = true;

		$currentSignatureStore = {
			ref: null,
			ts: null,
			data: {
				name: null,
				signature: []
			}
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
			currentSignature.data.signature = centerSignature(signaturePad.toData());
			$currentSignatureStore = currentSignature;
			empty = signaturePad.isEmpty();
		});

		window.onresize = resizeCanvas;
		resizeCanvas();

		currentSignatureStore.subscribe((data: signatureData) => {
			if (data != undefined) {
				// console.log(data);
				if (Object.keys(data).length > 0) {
					signaturePad.fromData(uncenterSignature(data.data.signature));
					// signaturePad.fromData(uncenterSignature(currentSignature.data.signature));
					// centeredData = currentSignature;
				}
			}
		});

		/* Rafael Nockmann, 2017-09-10 */
		/* Dieses Skript erkennt horizontale Touchgesten. */

		/* START Optionen */

		// Mindestdauer der Wischgeste in ms.
		var durationMin = 100;

		// Zurückgelegte Mindestdistanz auf der X-Koordinate.
		var distanceTraveledMin = 100;

		// Abweichung vom Kurs.
		// Beispiel: Wenn eine Wischbewegung von links nach rechts erkannt werden soll, der Wert, den die Wischgeste nach oben oder unten verrutschen darf -> Toleranz
		var courceTolerance = 150;

		// Element, das auf Wischgeste geprüft werden soll.

		var startX = null;
		var startY = null;
		var starttime = null;

		swipePad.ontouchstart = function (e) {
			startX = e.changedTouches[0].pageX;
			startY = e.changedTouches[0].pageY;
			starttime = new Date().getTime();
		};

		swipePad.ontouchend = function (e) {
			var endX = e.changedTouches[0].pageX;
			var endY = e.changedTouches[0].pageY;
			var endtime = new Date().getTime();

			verifyHorizontalSwipe(endX, endY, endtime);
		};

		function verifyHorizontalSwipe(endX, endY, endtime) {
			// Dauer der Touchgeste in ms:
			var duration = endtime - starttime;
			// Distanz der Touchgeste in Pixel:
			var distanceTraveled = endX - startX;
			// Abweichung der Touchgeste nach oben oder unten in Pixel:
			var deviation = Math.abs(endY - startY);

			if (
				duration >= durationMin &&
				Math.abs(distanceTraveled) >= distanceTraveledMin &&
				deviation <= courceTolerance
			) {
				if (Math.sign(distanceTraveled) == 1) {
					if ($refIndexStore > 0) loadDelta(-1);
				} else if (Math.sign(distanceTraveled) == -1) {
					if ($signatureRefsStore.length - $refIndexStore - 1 > 0) loadDelta(1);
				}
			} else {
				// alert(
				// 	"Keine Wischgeste erkannt\n" +
				// 		"Dauer: " +
				// 		duration +
				// 		"\nZurückgelegte Strecke: " +
				// 		Math.abs(distanceTraveled) +
				// 		"\nKursabweichung: " +
				// 		deviation
				// );
			}
		}

		/* Touchgeste soll nicht als Zoom oder Scrollen erkannt werden, wenn der User im entsprechenden Element auf der Seite eine Touchgeste ausführt. */
		swipePad.ontouchmove = function (e) {
			e.preventDefault();
		};

		/* Maussimulation */

		swipePad.onmousedown = function (e) {
			startX = e.pageX;
			startY = e.pageY;
			starttime = new Date().getTime();
		};

		swipePad.onmouseup = function (e) {
			var endX = e.pageX;
			var endY = e.pageY;
			var endtime = new Date().getTime();

			verifyHorizontalSwipe(endX, endY, endtime);
		};
	});

	let swipePad;
</script>

<div id="pad">
	<canvas id="signature" bind:this="{canvas}" class:dark="{$darkMode}"></canvas>
	<div class="swipe" bind:this="{swipePad}"></div>
	<div class="container">
		<div class="overlay">
			{#if !drawModeActive && $signatureRefsStore}
				{#if $signatureRefsStore.length - $refIndexStore - 1 > 0}
					<button id="next" on:click="{() => loadDelta(1)}" aria-label="{$t('signature.next')}">
						<i class="fa-solid fa-angle-right"></i>
					</button>
				{/if}
				{#if $refIndexStore > 0}
					<button id="prev" on:click="{() => loadDelta(-1)}" aria-label="{$t('signature.prev')}">
						<i class="fa-solid fa-angle-left"></i>
					</button>
				{/if}
			{/if}

			{#if drawModeActive}
				<button id="save" on:click="{() => saveCanvas()}" aria-label="{$t('signature.save')}">
					<i class="fa-solid fa-floppy-disk"></i>
				</button>
				<button id="clear" on:click="{() => clearCanvas()}" aria-label="{$t('signature.clear')}">
					<i class="fa-duotone fa-trash"></i>
				</button>
				<!--      <button id="exit" on:click={() => clearCanvas()}>-->
				<!--        <i class="fa-solid fa-xmark-large"></i>-->
				<!--      </button>-->
			{:else if !$admin}
				<button id="new" on:click="{() => newCanvas()}" aria-label="{$t('signature.new')}">
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
		/*padding: 0;*/
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
		/*padding: 0;*/
		/*margin: auto;*/
		/*padding: 0 2rem;*/
		padding: 0 2rem;
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
	#clear,
	#new {
		pointer-events: all;

		&:hover {
			background: var(--c-grey-10);
			cursor: pointer;
		}

		&:active {
			background: var(--c-grey-30);
			cursor: default;
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
		bottom: 2.5rem;
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
