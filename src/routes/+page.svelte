<script lang="ts">
	import type { PageData } from "./$types";
	import SignaturePad, { type PointGroup } from "signature_pad";
	import type { Signature } from "$lib/types";
	import { onMount } from "svelte";

	let { data }: { data: PageData } = $props();

	let signatureIndex = $state(0);
	let current = $state(data.first);

	let canvas: HTMLCanvasElement;
	let box: HTMLDivElement | undefined = $state();
	let signaturePad: SignaturePad;

	let cache = $state([data.first]);

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

			signaturePad.clear();

			signaturePad.fromData(uncenterSignature(JSON.parse(current.signature)));
		}
	}

	async function loadPrev() {
		if (signatureIndex > 0) {
			signatureIndex--;

			const new_ = cache.at(signatureIndex);

			if (new_) current = new_;

			await cachePrev();
			current = new_;
		}
	}

	async function loadNext() {
		if (signatureIndex < data.signatures.length - 1) {
			signatureIndex++;
			if (cache.length <= signatureIndex) {
				const new_ = cache.at(signatureIndex);
				if (new_) current = new_;
				await cacheNext();
			} else {
				await cacheNext();
				const new_ = cache.at(signatureIndex);
				current = new_;
			}
		}
	}

	async function cacheNext() {
		if (cache.length < data.signatures.length && !cache.at(signatureIndex + 1)) {
			console.log("load next cache");
			cache.push(
				(await fetch(`/api/signatures/${data.signatures[signatureIndex + 1].id}`).then((res) =>
					res.json()
				)) as Signature
			);
		}
	}

	async function cachePrev() {
		if (signatureIndex - 1 >= 0 && !cache.at(signatureIndex - 1)) {
			console.log("load pre cache");
			cache.push(
				(await fetch(`/api/signatures/${data.signatures[signatureIndex - 1].id}`).then((res) =>
					res.json()
				)) as Signature
			);
		}
	}

	onMount(async () => {
		if (canvas) {
			signaturePad = new SignaturePad(canvas);
			signaturePad.off();
			signaturePad.fromData(uncenterSignature(JSON.parse(current.signature)));
			resizeCanvas();
		}

		$effect(() => {
			console.log("effect");
			signaturePad.fromData(uncenterSignature(JSON.parse(current.signature)));
		});

		for (let signature of data.signatures.slice(1)) {
			cache.push((await fetch(`/api/signatures/${signature.id}`).then((res) => res.json())) as Signature);
		}
	});
</script>

<svelte:window onresize={resizeCanvas} />

{signatureIndex + 1}<br />
{cache.length}

<section class="container mx-auto">
	<div class="relative mt-8 h-[650px] min-w-full overflow-hidden rounded-xl bg-white-600">
		<div bind:this={box} class="absolute h-full w-full">
			<canvas bind:this={canvas} class="absolute z-40 m-0 p-0" width="1536" height="650"></canvas>
			<div class="overlay h-full w-full">
				<div class="relative">
					<h1 class="absolute left-10 top-8 text-4xl text-black">vr</h1>
					<img
						src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/c6f4adce-577c-406a-f795-6b0892730a00/h=610"
						alt="Valentin Rogg"
						class="mx-auto translate-x-4 translate-y-4 pt-10 brightness-105"
					/>
					<!-- {#if signatureIndex > 0} -->
					<button
						aria-label="Previous"
						class="absolute left-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition"
						onclick={() => loadPrev()}
						class:opacity-0={signatureIndex <= 0}
						class:pointer-events-none={signatureIndex <= 0}
					>
						<i class="fa-solid fa-regular fa-arrow-left"></i>
					</button>
					<!-- {/if} -->
					{#if signatureIndex < data.signatures.length - 1}
						<button
							aria-label="Next"
							class="absolute right-8 top-[50%] z-50 h-12 w-12 translate-y-[-50%] rounded-full bg-white-500 transition"
							onclick={() => loadNext()}
							class:opacity-0={signatureIndex >= data.signatures.length - 1}
							class:pointer-events-none={signatureIndex >= data.signatures.length - 1}
						>
							<i class="fa-solid fa-regular fa-arrow-right"></i>
						</button>
					{/if}
					<div class="absolute bottom-8 left-8 z-40">
						<div>
							{current.name}
						</div>
						<div class="text-xs">
							{new Date(current.ts_created).toLocaleDateString("de")}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
	<div class="relative top-4 z-30 flex w-full justify-center gap-8 px-40">
		{#each data.signatures as signature, i}
			<div
				id={signature.id}
				class="h-2 w-2 rounded-lg bg-white-700 transition"
				class:bg-skin={signatureIndex === i}
			></div>
		{/each}
	</div>
</section>

<style lang="postcss">
	.bg-skin {
		@apply bg-skin-500;
	}
</style>
