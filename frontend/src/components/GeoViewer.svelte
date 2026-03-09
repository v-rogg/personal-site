<script lang="ts">
import { onMount } from "svelte";
import type { PipelineStep } from "../lib/geo/types";
import type { GeocodingResult } from "../lib/geo/nominatim";
import { getSessionId } from "../lib/session";
import AddressInput from "./GeoViewer/AddressInput.svelte";
import StepProgress from "./GeoViewer/StepProgress.svelte";

let step = $state<PipelineStep>("idle");
let visualStep = $state<PipelineStep>("idle");
let errorMessage = $state("");
let statusText = $state("");
let iframe: HTMLIFrameElement;
let activeEventSource: EventSource | null = null;
let pendingIframeSrc = "";

// Load iframe only when visual progress finishes
$effect(() => {
	if (visualStep === "ready" && pendingIframeSrc) {
		iframe.src = pendingIframeSrc;
		pendingIframeSrc = "";
	}
});

onMount(() => {
	// Auto-load default address
	runPipeline({ lat: 48.3696, lng: 10.8979, displayName: "Rathausplatz 2, 86150 Augsburg" });

	return () => {
		activeEventSource?.close();
	};
});

function runPipeline(result: GeocodingResult) {
	// Cancel previous SSE stream
	activeEventSource?.close();
	activeEventSource = null;
	errorMessage = "";

	// Reset iframe
	if (iframe) {
		iframe.src = "about:blank";
	}

	visualStep = "geocoding";
	step = "geocoding";
	statusText = result.displayName;

	const sessionId = getSessionId();
	const address = encodeURIComponent(result.displayName);
	const url = `/api/geo/buildings/stream?lat=${result.lat}&lng=${result.lng}&session_id=${sessionId}&address=${address}`;
	const es = new EventSource(url);
	activeEventSource = es;

	es.addEventListener("progress", (e) => {
		const data = JSON.parse(e.data);
		console.log("[sse] progress:", data);

		switch (data.step) {
			case "fetching":
				step = "fetching";
				statusText = `Lade Kachel ${data.current}/${data.total} (${data.size_mb ? data.size_mb + " MB" : data.tile}${data.cache_hit ? ", cached" : ""})...`;
				break;
			case "parsing":
				step = "parsing";
				statusText = `Verarbeite Kachel ${data.current}/${data.total} (${data.size_mb} MB)...`;
				break;
			case "building":
				step = "building";
				statusText = `Erstelle GeoParquet (${data.surfaces} Flächen)...`;
				break;
		}
	});

	es.addEventListener("complete", (e) => {
		const data = JSON.parse(e.data);
		console.log("[sse] complete:", data);
		es.close();
		activeEventSource = null;
		step = "ready";
		statusText = "";
		pendingIframeSrc = `/api/geo/buildings/${data.key}?t=${Date.now()}`;
	});

	es.addEventListener("error", (e) => {
		console.log("[sse] error:", e);
		const messageEvent = e as MessageEvent;
		if (messageEvent.data) {
			try {
				const data = JSON.parse(messageEvent.data);
				errorMessage = data.message || "Fehler beim Laden";
			} catch {
				errorMessage = "Verbindung zum Server fehlgeschlagen";
			}
		} else {
			errorMessage = "Verbindung zum Server fehlgeschlagen";
		}
		es.close();
		activeEventSource = null;
		step = "error";
		statusText = "";
	});
}
</script>

<section class="mt-20">
	<div class="mb-4 flex flex-col items-end gap-12 justify-end lg:flex-row">
		<p class="text-black">
			3D-Gebäudemodelle aus Bayerns Open-Data-Portal &mdash; on-demand mit
			<a href="/blog/qq" class="underline">qq</a>-Geoviewer.
		</p>
		<h2
			class="-mb-[0.5em] block w-max text-[4rem] font-bold leading-none tracking-tight text-white-600 max-lg:-mt-10 max-md:pr-10 sm:text-[6rem] md:-mb-[0.14em] md:text-[10rem] lg:-ml-[8px] xl:-ml-[7px] 2xl:-ml-[12px] 2xl:text-[16rem]"
		>
			Geodaten
		</h2>
	</div>

	<div class="relative">
		<!-- Annotation with arrow, outside overflow:hidden so it can overlap left edge -->
		<div class="pointer-events-none absolute z-30 max-sm:hidden top-18.5 left-3 xl:-left-8">
			<svg
				width="20"
				viewBox="0 0 67 32"
				xmlns="http://www.w3.org/2000/svg"
				style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
				class="inline-block -rotate-80 absolute left-20 -top-4"
				fill="currentColor"
				><path
					d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
				></path></svg
			>
			<span
				class="rotate-6 inline-block rounded-lg border-4 border-white bg-white-700 px-2 py-1 text-center font-[450] leading-tight"
				style="margin-left: 4px;"
			>
				<span class="text-black">Such dein <br/> Gebäude</span>
			</span>
		</div>

		<div class="sm:rounded-xl bg-white-600 overflow-hidden">
			<div class="relative" style="height: 560px;">
				<!-- Address input overlaying iframe -->
				<div class="absolute z-40" style="top: 12px; left: 12px; width: 350px;">
					<AddressInput onSelect={runPipeline} initialValue="Rathausplatz 2, 86150 Augsburg" />
				</div>

			{#if visualStep === "idle"}
				<div class="flex items-center justify-center h-full text-black/30 text-sm">
					Gib eine bayerische Adresse ein, um 3D-Gebäude zu laden
				</div>
			{:else if visualStep !== "ready"}
				<div class="absolute inset-0 flex flex-col items-center justify-center z-10">
					<StepProgress bind:step {errorMessage} {statusText} onVisualStep={(s) => (visualStep = s)} />
				</div>
			{/if}
			<iframe
				bind:this={iframe}
				title="3D GeoViewer"
				class="w-full h-full border-0 {visualStep === 'ready' ? '' : 'hidden'}"
				sandbox="allow-scripts allow-same-origin"
			></iframe>
			</div>
		</div>
	</div>
</section>
