<script lang="ts">
import { trackEvent } from "$lib/tracking";

let scrollY = $state(0);
let over100h = $derived(scrollY > 0);

function handleScroll() {
	scrollY = window.scrollY;
}

$effect(() => {
	window.addEventListener("scroll", handleScroll);
	return () => window.removeEventListener("scroll", handleScroll);
});
</script>

<div class="mt-16 flex justify-center">
	<button
		onclick={() => {
			window.scrollTo({ top: 0, behavior: "smooth" });
			trackEvent("click.back-to-top");
		}}
		class="rounded-md px-2 py-1 transition hover:bg-white-600 active:bg-white-700 {over100h ? 'opacity-100' : 'opacity-0'}"
		aria-label="Zurück nach oben"
	>
		<i class="fa-solid fa-arrow-up mr-2"></i>
		Nach oben
	</button>
</div>
