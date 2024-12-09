<script lang="ts">
	import SignaturePad, { type PointGroup } from "signature_pad";
	import emblaCarouselSvelte from "embla-carousel-svelte";
	import Autoplay from "embla-carousel-autoplay";
	import type { PageData } from "./$types";
	import type { Signature } from "$lib/types";
	import { onMount } from "svelte";

	let { data }: { data: PageData } = $props();

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

	let cache: Signature[] = $state([]);
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
			canvas.width = box.offsetWidth;
			canvas.height = box.offsetHeight;
			buildCanvas.width = box.offsetWidth;
			buildCanvas.height = box.offsetHeight;

			// signaturePad.clear();

			// signaturePad.fromData(uncenterSignature(JSON.parse(current.signature)));
		}
	}

	// async function loadPrev() {
	// 	if (signatureIndex > 0) {
	// 		signatureIndex--;

	// 		const new_ = cache.at(signatureIndex);

	// 		if (new_) current = new_;

	// 		await cachePrev();
	// 		current = new_;
	// 	}
	// }

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

	// async function loadNext() {
	// 	if (signatureIndex < data.signatures.length - 1) {
	// 		signatureIndex++;
	// 		if (cache.length <= signatureIndex) {
	// 			const new_ = cache.at(signatureIndex);
	// 			if (new_) current = new_;
	// 			await cacheNext();
	// 		} else {
	// 			await cacheNext();
	// 			const new_ = cache.at(signatureIndex);
	// 			current = new_;
	// 		}
	// 	}
	// }

	// async function cacheNext() {
	// 	if (cache.length < data.signatures.length && !cache.at(signatureIndex + 1)) {
	// 		console.log("load next cache");
	// 		cache.push(
	// 			(await fetch(`/api/signatures/${data.signatures[signatureIndex + 1].id}`).then((res) =>
	// 				res.json()
	// 			)) as Signature
	// 		);
	// 	}
	// }

	// async function cachePrev() {
	// 	if (signatureIndex - 1 >= 0 && !cache.at(signatureIndex - 1)) {
	// 		console.log("load pre cache");
	// 		cache.push(
	// 			(await fetch(`/api/signatures/${data.signatures[signatureIndex - 1].id}`).then((res) =>
	// 				res.json()
	// 			)) as Signature
	// 		);
	// 	}
	// }

	onMount(async () => {
		if (canvas) {
			signaturePad = new SignaturePad(canvas);
			signaturePad.off();
			signatureBuildPad = new SignaturePad(buildCanvas);
			signatureBuildPad.off();
			resizeCanvas();
		}

		for (let signature of data.signatures) {
			const res = (await fetch(`/api/signatures/${signature.id}`).then((res) => res.json())) as Signature;
			cache.push(res);

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

{signatureIndex + 1}<br />
{cache.length}

<!-- <div class="embla absolute z-40 overflow-hidden" use:emblaCarouselSvelte onemblaInit={onInit}>
	<div class="embla__container overflow-hidden"> -->
<!-- {#each signatureImageCache as image, i} -->
<!-- <div class="embla__slide">
							{i}
							<img src={image} alt={current.id} />
						</div> -->
<!-- {/each} -->
<!-- <div class="embla__slide">1 <img src={signatureImageCache[0]} alt={current.id} /></div>
	</div>
	<div class="embla__slide">2</div>
</div> -->

<section class="container mx-auto">
	<div class="relative mt-8 h-[650px] min-w-full overflow-hidden rounded-xl bg-white-600">
		<div
			class="embla absolute z-40"
			use:emblaCarouselSvelte={{ options: { loop: true }, plugins: emblaPlugins }}
			onemblaInit={onInit}
		>
			<div class="embla__container">
				{#each signatureImageCache as slide}
					{@const date_created = new Date(slide.ts_created)}
					<div class="embla__slide relative font-sans">
						<img
							src={slide.image}
							alt={slide.name}
							class:old_signature_offset={date_created.getTime() <= 1733719053000}
						/>
						<div class="absolute bottom-8 left-8 z-40">
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
		<div bind:this={box} class="absolute h-full w-full">
			<canvas bind:this={canvas} class="absolute z-40 m-0 hidden p-0" width="1536" height="650"></canvas>
			<canvas bind:this={buildCanvas} class="absolute -z-50 m-0 hidden p-0" width="1536" height="650"></canvas>
			<div class="overlay h-full w-full">
				<div class="relative">
					<h1 class="absolute left-10 top-8 text-4xl text-black">vr</h1>
					<img
						src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/c6f4adce-577c-406a-f795-6b0892730a00/h=610"
						alt="Valentin Rogg"
						class="mx-auto pt-10 brightness-105"
					/>
					<!-- {#if signatureIndex > 0} -->
					<button
						aria-label="Previous"
						class="absolute left-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition"
						onclick={() => loadPrev()}
					>
						<i class="fa-solid fa-regular fa-arrow-left"></i>
					</button>
					<!-- {/if} -->
					<!-- {#if signatureIndex < data.signatures.length - 1} -->
					<button
						aria-label="Next"
						class="absolute right-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition"
						onclick={() => loadNext()}
					>
						<i class="fa-solid fa-regular fa-arrow-right"></i>
					</button>
					<!-- {/if} -->
					<!-- <div class="absolute bottom-8 left-8 z-40">
						<div>
							{current.name}
						</div>
						<div class="text-xs">
							{new Date(current.ts_created).toLocaleDateString("de")}
						</div>
					</div> -->
				</div>
			</div>
		</div>
	</div>
	<div class="relative top-4 z-30 mb-8 flex w-full justify-center gap-8 px-40">
		{#each data.signatures as signature, i}
			<div
				id={signature.id}
				class="size-2 rounded-full bg-white-700 transition"
				class:active={signatureIndex === i}
				class:opacity-25={signatureImageCache.length < i}
			></div>
		{/each}
	</div>
</section>

<section class="container mx-auto">
	<div class="flex min-h-[25rem] flex-col justify-center px-8">
		<h1 class="mb-4 font-sans text-4xl font-semibold text-black">Valentin Rogg</h1>
		<!-- prettier-ignore -->
		<p class="font-serif max-w-xl whitespace-pre text-md-lg text-black">
Hallo, ich bin Valentin. Ich bin Entwickler und Mediengestalter und spezialisiere mich
auf die Konzeption und Entwicklung von Micro-SaaS und MVPs. Mein Fokus liegt dabei
vor allem in der automatisierten Datenanalyse und interaktiven Visualisierung.
Ich benutzte gerne Svelte <img src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/5d02d81b-f14f-4945-c3f7-07659f629c00/h=18" alt="Svelte" class="inline -translate-y-0.5 -mx-0.5"/>, Rust 🦀 und WebGL <i class="fa-light fa-cubes text-[#980000]"></i>.
Privat bin ich leidenschaftlicher Musiker, Sportler und Globetrotter.
		</p>
		<div class="mb-6 mt-10 w-max text-2xl text-black">
			<div class="flex gap-4">
				<a
					href="https://www.komoot.de/user/2406562570904"
					rel="noreferrer nofollow"
					target="_blank"
					aria-label="Komoot"
				>
					<i class="fa-kit fa-komoot"></i>
				</a>
				<a
					href="https://www.instagram.com/valentin_rogg/"
					rel="noreferrer nofollow"
					target="_blank"
					aria-label="Instagram"><i class="fa-brands fa-instagram"></i></a
				>
				<a href="https://www.youtube.com/@roggnroll" rel="noreferrer nofollow" target="_blank" aria-label="YouTube"
					><i class="fa-brands fa-youtube"></i></a
				>
				<span class="overflow_hand cv">
					<!-- <span class="looking_for_cv font-handwriting">Verbinde dich mit mir</span>
					<span class="hand_arrow">
						<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
							<path
								d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
								fill="black"
							></path>
						</svg>
					</span> -->
					<a href="https://www.linkedin.com/in/vrogg/" rel="noreferrer nofollow" target="_blank" aria-label="LinkedIn"
						><i class="fa-brands fa-linkedin"></i></a
					>
				</span>
				<a href="mailto:mail@valentinrogg.de" rel="noreferrer nofollow" target="_blank" class="mail" aria-label="Mail"
					><i class="fa-solid fa-envelope"></i></a
				>
			</div>
		</div>
	</div>
</section>

<style lang="postcss">
	.embla {
		overflow: hidden;
	}
	.embla__container {
		display: flex;
	}
	.embla__slide {
		flex: 0 0 100%;
		min-width: 0;
	}

	.active {
		@apply bg-grey-800;
		@apply scale-125;
	}

	.old_signature_offset {
		@apply -translate-x-3 -translate-y-4;
	}
</style>
