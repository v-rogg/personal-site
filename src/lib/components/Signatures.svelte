<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import emblaCarouselSvelte from "embla-carousel-svelte";
	import Autoplay from "embla-carousel-autoplay";
	import type { Signature } from "$lib/types";
	import { onMount } from "svelte";

	let { signatures } = $props();

	let signatureIndex = $state(0);

	let canvas: HTMLCanvasElement;
	let buildCanvas: HTMLCanvasElement;
	let box: HTMLDivElement | undefined = $state();
	let signaturePad: SignaturePad;
	let signatureBuildPad: SignaturePad;

	let emblaApi: unknown;
	let emblaPlugins = [Autoplay({ playOnInit: true, delay: 4000 })];

	interface ImageCache extends Signature {
		image: string;
	}

	let cache: Map<string, Signature> = $state(new Map());
	let signatureImageCache: ImageCache[] = $state([]);

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
		emblaApi.plugins().autoplay.reset();
	}

	function loadNext() {
		// @ts-expect-error type unknown
		emblaApi.scrollNext();
		// @ts-expect-error type unknown
		emblaApi.plugins().autoplay.reset();
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

		for (let signature of signatures) {
			const res = (await fetch(`/api/signatures/${signature.id}`).then((res) => res.json())) as Signature;
			cache.set(res.id, res);

			signatureBuildPad.fromData(uncenterSignature(JSON.parse(res.signature)));
			signatureImageCache.push({ ...res, image: signatureBuildPad.toDataURL() });
		}
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
				{#each signatureImageCache as slide}
					{@const date_created = new Date(slide.ts_created)}
					<div class="embla__slide relative min-w-0 overflow-hidden font-sans" style="flex: 0 0 100%">
						<div class="relative h-full w-full" class:old_signature_offset={date_created.getTime() <= 1733719053000}>
							<img
								src={slide.image}
								alt={slide.name}
								class="absolute bottom-0 left-[50%] mx-auto block h-auto max-w-[unset] translate-x-[-50%]"
							/>
						</div>
						<div class="absolute bottom-8 left-10 z-40">
							<div class="font-[450]">
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
					aria-label="Previous"
					class="absolute left-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition max-lg:hidden"
					onclick={() => loadPrev()}
				>
					<i class="fa-solid fa-regular fa-arrow-left"></i>
				</button>
				<button
					aria-label="Next"
					class="absolute right-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition max-lg:hidden"
					onclick={() => loadNext()}
				>
					<i class="fa-solid fa-regular fa-arrow-right"></i>
				</button>
			</div>
			<!-- </div> -->
		</div>
		<div class="absolute bottom-0 right-10 z-30 mb-8 flex gap-3">
			{#each signatures as signature, i}
				<div
					id={signature.id}
					class="size-2 rounded-full bg-white-700 transition"
					class:active={signatureIndex === i}
					class:opacity-25={signatureImageCache.length < i}
				></div>
			{/each}
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
