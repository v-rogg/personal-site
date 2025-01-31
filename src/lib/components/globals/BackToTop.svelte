<script lang="ts">
	import posthog from "posthog-js";
	import { scrollY } from "svelte/reactivity/window";

	let over100h = $derived.by(() => {
		return scrollY.current ? scrollY.current > 0 : false;
	});
</script>

<div class="mt-16 flex justify-center">
	<button
		onclick={() => {
			window.scrollTo({ top: 0, behavior: "smooth" });
			posthog.capture("click.back-to-top");
		}}
		class:opaque={over100h}
		class="rounded-md px-2 py-1 opacity-0 transition hover:bg-white-600 active:bg-white-700"
		aria-label="Zurück nach oben"
	>
		<i class="fa-solid fa-arrow-up mr-2"></i>
		Nach oben
	</button>
</div>

<style lang="postcss">
	.opaque {
		opacity: 100;
	}
</style>
