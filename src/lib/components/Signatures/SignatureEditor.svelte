<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import iro from "@jaames/iro";
	import { onMount } from "svelte";
	import { expoInOut } from "svelte/easing";
	import { blur, fade } from "svelte/transition";
	import { createDialog } from "@melt-ui/svelte";
	import { enhance } from "$app/forms";
	import { page } from "$app/stores";
	import { addToast } from "../globals/Toaster.svelte";
	import { goto } from "$app/navigation";

	let { signatures = $bindable(), editMode = $bindable(), closeEditMode } = $props();

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

	let penColor = $state("#0f1418");
	let picker = null;

	let signatureSentId: string | undefined = $state(undefined);

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

	const {
		elements: { overlay, content, title, description, close, portalled },
		states: { open }
	} = createDialog({
		role: "alertdialog",
		closeOnOutsideClick: false,
		forceVisible: true
	});

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
			class="size-12 bg-white transition hover:bg-white-700 active:bg-white-600 max-sm:bg-white-600"
			class:active={tool == Tools.eraser}
			aria-label="Radiergummi"
			onclick={enabledErase}
		>
			<i class="fa-solid fa-eraser"> </i>
		</button>

		<div class="absolute right-full top-0 mr-2 rounded-2xl bg-white p-2" class:hidden={!colorPicker}>
			<div id="picker"></div>
		</div>

		<hr class="text-white-700" />
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
			class="fixed left-1/2 top-1/2 z-50 w-[90vw]
            max-w-[450px] -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white
            p-6 shadow-lg"
			{...$content}
			transition:fade={{ duration: 250 }}
			use:content
		>
			{#if signatureSentId === undefined}
				<h2 {...$title} use:title class="m-0 -mt-2 text-xl font-semibold text-black">Zeichnung einreichen</h2>
				<p {...$description} use:description class="mb-8 mt-2">
					Danke für das Erstellen deiner Zeichnung. Ich freue mich sehr darüber 🤗
				</p>

				<form
					method="POST"
					action="/api/actions/signatures?/create"
					use:enhance={({ formData }) => {
						formData.set("signature", JSON.stringify(history));

						return ({ result }) => {
							if (result.status === 200) {
								// @ts-expect-error data
								signatureSentId = result.data.id;
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
					<div class="mt-6 flex flex-row-reverse justify-start gap-4">
						<button class="rounded-lg bg-black px-4 py-2 text-white transition hover:bg-grey-800" type="submit">
							Zeichnung einreichen
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
				{@const signatureUrl = `${$page.url.origin}/?s=${signatureSentId}`}
				{@const baseText =
					"Ich%20habe%20mich%20verk%C3%BCnstelt%20und%20Valentin%20angemalt.%20Schau%20dir%20meine%20Meisterwerk%20an%20unter%3A%20"}
				<h2 {...$title} use:title class="m-0 -mt-2 text-xl font-semibold text-black">Zeichnung eingereicht</h2>
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
				<div class="mt-6 flex flex-row-reverse justify-start gap-4">
					<button
						class="rounded-lg bg-black px-4 py-2 text-white transition hover:bg-grey-800"
						onclick={() => {
							toClipboard(signatureUrl);
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
	.active {
		@apply bg-black;
		@apply text-white;
		@apply hover:bg-grey-800;
	}

	.inactive {
		@apply bg-white-600;
	}
</style>
