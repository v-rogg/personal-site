<script lang="ts">
import { untrack } from "svelte";
import { STEP_LABELS, type PipelineStep } from "../../lib/geo/types";

let {
	step = $bindable(),
	errorMessage = "",
	statusText = "",
	onVisualStep
}: {
	step: PipelineStep;
	errorMessage?: string;
	statusText?: string;
	onVisualStep?: (step: PipelineStep) => void;
} = $props();

const steps: PipelineStep[] = ["geocoding", "fetching", "parsing", "building"];

const stepToIndex: Record<PipelineStep, number> = {
	idle: -1,
	geocoding: 0,
	fetching: 1,
	parsing: 2,
	building: 3,
	ready: 4,
	error: -2
};

let visualStep = $state<PipelineStep>("idle");
let visualDescription = $state("");
let smoothProgress = $state(0);

type QueueEntry =
	| { type: "step"; step: PipelineStep }
	| { type: "msg"; text: string; step: PipelineStep };
let queue: QueueEntry[] = [];
let processing = false;
let stepShownAt = 0;
let rafId = 0;

const BASE_STEP_MS: Partial<Record<PipelineStep, number>> = {
	geocoding: 200,
	fetching: 600,
	parsing: 400,
	building: 200,
	ready: 200
};

const JITTER = 0.25;

function jitteredMs(step: PipelineStep): number {
	const base = BASE_STEP_MS[step] ?? 0;
	return base * (1 + (Math.random() * 2 - 1) * JITTER);
}

// Ease-out cubic: fast start, slows toward end
function easeOut(t: number): number {
	return 1 - (1 - t) * (1 - t) * (1 - t);
}

function startSubstepAnimation(stepIdx: number, durationMs: number) {
	cancelAnimationFrame(rafId);
	const segmentStart = (stepIdx / steps.length) * 100;
	const segmentSize = (1 / steps.length) * 100;
	const targetFill = 0.9; // fill up to 90% of the segment
	const startTime = performance.now();

	function tick(now: number) {
		const elapsed = now - startTime;
		const t = Math.min(elapsed / durationMs, 1);
		smoothProgress = segmentStart + segmentSize * targetFill * easeOut(t);
		if (t < 1) {
			rafId = requestAnimationFrame(tick);
		}
	}
	rafId = requestAnimationFrame(tick);
}

function snapProgress(stepIdx: number) {
	cancelAnimationFrame(rafId);
	if (stepIdx >= steps.length) {
		smoothProgress = 100;
	} else {
		smoothProgress = (stepIdx / steps.length) * 100;
	}
}

async function processQueue() {
	if (processing) return;
	processing = true;
	while (queue.length > 0) {
		const entry = queue.shift()!;

		if (entry.type === "msg") {
			const vIdx = untrack(() => stepToIndex[visualStep]);
			if (stepToIndex[entry.step] === vIdx) {
				visualDescription = entry.text;
			}
			continue;
		}

		// Step transition
		const next = entry.step;
		const currentIdx = untrack(() => stepToIndex[visualStep]);
		if (stepToIndex[next] < currentIdx) continue;
		if (next === untrack(() => visualStep)) continue;

		// Enforce minimum display time for the previous step
		const prevStep = untrack(() => visualStep);
		const prevMinMs = prevStep !== "idle" ? jitteredMs(prevStep) : 0;
		const elapsed = Date.now() - stepShownAt;
		const remaining = prevMinMs - elapsed;
		if (remaining > 0) {
			await new Promise((r) => setTimeout(r, remaining));
		}

		visualStep = next;
		stepShownAt = Date.now();
		onVisualStep?.(next);

		// Start substep animation for this step
		const nextIdx = stepToIndex[next];
		if (nextIdx >= 0 && nextIdx < steps.length) {
			const animDuration = jitteredMs(next);
			startSubstepAnimation(nextIdx, animDuration);
		} else if (next === "ready") {
			snapProgress(steps.length);
		}

		// Show first queued message for this step immediately, or clear
		if (queue.length > 0 && queue[0].type === "msg" && queue[0].step === next) {
			visualDescription = (queue.shift() as Extract<QueueEntry, { type: "msg" }>).text;
		} else {
			visualDescription = "";
		}
	}
	processing = false;
}

// Track step changes
$effect(() => {
	const s = step;
	untrack(() => {
		if (s === "geocoding") {
			queue = [];
			processing = false;
			cancelAnimationFrame(rafId);
			visualStep = "geocoding";
			visualDescription = "";
			stepShownAt = Date.now();
			smoothProgress = 0;
			onVisualStep?.("geocoding");
			startSubstepAnimation(0, jitteredMs("geocoding"));
		}
		queue.push({ type: "step", step: s });
		processQueue();
	});
});

// Track statusText changes — queue as messages
$effect(() => {
	const text = statusText;
	untrack(() => {
		if (text) {
			queue.push({ type: "msg", text, step });
			processQueue();
		}
	});
});

function getStepState(i: number): "completed" | "active" | "pending" | "error" {
	const current = stepToIndex[visualStep];
	if (visualStep === "error") {
		if (i === 0) return "completed";
		return i <= 1 ? "error" : "pending";
	}
	if (current > i) return "completed";
	if (current === i) return "active";
	return "pending";
}
</script>

<div style="display: flex; flex-direction: column; align-items: center; gap: 16px; width: 460px; max-width: 100%;">
	<!-- Step labels -->
	<div style="display: flex; justify-content: space-between; width: 100%;">
		{#each STEP_LABELS as label, i}
			{@const state = getStepState(i)}
			<span
				style="font-size: 13px; font-family: 'JetBrains Mono', 'Fira Code', monospace; transition: color 0.3s;
					color: {state === 'error' ? '#ef4444' : state === 'pending' ? 'rgba(0,0,0,0.25)' : 'rgba(0,0,0,0.5)'};"
			>
				{label}
			</span>
		{/each}
	</div>

	<!-- Track -->
	<div style="position: relative; height: 8px; width: 100%; border-radius: 9999px; overflow: hidden; background: rgba(0,0,0,0.1);">
		<div
			style="position: absolute; inset: 0; left: 0; width: {smoothProgress}%;
				border-radius: 9999px;
				background: {visualStep === 'error' ? '#ef4444' : '#191919'};"
		></div>
		{#each [25, 50, 75] as pct}
			<div
				style="position: absolute; top: 0; bottom: 0; left: {pct}%; width: 3px; background: #f7f4f2;"
			></div>
		{/each}
	</div>

	<!-- Description -->
	<div style="font-size: 13px; color: {visualStep === 'error' ? '#ef4444' : 'rgba(0,0,0,0.5)'}; text-align: center; min-height: 1.2em;">
		{visualStep === "error" ? (errorMessage || "Fehler beim Laden") : visualDescription}
	</div>
</div>
