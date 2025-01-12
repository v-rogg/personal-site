<script lang="ts">
	import { afterNavigate } from "$app/navigation";
	import { blur as fly } from "svelte/transition";
	import { page } from "$app/state";
	import type { NavigationTarget } from "@sveltejs/kit";
	import { expoIn, expoOut } from "svelte/easing";
	import { appState } from "$lib/stores.svelte";

	let navigationHistory: NavigationTarget[] = $state([]);

	let slugs = $derived.by(() => {
		if (page.url) {
			const path = page.url.pathname;
			const segments = path.split("/").filter(Boolean);

			const possibleRoutes = segments.map((_, index) => {
				return "/" + segments.slice(0, index + 1).join("/");
			});

			const currentSlug = appState.metadata?.slug;
			if (currentSlug && page.route.id?.includes("[post]")) {
				possibleRoutes.pop();
				possibleRoutes.push(currentSlug);
			}

			return possibleRoutes;
		} else return [];
	});

	afterNavigate((e) => {
		console.log(e, slugs);
		if (e.from) navigationHistory.push(e.from);
	});
</script>

<header
	class="pointer-events-none absolute z-50 w-full max-w-[100vw] -translate-y-0.5 overflow-hidden bg-transparent transition sm:top-8"
	class:floating-header={page.url.pathname != "/"}
>
	<div class="container mx-auto mb-10 mt-8 flex items-center justify-between px-2 sm:px-10">
		<div class="flex h-10 items-center gap-4">
			{#if page.url.pathname == "/"}
				<span class="flex size-10 items-center justify-center rounded">
					<svg
						width="40"
						height="40"
						viewBox="0 0 717 500"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						><path
							d="M327.107,499.126L174.648,499.126L-0,0L143.395,0C143.395,0 238.714,326.09 247.532,356.259C247.918,357.58 249.13,358.489 250.508,358.489L250.942,358.489C251.98,358.489 252.938,358.054 253.617,357.335L257.56,346.128C276.369,289.039 360.886,51.774 376.768,7.227L379.31,0L519.187,0C627.964,0 716.277,72.692 716.277,197.09C716.277,254.796 691.424,306.743 651.846,342.801L706.545,499.126L563.151,499.126L532.347,393.747C532.317,393.749 532.287,393.751 532.256,393.753C521.454,356.8 509.169,314.77 497.754,275.721C497.495,274.833 497.669,273.875 498.224,273.134C498.778,272.394 499.65,271.958 500.575,271.958C507.03,271.958 519.187,271.958 519.187,271.958C560.508,271.958 594.055,238.411 594.055,197.09C594.055,155.769 560.508,122.222 519.187,122.222L459.37,122.222L327.107,499.126Z"
							style="fill:currentColor"
						></path></svg
					>
				</span>
			{:else}
				<a
					class="pointer-events-auto flex size-10 items-center justify-center rounded hover:text-white-700 active:text-white-600"
					href="/"
					aria-label="Home"
				>
					<svg
						width="40"
						height="40"
						viewBox="0 0 717 500"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						><path
							d="M327.107,499.126L174.648,499.126L-0,0L143.395,0C143.395,0 238.714,326.09 247.532,356.259C247.918,357.58 249.13,358.489 250.508,358.489L250.942,358.489C251.98,358.489 252.938,358.054 253.617,357.335L257.56,346.128C276.369,289.039 360.886,51.774 376.768,7.227L379.31,0L519.187,0C627.964,0 716.277,72.692 716.277,197.09C716.277,254.796 691.424,306.743 651.846,342.801L706.545,499.126L563.151,499.126L532.347,393.747C532.317,393.749 532.287,393.751 532.256,393.753C521.454,356.8 509.169,314.77 497.754,275.721C497.495,274.833 497.669,273.875 498.224,273.134C498.778,272.394 499.65,271.958 500.575,271.958C507.03,271.958 519.187,271.958 519.187,271.958C560.508,271.958 594.055,238.411 594.055,197.09C594.055,155.769 560.508,122.222 519.187,122.222L459.37,122.222L327.107,499.126Z"
							style="fill:currentColor"
						></path></svg
					>
				</a>
			{/if}
			{#if page.status === 200}
				<div class="flex items-center gap-1 max-[320px]:hidden">
					{#each slugs as p, i}
						{@const name = p.split("/").at(-1) || ""}
						{@const delay = 100}
						{@const duration = 500}
						{@const amount = 10}
						<div
							id="{i}+''"
							in:fly={{ delay: i * delay, duration, amount, easing: expoOut }}
							out:fly={{ delay: (slugs.length + 1 - i) * delay, duration, amount, easing: expoIn }}
							class="w-max"
						>
							<i class="fa-solid fa-slash-forward"></i>
							{#if slugs.length - 1 === i}
								<span class="mx-1 overflow-hidden text-ellipsis rounded px-1 py-0.5">
									{name.charAt(0).toUpperCase() + name.slice(1)}
								</span>
							{:else}
								<a
									href={p}
									class="pointer-events-auto overflow-hidden text-ellipsis rounded px-2 py-1 hover:bg-white-700 active:bg-white-600"
								>
									{name.charAt(0).toUpperCase() + name.slice(1)}
								</a>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<div>
					Error {page.status}: {page.error?.message}
				</div>
			{/if}
		</div>
		{#if navigationHistory.length > 0 && page.url.pathname != "/"}
			<a
				href={navigationHistory.at(-1)?.url.href}
				in:fly={{ duration: 600, amount: 10 }}
				class="pointer-events-auto rounded px-2 py-1 hover:bg-white-700 active:bg-white-600 max-sm:hidden"
			>
				<i class="fa-solid fa-arrow-left mr-2"></i>
				Zurück
			</a>
		{/if}
	</div>
</header>

<style lang="postcss">
	.floating-header {
		position: fixed;
		@apply bg-white/40;
		@apply backdrop-blur;
		@apply top-0;
		@apply pt-0;
		@apply sm:pt-8;

		> div {
			@apply mt-0;
			@apply mb-0;
			@apply py-8;
		}
	}
</style>
