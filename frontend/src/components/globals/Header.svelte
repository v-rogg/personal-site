<script lang="ts">
import { expoIn, expoOut } from "svelte/easing";
import { blur } from "svelte/transition";

let pathname = $state(typeof window !== "undefined" ? window.location.pathname : "/");

let slugs = $derived.by(() => {
	const segments = pathname.split("/").filter(Boolean);
	return segments.map((_, index) => `/${segments.slice(0, index + 1).join("/")}`);
});

// Listen for Astro View Transitions navigation
$effect(() => {
	if (typeof window !== "undefined" && typeof document !== "undefined") {
		const handleNavigation = () => {
			pathname = window.location.pathname;
		};

		// Astro View Transitions events
		document.addEventListener("astro:after-swap", handleNavigation);
		// Fallback for browser navigation
		window.addEventListener("popstate", handleNavigation);

		return () => {
			document.removeEventListener("astro:after-swap", handleNavigation);
			window.removeEventListener("popstate", handleNavigation);
		};
	}
});
</script>

<header
	class="pointer-events-none absolute z-50 w-full max-w-[100vw] -translate-y-0.5 overflow-hidden bg-transparent transition sm:top-8"
	class:floating-header={pathname !== "/"}>
	<div class="container relative mx-auto mb-10 mt-8 flex items-center justify-between px-2 sm:px-10">
		<div class="flex h-10 items-center gap-4">
			{#if pathname === "/"}
				<span class="flex size-10 items-center justify-center rounded">
					<svg
						width="40"
						height="40"
						viewBox="0 0 717 500"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
						<path
							d="M327.107,499.126L174.648,499.126L-0,0L143.395,0C143.395,0 238.714,326.09 247.532,356.259C247.918,357.58 249.13,358.489 250.508,358.489L250.942,358.489C251.98,358.489 252.938,358.054 253.617,357.335L257.56,346.128C276.369,289.039 360.886,51.774 376.768,7.227L379.31,0L519.187,0C627.964,0 716.277,72.692 716.277,197.09C716.277,254.796 691.424,306.743 651.846,342.801L706.545,499.126L563.151,499.126L532.347,393.747C532.317,393.749 532.287,393.751 532.256,393.753C521.454,356.8 509.169,314.77 497.754,275.721C497.495,274.833 497.669,273.875 498.224,273.134C498.778,272.394 499.65,271.958 500.575,271.958C507.03,271.958 519.187,271.958 519.187,271.958C560.508,271.958 594.055,238.411 594.055,197.09C594.055,155.769 560.508,122.222 519.187,122.222L459.37,122.222L327.107,499.126Z"
							style="fill:currentColor" />
					</svg>
				</span>
			{:else}
				<a
					class="pointer-events-auto flex size-10 items-center justify-center rounded hover:text-white-700 active:text-white-600"
					href="/"
					aria-label="Home">
					<svg
						width="40"
						height="40"
						viewBox="0 0 717 500"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
						<path
							d="M327.107,499.126L174.648,499.126L-0,0L143.395,0C143.395,0 238.714,326.09 247.532,356.259C247.918,357.58 249.13,358.489 250.508,358.489L250.942,358.489C251.98,358.489 252.938,358.054 253.617,357.335L257.56,346.128C276.369,289.039 360.886,51.774 376.768,7.227L379.31,0L519.187,0C627.964,0 716.277,72.692 716.277,197.09C716.277,254.796 691.424,306.743 651.846,342.801L706.545,499.126L563.151,499.126L532.347,393.747C532.317,393.749 532.287,393.751 532.256,393.753C521.454,356.8 509.169,314.70 497.754,275.721C497.495,274.833 497.669,273.875 498.224,273.134C498.778,272.394 499.65,271.958 500.575,271.958C507.03,271.958 519.187,271.958 519.187,271.958C560.508,271.958 594.055,238.411 594.055,197.09C594.055,155.769 560.508,122.222 519.187,122.222L459.37,122.222L327.107,499.126Z"
							style="fill:currentColor" />
					</svg>
				</a>
			{/if}
			<div class="flex items-center gap-1 max-[320px]:hidden">
				{#each slugs as p, i (p)}
					{@const name = p.split("/").at(-1) || ""}
					{@const delay = 100}
					{@const duration = 500}
					{@const amount = 10}
					<div
						class="w-max"
						in:blur={{ delay: i * delay, duration, amount, easing: expoOut }}
						out:blur={{ delay: (slugs.length + 1 - i) * delay, duration, amount, easing: expoIn }}>
						<i class="fa-solid fa-slash-forward"></i>
						{#if slugs.length - 1 === i}
							<span class="mx-1 overflow-hidden text-ellipsis rounded px-1 py-0.5">
								{name.charAt(0).toUpperCase() + name.slice(1)}
							</span>
						{:else}
							<a
								href={p}
								class="pointer-events-auto overflow-hidden text-ellipsis rounded-md px-2 py-1 hover:bg-white-600 active:bg-white-700">
								{name.charAt(0).toUpperCase() + name.slice(1)}
							</a>
						{/if}
					</div>
				{/each}
			</div>
		</div>
		{#if pathname !== "/"}
			<a
				href="/"
				in:blur={{ duration: 600, amount: 10 }}
				class="pointer-events-auto rounded-md px-2 py-1 hover:bg-white-600 active:bg-white-700 max-sm:hidden">
				<i class="fa-solid fa-arrow-left mr-2"></i>
				Zurück
			</a>
		{/if}
	</div>
</header>

<style>
	.floating-header {
		position: fixed;
		background: rgba(255, 255, 255, 0.4);
		backdrop-filter: blur(8px);
		top: 0;
		padding-top: 0;
	}

	@media (min-width: 640px) {
		.floating-header {
			padding-top: 2rem;
		}
	}

	.floating-header > div {
		margin-top: 0;
		margin-bottom: 0;
		padding-top: 2rem;
		padding-bottom: 2rem;
	}
</style>
