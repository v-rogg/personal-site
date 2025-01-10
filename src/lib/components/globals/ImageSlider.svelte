<script lang="ts">
	import { createSlider } from "@melt-ui/svelte";
	import { innerWidth } from "svelte/reactivity/window";

	let { a, b, aAlt = "", bAlt = "" } = $props();

	const {
		elements: { root: hRoot, thumbs: hThumbs },
		states: { value: hValue }
	} = createSlider({
		defaultValue: [50],
		min: 0.1,
		max: 99.9,
		step: 0.1
	});

	const {
		elements: { root: vRoot, thumbs: vThumbs },
		states: { value: vValue }
	} = createSlider({
		defaultValue: [50],
		min: 0.1,
		max: 99.9,
		step: 0.1,
		orientation: "vertical",
		dir: "rtl"
	});
</script>

{#if innerWidth.current && innerWidth.current < 768}
	<div class="relative my-8 h-full w-full overflow-hidden" {...$vRoot} use:vRoot>
		<div
			class="pointer-events-none relative w-full"
			style="clip-path: polygon(0 0, 100% 0, 100% {$vValue}%, 0 {$vValue}%);"
		>
			<img src={a} alt={aAlt} class="relative m-0" />
			<div class="absolute left-4 top-4 z-10 rounded-md bg-white/70 p-2 font-medium leading-none backdrop-blur">
				{aAlt}
			</div>
		</div>
		<div
			class="pointer-events-none absolute top-0"
			style="clip-path: polygon(0 {$vValue}%, 100% {$vValue}%, 100% 100%, 0 100%);"
		>
			<img src={b} alt={bAlt} class=" m-0" />
			<div class="absolute bottom-4 left-4 z-10 rounded-md bg-white/70 p-2 font-medium leading-none backdrop-blur">
				{bAlt}
			</div>
		</div>
		<span
			{...$vThumbs[0]}
			use:vThumbs
			class="pane-vertical absolute left-1/2 h-full cursor-grab active:cursor-grabbing"
		>
			<span
				class="absolute top-1/2 flex size-8 -translate-y-4 translate-x-1/2 items-center justify-center rounded-full bg-white focus:ring-4 focus:!ring-black/40"
			>
				<i class="fa-solid fa-sort"></i>
			</span>
		</span>
	</div>
{:else}
	<div class="relative my-8 h-full max-w-max overflow-hidden" {...$hRoot} use:hRoot>
		<div
			class="pointer-events-none relative w-full max-w-max"
			style="clip-path: polygon(0 0, {$hValue}% 0, {$hValue}% 100%, 0% 100%);"
		>
			<img src={a} alt={aAlt} class="relative m-0 w-full" />
			<div class="absolute left-4 top-4 z-10 rounded-md bg-white/70 p-2 font-medium leading-none backdrop-blur">
				{aAlt}
			</div>
		</div>
		<div
			class="pointer-events-none absolute top-0"
			style="clip-path: polygon({$hValue}% 0, 100% 0, 100% 100%, {$hValue}% 100%);"
		>
			<img src={b} alt={bAlt} class=" m-0" />
			<div class="absolute right-4 top-4 z-10 rounded-md bg-white/70 p-2 font-medium leading-none backdrop-blur">
				{bAlt}
			</div>
		</div>
		<span {...$hThumbs[0]} use:hThumbs class="pane absolute top-0 h-full cursor-grab active:cursor-grabbing">
			<span
				class="absolute top-1/2 flex size-8 -translate-x-1/2 -translate-y-4 items-center justify-center rounded-full bg-white focus:ring-4 focus:!ring-black/40"
			>
				<i class="fa-solid fa-sort fa-rotate-90"></i>
			</span>
		</span>
	</div>
{/if}

<style lang="postcss">
	.pane {
		outline: none;

		&:before {
			content: "";
			position: absolute;
			top: 0;
			left: 0;
			height: 100%;
			width: 2px;
			transform: translateX(-1px);
			@apply bg-white;
		}
	}

	.pane-vertical {
		outline: none;

		&:before {
			content: "";
			position: absolute;
			top: 50%;
			left: -500px;
			height: 2px;
			width: 1000px;
			transform: translateY(-1px);
			@apply bg-white;
		}
	}
</style>
