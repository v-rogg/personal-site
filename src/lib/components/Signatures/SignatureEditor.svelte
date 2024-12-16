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

	let tool = $state(Tools.pen);

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
	}

	function clear() {
		history = [];
		signaturePad.clear();
	}

	onMount(async () => {
		if (canvas && box) {
			signaturePad = new SignaturePad(canvas);
			signaturePad.on();
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;

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
		<button class="size-12 rounded-t-xl transition" class:active={tool == Tools.pen} aria-label="Stift">
			<i class="fa-solid fa-pen"> </i>
		</button>
		<button
			class="size-12 transition"
			aria-label="Farbe"
			onclick={() => {
				colorPicker = true;
			}}
		>
			<i class="fa-solid fa-palette" style="color: {penColor}"> </i>
		</button>
		<div class="absolute right-full top-0 mr-2 rounded-2xl bg-white p-2" class:hidden={!colorPicker}>
			<div id="picker"></div>
		</div>

		<button class="size-12 transition" class:active={tool == Tools.eraser} aria-label="Radiergummi">
			<i class="fa-solid fa-eraser"> </i>
		</button>
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
