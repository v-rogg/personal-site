<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import iro from "@jaames/iro";
	import { onMount } from "svelte";
	import { expoInOut } from "svelte/easing";
	import { blur, fade } from "svelte/transition";
	import { createDialog, createSlider } from "@melt-ui/svelte";
	import { enhance } from "$app/forms";
	import { page } from "$app/state";
	import { addToast } from "$lib/components/globals/Toaster.svelte";
	import { goto } from "$app/navigation";
	import posthog from "posthog-js";

	let { signatures = $bindable(), editMode = $bindable(), closeEditMode } = $props();

	const Tools = {
		pen: 1,
		eraser: 2
	};

	let canvas: HTMLCanvasElement;
	let box: HTMLDivElement | undefined = $state();
	let signaturePad: SignaturePad;
	let tool: number = $state(0);
	let history: PointGroup[] = $state([]);
	let colorPicker = $state(false);
	let penColor = $state("#0f1418");
	let picker = null;
	let signatureSentId: string | undefined = $state(undefined);
	let signatureFormSent = $state(false);

	const {
		elements: { overlay, content, title, description, close, portalled },
		states: { open }
	} = createDialog({
		role: "alertdialog",
		closeOnOutsideClick: false,
		forceVisible: true
	});

	const {
		elements: { root, range, thumbs },
		states: { value: penWidthMultiplicator }
	} = createSlider({
		defaultValue: [1],
		min: 0.5,
		max: 4,
		step: 0.1
	});

	penWidthMultiplicator.subscribe((v) => {
		if (tool === Tools.pen) {
			signaturePad.minWidth = 0.5 * v[0];
			signaturePad.maxWidth = 2.5 * v[0];
		}
	});

	function hslToHex(h: number, s: number, l: number) {
		l /= 100;
		const a = (s * Math.min(l, 1 - l)) / 100;
		const f = (n: number) => {
			const k = (n + h / 30) % 12;
			const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
			return Math.round(255 * color)
				.toString(16)
				.padStart(2, "0");
		};
		return `#${f(0)}${f(8)}${f(4)}`;
	}

	function resizeCanvas() {
		if (canvas && box) {
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;

			signaturePad.fromData(uncenterSignature(history));
		}
	}

	function undo() {
		history.pop();
		signaturePad.fromData(uncenterSignature(history));
		if (colorPicker) posthog.capture("click.signatures.colorPicker.close", { color: penColor });
		colorPicker = false;
		posthog.capture("click.signatures.editor.undo");
	}

	function clear() {
		history = [];
		signaturePad.clear();
		if (colorPicker) posthog.capture("click.signatures.colorPicker.close", { color: penColor });
		colorPicker = false;
		posthog.capture("click.signatures.editor.clear");
	}

	function enablePen() {
		if (tool !== Tools.pen) {
			tool = Tools.pen;
			signaturePad.compositeOperation = "source-over";
			signaturePad.minWidth = 0.5 * $penWidthMultiplicator[0];
			signaturePad.maxWidth = 2.5 * $penWidthMultiplicator[0];
			posthog.capture("click.signatures.editor.pen");
		} else {
			colorPicker = !colorPicker;
			if (colorPicker) {
				posthog.capture("click.signatures.colorPicker.open");
			} else {
				posthog.capture("click.signatures.colorPicker.close", { color: penColor });
			}
		}
	}

	function enabledErase() {
		tool = Tools.eraser;
		signaturePad.compositeOperation = "destination-out";
		signaturePad.minWidth = 4;
		signaturePad.maxWidth = 4;
		if (colorPicker) posthog.capture("click.signatures.colorPicker.close", { color: penColor });
		colorPicker = false;
		posthog.capture("click.signatures.editor.eraser");
	}

	function centerSignature(data: PointGroup[]): PointGroup[] {
		if (box) {
			const middleH = box.offsetWidth / 2;
			const bottomV = box.offsetHeight;

			let centeredData = [];

			for (let signature of data) {
				let centeredSignature = JSON.parse(JSON.stringify(signature));

				centeredSignature.points.forEach((point: { x: number; y: number }) => {
					point.x = point.x - middleH;
					point.y = point.y - bottomV;
				});

				centeredData.push(centeredSignature);
			}

			return centeredData;
		}
		return data;
	}

	function uncenterSignature(data: PointGroup[]): PointGroup[] {
		if (box) {
			const middleH = box.offsetWidth / 2;
			const bottomV = box.offsetHeight;

			let centeredData = [];

			for (let signature of data) {
				let centeredSignature = JSON.parse(JSON.stringify(signature));

				centeredSignature.points.forEach((point: { x: number; y: number }) => {
					point.x = point.x + middleH;
					point.y = point.y + bottomV;
				});

				centeredData.push(centeredSignature);
			}

			return centeredData;
		}
		return data;
	}

	function toClipboard(text: string) {
		navigator.clipboard.writeText(text);
		addToast({
			data: {
				title: "In die Zwischenablage kopiert",
				description: "fa-solid fa-copy"
			}
		});
	}

	onMount(async () => {
		if (canvas && box) {
			penColor = hslToHex(Math.random() * 360, 100, 50);
			signaturePad = new SignaturePad(canvas);
			signaturePad.on();
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;
			enablePen();

			signaturePad.addEventListener("endStroke", () => {
				let data = centerSignature([...signaturePad.toData()]);
				// let data = signaturePad.toData();
				history.push(data[data.length - 1]);
			});

			signaturePad.addEventListener("beginStroke", () => {
				signaturePad.penColor = penColor;
				if (colorPicker) posthog.capture("click.signatures.colorPicker.close", { color: penColor });
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
	class="absolute left-0 top-0 z-40 m-0 cursor-crosshair p-0"
	width="1536"
	height="650"
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
></canvas>
<div
	class="pointer-events-none relative z-50 h-full w-full"
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
>
	<div
		class="pointer-events-auto absolute flex flex-col rounded-xl bg-white max-sm:-bottom-16 max-sm:left-10 max-sm:flex-row sm:right-10 sm:top-[50%] sm:translate-y-[-50%]"
	>
		<button
			class="relative size-12 bg-white transition hover:bg-white-700 active:bg-white-600 max-sm:rounded-l-xl sm:rounded-t-xl"
			class:active={tool == Tools.pen}
			aria-label="Stift"
			onclick={enablePen}
		>
			<i class="fa-solid fa-pen"> </i>
			<i class="fa-solid fa-circle absolute bottom-1 right-1" style="color: {penColor}"> </i>
			<i class="fa-regular fa-circle absolute bottom-1 right-1 text-white"> </i>
			<span
				class="absolute -left-12 -top-[82px] right-full w-max rotate-6 rounded-lg border-4 border-white bg-white-700 px-2 py-1 text-center font-[450] leading-tight max-sm:hidden"
			>
				<span class="relative z-10 text-black" style="">
					Jetzt auch<br />
					in <span class="color-animate font-bold">Farbe</span>
				</span>
			</span>
			<span
				class="text-outline pointer-events-none absolute bottom-full right-10 -translate-y-1 -rotate-6 -scale-x-100 max-sm:hidden"
			>
				<svg
					width="20"
					viewBox="0 0 67 32"
					version="1.1"
					xmlns="http://www.w3.org/2000/svg"
					style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
					class="-translate-y-[2.5px] translate-x-[4px] -rotate-[80deg] -scale-x-100"
					><path
						d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
					></path></svg
				>
			</span>
		</button>
		<button
			class="size-12 bg-white transition hover:bg-white-700 active:bg-white-600 max-sm:bg-white-600"
			class:active={tool == Tools.eraser}
			aria-label="Radiergummi"
			onclick={enabledErase}
		>
			<i class="fa-solid fa-eraser"> </i>
		</button>

		<div
			class="absolute rounded-2xl border-2 border-white-700 bg-white p-2 max-sm:bottom-full max-sm:left-0 max-sm:mb-2 sm:right-full sm:top-0 sm:mr-2"
			class:hidden={!colorPicker}
		>
			<div id="picker"></div>
			<div id="penWidth" class="relative mb-1 ml-3 mr-2 mt-4">
				<span {...$root} use:root class="relative mt-1 flex h-[20px] w-[80px] items-center">
					<span {...$range} use:range class="absolute h-1 w-[84px] rounded-full bg-white-600"></span>
					<span
						{...$thumbs[0]}
						use:thumbs
						class="absolute size-3 rounded-full outline-none"
						style="{$thumbs[0].style}; transform:  translateX(-50%) scale({$penWidthMultiplicator[0] *
							0.6}); background: {penColor}"
					></span>
				</span>
			</div>
		</div>

		<hr class="h-[2px] bg-white-700 text-transparent" />
		<button
			class="size-12 bg-white transition hover:bg-white-700 active:bg-white-600 disabled:cursor-not-allowed disabled:bg-white-600 disabled:hover:bg-white-600 disabled:active:bg-white-600 max-sm:bg-white-600 disabled:max-sm:bg-white"
			aria-label="Rückgängig"
			onclick={undo}
			disabled={history.length === 0}
		>
			<i class="fa-solid fa-undo"> </i>
		</button>
		<button
			class="size-12 bg-white transition hover:bg-white-700 active:bg-white-600 disabled:cursor-not-allowed disabled:bg-white-600 disabled:hover:bg-white-600 disabled:active:bg-white-600 max-sm:rounded-r-xl max-sm:bg-white-600 disabled:max-sm:bg-white sm:rounded-b-xl"
			aria-label="Löschen"
			disabled={history.length === 0}
			onclick={clear}
		>
			<i class="fa-solid fa-trash"> </i>
		</button>
	</div>
	<button
		class="pointer-events-auto absolute bottom-8 right-10 size-12 rounded-full text-black transition hover:bg-white-700 active:bg-white-600 disabled:cursor-not-allowed disabled:bg-white-600 disabled:hover:bg-white-600 disabled:active:bg-white-600 max-sm:-bottom-16 max-sm:bg-white-600 max-sm:disabled:bg-white sm:bg-white"
		aria-label="Speichern"
		onclick={() => {
			$open = true;
			posthog.capture("click.signatures.editor.openSaveDialog");
		}}
		disabled={history.length === 0}
	>
		<i class="fa-solid fa-floppy-disk"></i>
	</button>
	<button
		class="pointer-events-auto absolute right-10 top-6 size-12 rounded-full text-black transition hover:bg-white-700 active:bg-white-600"
		aria-label="Schließen"
		onclick={closeEditMode}
	>
		<i class="fa-solid fa-xmark-large"></i>
	</button>
</div>

{#if $open}
	<div class="" {...$portalled} use:portalled>
		<div
			{...$overlay}
			use:overlay
			class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
			transition:fade={{ duration: 250 }}
		></div>
		<div
			class="fixed left-1/2 top-1/2 z-50 w-full max-w-[450px]
            -translate-x-1/2 -translate-y-1/2 bg-white p-6 shadow-lg
            min-[450px]:w-[90vw] min-[450px]:rounded-xl"
			{...$content}
			transition:fade={{ duration: 250 }}
			use:content
		>
			{#if signatureSentId === undefined}
				<h2 {...$title} use:title class="m-0 -mt-2 pr-5 text-xl font-semibold text-black">Zeichnung einreichen</h2>
				<p {...$description} use:description class="mb-8 mt-2">
					Danke für das Erstellen deiner Zeichnung. Ich freue mich sehr darüber!
				</p>

				<form
					method="POST"
					action="/api/actions/signatures?/create"
					use:enhance={({ formData }) => {
						formData.set("signature", JSON.stringify(history));
						signatureFormSent = true;

						return ({ result }) => {
							if (result.status === 200) {
								// @ts-expect-error data
								signatureSentId = result.data.id;
								posthog.identify(signatureSentId, { id: signatureSentId, email: formData.get("email") });
								signatureFormSent = false;
								signatures.unshift({
									id: signatureSentId,
									approved: null
								});
							}
						};
					}}
				>
					<fieldset class="mb-8 flex flex-col gap-4">
						<label for="name">Bitte gibt deinem Kunstwerk noch einen Namen:</label>
						<input
							class="inline-flex h-8 w-full flex-1 items-center justify-center
                    rounded-lg bg-white-700 px-4 py-2 text-black placeholder:font-normal placeholder:text-skin-500"
							id="name"
							name="name"
							required
							placeholder="Name deiner Zeichnung"
						/>
					</fieldset>
					<fieldset class="mb-8 flex flex-col gap-4">
						<label for="email" class="text-sm"
							>Optional kannst du auch eine E-Mail hinterlegen um über die Veröffentlichung deiner Zeichnung informiert
							zu werden:</label
						>
						<input
							class="inline-flex w-full flex-1 items-center justify-center
                    rounded-lg bg-white-700 px-4 py-2 text-black placeholder:font-normal placeholder:text-skin-500"
							id="email"
							name="email"
							type="email"
							placeholder="E-Mail (optional)"
						/>
					</fieldset>
					<div class="mt-6 flex flex-col justify-start gap-4 min-[450px]:flex-row-reverse">
						<button class="rounded-lg bg-black px-4 py-2 text-white transition hover:bg-grey-800" type="submit">
							Zeichnung einreichen
							{#if signatureFormSent}
								<i class="fa-solid fa-spinner-third fa-spin ml-2"></i>
							{:else}
								<i class="fa-solid fa-cloud-arrow-up ml-2"></i>
							{/if}
						</button>
						<button {...$close} use:close class="py-w rounded-lg px-4 text-grey-800 transition hover:bg-grey-600">
							Abbrechen
						</button>
					</div>
					<button
						{...$close}
						use:close
						aria-label="close"
						class="absolute right-4 top-4 inline-flex size-8
                appearance-none items-center justify-center rounded-full
                transition hover:bg-white-700"
					>
						<i class="fa-solid fa-xmark-large size-4"> </i></button
					>
				</form>
			{:else}
				{@const signatureUrl = `${page.url.origin}/?s=${signatureSentId}`}
				{@const baseText =
					"Ich%20habe%20mich%20verk%C3%BCnstelt%20und%20Valentin%20angemalt.%20Schau%20dir%20mein%20Meisterwerk%20an%20unter%3A%20"}
				<h2 {...$title} use:title class="m-0 -mt-2 pr-5 text-xl font-semibold text-black">Zeichnung eingereicht</h2>
				<p {...$description} use:description class="mb-8 mt-2 text-balance">
					Deine Zeichnung wurde eingereicht.<br /> Ich werde sie im Laufe der nächsten Tage freigeben.
				</p>
				<fieldset>
					<label for="url">Teile dein Meisterwerk auch schon vor Freigabe:</label>
					<button
						class="mt-2 flex w-full items-center justify-between rounded-lg bg-white-600 px-4 py-2 text-sm hover:bg-white-700 active:bg-white-500"
						id="url"
						onclick={() => {
							toClipboard(signatureUrl);
							posthog.capture("click.signatures.editor.clipboard.text");
						}}
					>
						{signatureUrl}
						<i class="fa-regular fa-copy text-grey-800"> </i>
					</button>
				</fieldset>
				<fieldset class="mt-4 flex justify-center gap-4">
					<a
						aria-label="Auf Whatsapp teilen"
						href="https://wa.me/?text={baseText}{signatureUrl}"
						target="_blank"
						rel="noopener noreferrer"
						class="flex size-14 items-center justify-center rounded-lg bg-white-600 text-3xl text-black hover:bg-white-700"
					>
						<i class="fa-brands fa-whatsapp"></i>
					</a>
					<a
						aria-label="Auf Signal teilen"
						href="sms:?&body={baseText}{signatureUrl}"
						rel="noopener noreferrer"
						class="flex size-14 items-center justify-center rounded-lg bg-white-600 text-3xl text-black hover:bg-white-700"
					>
						<i class="fa-brands fa-signal-messenger"></i>
					</a>
				</fieldset>
				<div class="mt-6 flex justify-start gap-4 min-[450px]:flex-row-reverse">
					<button
						class="rounded-lg bg-black px-4 py-2 text-white transition hover:bg-grey-800"
						onclick={() => {
							toClipboard(signatureUrl);
							posthog.capture("click.signatures.editor.clipboard.button");
						}}
						type="submit"
					>
						Link kopieren
					</button>
					<button
						onclick={() => {
							$open = false;
							goto(signatureUrl, { replaceState: true, invalidateAll: true });
							closeEditMode();
						}}
						class="py-w rounded-lg px-4 text-grey-800 transition hover:bg-grey-600"
					>
						Schließen
					</button>
				</div>
				<button
					onclick={() => {
						$open = false;
						goto(signatureUrl, { replaceState: true, invalidateAll: true });
						closeEditMode();
					}}
					aria-label="close"
					class="absolute right-4 top-4 inline-flex size-8
                appearance-none items-center justify-center rounded-full
                transition hover:bg-white-700"
				>
					<i class="fa-solid fa-xmark-large size-4"> </i></button
				>
			{/if}
		</div>
	</div>
{/if}

<style lang="postcss">
	.color-animate {
		animation: colorchange 20s infinite alternate;
	}

	@keyframes colorchange {
		0% {
			color: blue;
		}
		10% {
			color: #8e44ad;
		}
		20% {
			color: #1abc9c;
		}
		30% {
			color: #d35400;
		}
		40% {
			color: blue;
		}
		50% {
			color: #34495e;
		}
		60% {
			color: blue;
		}
		70% {
			color: #2980b9;
		}
		80% {
			color: #f1c40f;
		}
		90% {
			color: #2980b9;
		}
		100% {
			color: pink;
		}
	}

	.active {
		@apply bg-black;
		@apply text-white;
		@apply hover:bg-grey-800;
	}

	.inactive {
		@apply bg-white-600;
	}
</style>
