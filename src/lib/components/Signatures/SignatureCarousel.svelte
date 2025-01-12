<script lang="ts">
	import type { EmblaCarouselType } from "embla-carousel";
	import Autoplay from "embla-carousel-autoplay";
	import emblaCarouselSvelte from "embla-carousel-svelte";
	import { blur } from "svelte/transition";
	import { expoInOut } from "svelte/easing";
	import SignaturePad, { type PointGroup } from "signature_pad";
	import { onMount } from "svelte";
	import type { Signature, SignatureMeta } from "$lib/types";
	import posthog from "posthog-js";

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
		posthog.capture("click.signatures.previous");
	}

	function loadNext() {
		emblaApi.scrollNext();
		emblaApi.plugins().autoplay.stop();
		posthog.capture("click.signatures.next");
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
				<div class="absolute -bottom-16 left-10 z-40 h-12 text-[450] sm:bottom-6">
					<div class="translate-y-1 font-[450]">
						<i class="fa-regular fa-tag"></i>
						{slide.name}
					</div>
					<div class="translate-y-[2px] text-xs">
						{date_created.toLocaleDateString("de-DE", { day: "2-digit", month: "2-digit", year: "2-digit" })}
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
				posthog.capture("click.signatures.autoplay.stop");
			}}
			in:blur={{ amount: 1 }}
			class="relative mr-2 size-2 max-sm:hidden"
		>
			<i class="fa-solid fa-pause absolute top-1/2 -translate-y-1/2"></i>
		</button>
	{:else}
		<button
			aria-label="Autoplay Starten"
			title="Autoplay Starten"
			onclick={() => {
				emblaApi.plugins().autoplay.play();
				posthog.capture("click.signatures.autoplay.start");
			}}
			in:blur={{ amount: 1 }}
			class="relative mr-2 size-2 max-sm:hidden"
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
				posthog.capture("click.signatures.carousel.skip");
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
			class="absolute right-full w-max rounded-lg border-4 border-white bg-white-700 px-2 py-1 text-center font-[450] leading-tight max-sm:hidden sm:-left-16 sm:-top-[76px] sm:-rotate-6 xl:-left-5 xl:-top-[76px] xl:rotate-6"
		>
			<span class="relative z-10" style="">
				Erstelle auch<br /> eine Zeichnung
			</span>
		</span>
		<span
			class="pointer-events-none absolute bottom-full max-sm:hidden sm:-right-1 sm:-top-[12px] sm:rotate-[6deg] xl:-right-2 xl:-top-[9px] xl:rotate-[24deg]"
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

	.old_signature_offset {
		@apply -translate-x-3 -translate-y-4;
	}
</style>
