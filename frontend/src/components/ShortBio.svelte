<script lang="ts">
import Socials from "./globals/Socials.svelte";

let { cls }: { cls?: string } = $props();
</script>

<section class="rounded-xl {cls}">
	<div class="flex min-h-128 flex-col justify-between px-10 py-10">
		<div>
			<h1 class="relative mb-4 flex w-fit flex-wrap items-end gap-2 text-left font-sans text-4xl font-semibold">
				Valentin Rogg
			</h1>
			<p class="text-justify text-black" lang="de">
				<b>Geospatial Data Engineer</b> bei
				<a href="https://credium.de" target="_blank" rel="noopener noreferrer" class="underline">credium</a>.
				Daten&shy;infra&shy;struktur, Tooling und Geo&shy;visualisierungen in Rust, Python und Spark.
				Nebenbei <a href="https://singtonic.net" target="_blank" rel="noopener noreferrer" class="underline">Acapella-Gesang</a> und
				<a
					href="https://anmeldung.sport.uni-augsburg.de/angebote/aktueller_zeitraum/_Kleine_Spiele.html"
					target="_blank"
					rel="noopener noreferrer"
					class="underline">Sport</a>.
			</p>
		</div>
		<Socials />
	</div>
</section>
