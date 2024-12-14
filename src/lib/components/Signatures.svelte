<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import emblaCarouselSvelte from "embla-carousel-svelte";
	import Autoplay from "embla-carousel-autoplay";
	import type { Signature, SignatureMeta } from "$lib/types";
	import { onMount } from "svelte";
	import { blur } from "svelte/transition";
	import { expoOut } from "svelte/easing";

	let { signatures } = $props();

	let signatureIndex = $state(0);

	let canvas: HTMLCanvasElement;
	let buildCanvas: HTMLCanvasElement;
	let box: HTMLDivElement | undefined = $state();
	let signaturePad: SignaturePad;
	let signatureBuildPad: SignaturePad;

	let emblaApi: unknown;
	let emblaPlugins = [Autoplay({ playOnInit: true, delay: 6000 })];

	interface ImageCache extends Signature {
		image: string;
	}

	// let cache: Map<string, Signature> = $state(new Map());
	let signatureImageCache: ImageCache[] = $state([]);
	signatureImageCache = Array(signatures.length);

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
		return [];
	}

	function resizeCanvas() {
		if (box) {
			console.log("resize");
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;

			// signaturePad.clear();

			// signatureImageCache = [];
			// for (let [, signature] of cache) {
			// 	signatureBuildPad.fromData(uncenterSignature(JSON.parse(signature.signature)));
			// 	signatureImageCache.push({ ...signature, image: signatureBuildPad.toDataURL() });
			// }

			// signaturePad.fromData(uncenterSignature(JSON.parse(current.signature)));
		}
	}

	function loadPrev() {
		// @ts-expect-error type unknown
		emblaApi.scrollPrev();
		// @ts-expect-error type unknown
		emblaApi.plugins().autoplay.stop();
	}

	function loadNext() {
		// @ts-expect-error type unknown
		emblaApi.scrollNext();
		// @ts-expect-error type unknown
		emblaApi.plugins().autoplay.stop();
	}

	onMount(async () => {
		if (canvas && box) {
			signaturePad = new SignaturePad(canvas);
			signaturePad.off();
			signatureBuildPad = new SignaturePad(buildCanvas);
			signatureBuildPad.off();
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;
			buildCanvas.width = 1536;
			buildCanvas.height = 650;
		}

		const res = (await fetch(`/api/signatures/${signatures[0].id}`).then((res) => res.json())) as Signature;
		// cache.set(res.id, res);
		signatureBuildPad.fromData(uncenterSignature(JSON.parse(res.signature)));
		signatureImageCache[0] = { ...res, image: signatureBuildPad.toDataURL() };

		await Promise.all(
			signatures.slice(1).map(async (signature: SignatureMeta, i: number) => {
				const res = (await fetch(`/api/signatures/${signature.id}`).then((res) => res.json())) as Signature;
				// cache.set(res.id, res);

				signatureBuildPad.fromData(uncenterSignature(JSON.parse(res.signature)));
				signatureImageCache[i + 1] = { ...res, image: signatureBuildPad.toDataURL() };
			})
		);
	});

	// @ts-expect-error type unknown
	function onInit(event) {
		emblaApi = event.detail;
		// @ts-expect-error type unknown
		emblaApi.on("select", () => {
			// @ts-expect-error type unknown
			signatureIndex = emblaApi.selectedScrollSnap();
		});
	}
</script>

<svelte:window onresize={resizeCanvas} />

<section class="container mx-auto">
	<div class="relative mt-8 h-[650px] min-w-full overflow-hidden bg-white-600 sm:rounded-xl">
		<div
			class="embla absolute z-40 h-full w-full overflow-hidden"
			use:emblaCarouselSvelte={{ options: { loop: true }, plugins: emblaPlugins }}
			onemblaInit={onInit}
		>
			<div class="embla__container flex h-full w-full">
				{#each signatureImageCache.filter((e) => e != undefined) as slide, i}
					{@const date_created = new Date(slide.ts_created)}
					<div
						class="embla__slide relative min-w-0 overflow-hidden font-sans"
						style="flex: 0 0 100%"
						in:blur={{ duration: 500, amount: i === 0 ? 10 : 0, easing: expoOut }}
					>
						<div class="relative h-full w-full" class:old_signature_offset={date_created.getTime() <= 1733719053000}>
							<img
								src={slide.image}
								alt={slide.name}
								class="absolute bottom-0 left-[50%] mx-auto block h-auto max-w-[unset] translate-x-[-50%]"
							/>
						</div>
						<div class="absolute bottom-8 left-10 z-40">
							<div class="font-hand font-[450]">
								<i class="fa-regular fa-tag"></i>
								{slide.name}
							</div>
							<div class="text-xs">
								{date_created.toLocaleDateString("de")}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
		<div bind:this={box} class="h-full w-full">
			<canvas bind:this={canvas} class="absolute z-40 m-0 hidden p-0" width="1536" height="650"></canvas>
			<canvas bind:this={buildCanvas} class="absolute -z-50 m-0 hidden p-0" width="1536" height="650"></canvas>
			<!-- <div class="overlay h-full w-full"> -->
			<div class="relative h-full w-full">
				<img
					src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/c6f4adce-577c-406a-f795-6b0892730a00/h=610"
					alt="Valentin Rogg"
					class="absolute bottom-0 left-[50%] mx-auto block h-auto max-w-[unset] translate-x-[-50%] pt-10 brightness-105"
					height="650"
				/>
				<button
					aria-label="Vorherige Zeichnung"
					class="absolute left-10 top-[50%] z-50 size-12 translate-y-[-50%] rounded-full bg-white-500 transition hover:bg-white-700 active:bg-white-600 max-lg:hidden"
					onclick={() => loadPrev()}
				>
					<i class="fa-solid fa-regular fa-arrow-left"></i>
				</button>
				<button
					aria-label="Nächste Zeichnung"
					class="absolute right-10 top-[50%] z-50 size-12 translate-y-[-50%] rounded-full bg-white-500 transition hover:bg-white-700 active:bg-white-600 max-lg:hidden"
					onclick={() => loadNext()}
				>
					<i class="fa-solid fa-regular fa-arrow-right"></i>
				</button>
			</div>
			<!-- </div> -->
		</div>
		<div class="absolute bottom-0 right-10 z-50 mb-8 flex items-center gap-3">
			{#each signatures as signature, i}
				<button
					id={signature.id}
					class="size-2 rounded-full bg-white-700 transition hover:scale-125 hover:bg-grey-800"
					class:active={signatureIndex === i}
					class:opacity-25={signatureImageCache[i] == undefined}
					aria-label={`Zeichung Nr. ${i + 1}`}
					onclick={() => {
						// @ts-expect-error type unknown
						emblaApi.scrollTo(i);
						// @ts-expect-error type unknown
						emblaApi.plugins().autoplay.stop();
					}}
					disabled={signatureImageCache[i] == undefined}
				>
				</button>
			{/each}
			<button
				aria-label="Neue Zeichnung erstellen"
				class="relative size-12 rounded-full bg-white-500 transition hover:bg-white-700 active:bg-white-600"
			>
				<span class="absolute -left-16 -top-16 w-max rotate-6 text-center font-hand">
					Erstell' auch<br /> eine Zeichnung
				</span>
				<span class="absolute bottom-full right-10 -rotate-6 -scale-x-100">
					<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
						<path
							d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
							fill="black"
						></path>
					</svg>
				</span>
				<i class="fa-solid fa-plus"> </i></button
			>
		</div>
	</div>
</section>

<style lang="postcss">
	.active {
		@apply bg-grey-800;
		@apply scale-125;
	}

	.old_signature_offset {
		@apply -translate-x-3 -translate-y-4;
	}
</style>
