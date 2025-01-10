<script lang="ts">
	import type { EmblaCarouselType } from "embla-carousel";
	import Autoplay from "embla-carousel-autoplay";
	import emblaCarouselSvelte from "embla-carousel-svelte";
	import { blur } from "svelte/transition";
	import { expoInOut } from "svelte/easing";
	import SignaturePad, { type PointGroup } from "signature_pad";
	import { onMount } from "svelte";
	import type { Signature, SignatureMeta } from "$lib/types";

	let { signatures = $bindable(), openEditMode, autoplay } = $props();

	interface ImageCache extends Signature {
		image: string;
	}

	let emblaApi: EmblaCarouselType;
	let emblaPlugins = [Autoplay({ playOnInit: autoplay, delay: 6000 })];
	let autoplayState: boolean | undefined = $state(undefined);
	let signatureIndex = $state(0);
	let buildCanvas: HTMLCanvasElement;
	let signatureBuildPad: SignaturePad;
	let signatureImageCache: ImageCache[] = $state([]);
	signatureImageCache = Array(signatures.length);

	function onInit(event: CustomEvent<EmblaCarouselType>) {
		emblaApi = event.detail;
		emblaApi.on("select", () => {
			signatureIndex = emblaApi.selectedScrollSnap();
		});
		emblaApi.on("autoplay:play", () => {
			autoplayState = true;
		});
		emblaApi.on("autoplay:stop", () => {
			autoplayState = false;
		});
	}

	function uncenterSignature(data: PointGroup[]): PointGroup[] {
		const middleH = 1536 / 2;
		const bottomV = 650;

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

	function loadPrev() {
		emblaApi.scrollPrev();
		emblaApi.plugins().autoplay.stop();
	}

	function loadNext() {
		emblaApi.scrollNext();
		emblaApi.plugins().autoplay.stop();
	}

	onMount(async () => {
		if (buildCanvas) {
			signatureBuildPad = new SignaturePad(buildCanvas);
			signatureBuildPad.off();
			buildCanvas.width = 1536;
			buildCanvas.height = 650;

			const cache = localStorage.getItem("vr-www.signature." + signatures[0].id);
			let res: Signature;
			if (cache === null) {
				res = (await fetch(`/api/signatures/${signatures[0].id}`).then((res) => res.json())) as Signature;

				localStorage.setItem("vr-www.signature." + signatures[0].id, JSON.stringify(res));
			} else {
				res = JSON.parse(cache);
			}
			signatureBuildPad.fromData(uncenterSignature(JSON.parse(res.signature)));
			signatureImageCache[0] = { ...res, image: signatureBuildPad.toDataURL() };

			await Promise.all(
				signatures.slice(1).map(async (signature: SignatureMeta, i: number) => {
					const cache = localStorage.getItem("vr-www.signature." + signature.id);
					let res: Signature;
					if (cache === null) {
						res = (await fetch(`/api/signatures/${signature.id}`).then((res) => res.json())) as Signature;

						localStorage.setItem("vr-www.signature." + signature.id, JSON.stringify(res));
					} else {
						res = JSON.parse(cache);
					}

					setTimeout(() => {
						signatureBuildPad.fromData(uncenterSignature(JSON.parse(res.signature)));
						signatureImageCache[i + 1] = { ...res, image: signatureBuildPad.toDataURL() };
					}, 0);
				})
			);
		}
	});
</script>

<div
	class="embla absolute top-0 z-40 h-full w-full sm:overflow-hidden"
	use:emblaCarouselSvelte={{ options: { loop: true }, plugins: emblaPlugins }}
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
	onemblaInit={onInit}
>
	<div class="embla__container flex h-full w-full cursor-grab active:cursor-grabbing">
		{#each signatureImageCache.filter((e) => e != undefined) as slide}
			{@const date_created = new Date(slide.ts_created)}
			<div id={slide.id} class="embla__slide relative z-30 min-w-0 font-sans" style="flex: 0 0 100%">
				<div class="relative h-full w-full overflow-hidden">
					<div class="h-full w-full" class:old_signature_offset={date_created.getTime() <= 1733719053000}>
						<img
							src={slide.image}
							alt={slide.name}
							class="absolute bottom-0 left-[50%] mx-auto block h-auto max-w-[unset] translate-x-[-50%] overflow-hidden"
						/>
					</div>
				</div>
				<div class="text-outline absolute -bottom-16 left-10 z-40 h-12 sm:bottom-8">
					<div class="translate-y-3 font-hand font-[450]">
						<i class="fa-regular fa-tag"></i>
						{slide.name}
					</div>
					<div class="translate-y-1 text-xs">
						{date_created.toLocaleDateString("de")}
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
<div
	class="absolute bottom-0 right-10 z-50 flex h-12 items-center gap-3 sm:bottom-8"
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
>
	{#if autoplayState}
		<button
			aria-label="Autoplay Stoppen"
			title="Autoplay Stoppen"
			onclick={() => {
				emblaApi.plugins().autoplay.stop();
			}}
			in:blur={{ amount: 1 }}
			class="relative mr-2 size-2"
		>
			<i class="fa-solid fa-pause absolute top-1/2 -translate-y-1/2"></i>
		</button>
	{:else}
		<button
			aria-label="Autoplay Starten"
			title="Autoplay Starten"
			onclick={() => {
				emblaApi.plugins().autoplay.play();
			}}
			in:blur={{ amount: 1 }}
			class="relative mr-2 size-2"
		>
			<i class="fa-solid fa-play absolute top-1/2 -translate-y-1/2"></i>
		</button>
	{/if}
	{#each signatures as signature, i}
		<button
			id={signature.id}
			class="size-2 rounded-full bg-white-700 shadow-lg transition hover:scale-125 hover:bg-grey-800 max-2xl:hidden"
			class:active={signatureIndex === i}
			class:opacity-25={signatureImageCache[i] == undefined}
			aria-label={`Zeichung Nr. ${i + 1}`}
			onclick={() => {
				emblaApi.scrollTo(i);
				emblaApi.plugins().autoplay.stop();
			}}
			disabled={signatureImageCache[i] == undefined}
		>
		</button>
	{/each}
	<button
		aria-label="Neue Zeichnung erstellen"
		class="relative z-40 block size-12 rounded-full bg-white-700 transition hover:bg-white-700 active:bg-white-600 max-sm:-bottom-16 sm:bg-white-500"
		onclick={() => {
			openEditMode();
		}}
		title="Neue Zeichnung erstellen"
	>
		<span
			class="text-outline absolute right-full w-max text-center font-hand max-sm:hidden sm:-left-16 sm:-top-16 sm:rotate-6"
		>
			<span class="relative z-10" style="">
				Erstell' auch<br /> eine Zeichnung
			</span>
		</span>
		<span class="text-outline pointer-events-none absolute bottom-full right-10 -rotate-6 -scale-x-100 max-sm:hidden">
			<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
					fill="black"
				></path>
			</svg>
		</span>
		<i class="fa-solid fa-plus"> </i>
	</button>
</div>
<button
	aria-label="Vorherige Zeichnung"
	class="absolute left-10 top-[50%] z-50 size-12 translate-y-[-50%] rounded-full bg-white-500 transition hover:bg-white-700 active:bg-white-600 max-lg:hidden"
	onclick={() => loadPrev()}
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
>
	<i class="fa-solid fa-regular fa-arrow-left"></i>
</button>
<button
	aria-label="Nächste Zeichnung"
	class="absolute right-10 top-[50%] z-50 size-12 translate-y-[-50%] rounded-full bg-white-500 transition hover:bg-white-700 active:bg-white-600 max-lg:hidden"
	onclick={() => loadNext()}
	transition:blur={{ amount: 10, duration: 600, easing: expoInOut }}
>
	<i class="fa-solid fa-regular fa-arrow-right"></i>
</button>
<canvas bind:this={buildCanvas} class="absolute -z-50 m-0 hidden p-0" width="1536" height="650"></canvas>

<style lang="postcss">
	.active {
		@apply bg-grey-800;
		@apply scale-125;
	}

	.text-outline {
		@media (min-width: 640px) {
			text-shadow:
				1px 1px 0 theme("colors.white.700"),
				-1px -1px 0 theme("colors.white.700"),
				1px -1px 0 theme("colors.white.700"),
				-1px 1px 0 theme("colors.white.700");
		}
	}

	.old_signature_offset {
		@apply -translate-x-3 -translate-y-4;
	}
</style>
