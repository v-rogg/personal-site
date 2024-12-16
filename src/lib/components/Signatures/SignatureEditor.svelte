<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import iro from "@jaames/iro";
	import { onMount } from "svelte";
	import { expoInOut } from "svelte/easing";
	import { blur } from "svelte/transition";

	let { editMode = $bindable(), closeEditMode } = $props();

	let canvas: HTMLCanvasElement;
	let box: HTMLDivElement | undefined = $state();
	let signaturePad: SignaturePad;

	const Tools = {
		pen: 1,
		eraser: 2
	};

	let tool: number = $state(0);

	let history: PointGroup[] = $state([]);

	let colorPicker = $state(false);
	let penColor = $state("#000000");
	let picker = null;
	function resizeCanvas() {
		if (canvas && box) {
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;
		}
	}

	function undo() {
		history.pop();
		signaturePad.fromData(history);
		colorPicker = false;
	}

	function clear() {
		history = [];
		signaturePad.clear();
		colorPicker = false;
	}

	function enablePen() {
		if (tool !== Tools.pen) {
			tool = Tools.pen;
			signaturePad.compositeOperation = "source-over";
			signaturePad.minWidth = 0.5;
			signaturePad.maxWidth = 2.5;
		} else {
			colorPicker = !colorPicker;
		}
	}

	function enabledErase() {
		tool = Tools.eraser;
		signaturePad.compositeOperation = "destination-out";
		signaturePad.minWidth = 4;
		signaturePad.maxWidth = 4;
		colorPicker = false;
	}

	function storeSignature() {}

	onMount(async () => {
		if (canvas && box) {
			signaturePad = new SignaturePad(canvas);
			signaturePad.on();
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;
			enablePen();

			signaturePad.addEventListener("endStroke", () => {
				let data = signaturePad.toData();
				history.push(data[data.length - 1]);
			});

			signaturePad.addEventListener("beginStroke", () => {
				signaturePad.penColor = penColor;
				colorPicker = false;
			});
		}

		picker = iro.ColorPicker("#picker", {
			width: 100,
			color: penColor,
			margin: 6,
			padding: 2,
			layout: [
				{
					component: iro.ui.Box
				},
				{
					component: iro.ui.Slider,
					options: {
						sliderType: "hue"
					}
				}
			]
		});

		picker.on("color:change", (color: { hexString: string }) => {
			penColor = color.hexString;
		});
	});
</script>

<svelte:window onresize={resizeCanvas} />

<div class="pointer-events-none absolute left-0 top-0 h-full w-full" bind:this={box}></div>
<canvas
	bind:this={canvas}
	class="absolute left-0 top-0 z-40 m-0 p-0"
	width="1536"
	height="650"
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
></canvas>
<div
	class="pointer-events-none relative z-50 h-full w-full"
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
>
	<div class="pointer-events-auto absolute right-10 top-[50%] flex translate-y-[-50%] flex-col rounded-xl bg-white">
		<button
			class="relative size-12 rounded-t-xl transition"
			class:active={tool == Tools.pen}
			aria-label="Stift"
			onclick={enablePen}
		>
			<i class="fa-solid fa-pen"> </i>
			<i class="fa-solid fa-circle absolute bottom-1 right-1" style="color: {penColor}"> </i>
			<i class="fa-regular fa-circle absolute bottom-1 right-1 text-white"> </i>
			<span
				class="text-outline absolute right-full w-max text-center font-hand max-sm:hidden sm:-left-8 sm:-top-16 sm:rotate-6"
			>
				<span class="relative z-10 text-black" style="">
					Jetzt auch<br />
					in Farbe
				</span>
			</span>
			<span
				class="text-outline pointer-events-none absolute bottom-full right-10 -translate-y-1 -rotate-6 -scale-x-100 max-sm:hidden"
			>
				<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
					<path
						d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
						fill="black"
					></path>
				</svg>
			</span>
		</button>
		<button
			class="size-12 transition"
			class:active={tool == Tools.eraser}
			aria-label="Radiergummi"
			onclick={enabledErase}
		>
			<i class="fa-solid fa-eraser"> </i>
		</button>
		<hr class="text-white-700" />

		<div class="absolute right-full top-0 mr-2 rounded-2xl bg-white p-2" class:hidden={!colorPicker}>
			<div id="picker"></div>
		</div>

		<hr class="text-white-700" />
		<button
			class="size-12 bg-white transition duration-500"
			aria-label="Rückgängig"
			onclick={undo}
			disabled={history.length === 0}
			class:inactive={history.length === 0}
		>
			<i class="fa-solid fa-undo"> </i>
		</button>
		<button
			class="size-12 rounded-b-xl bg-white transition duration-500"
			aria-label="Löschen"
			disabled={history.length === 0}
			class:inactive={history.length === 0}
			onclick={clear}
		>
			<i class="fa-solid fa-trash"> </i>
		</button>
	</div>
	<button
		class="pointer-events-auto absolute bottom-8 right-10 size-12 rounded-full bg-white text-black"
		aria-label="Speichern"
		onclick={storeSignature}
	>
		<i class="fa-solid fa-floppy-disk"></i>
	</button>
	<button
		class="pointer-events-auto absolute right-10 top-8 size-12 rounded-full text-black"
		aria-label="Schließen"
		onclick={closeEditMode}
	>
		<i class="fa-solid fa-xmark-large"></i>
	</button>
</div>

<style lang="postcss">
	.active {
		@apply bg-black;
		@apply text-white;
	}

	.inactive {
		@apply bg-white-600;
	}
</style>
