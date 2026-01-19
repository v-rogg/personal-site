<script lang="ts">
import Socials from "./globals/Socials.svelte";

let { cls }: { cls?: string } = $props();
</script>

<section class="rounded-xl {cls}">
	<div class="flex min-h-128 flex-col justify-between px-10 py-10">
		<div>
			<h1 class="relative mb-4 flex w-fit flex-wrap items-end gap-2 text-left font-sans text-4xl font-semibold">
				Valentin Rogg<span class="relative bottom-0.5 text-sm">M.A. B.Sc.</span>
			</h1>
			<p class="text-justify text-black" lang="de">
				Hey, ich bin Valentin. Als studierter <b>Software Engineer</b> und <b>Digital Product Designer</b> spezialisiere ich mich auf die Konzeption,
				Umsetzung und Vermarktung von digitalen Produkten, Services und hardware&shy;basierter Sensor&shy;technologie. Mein Fokus liegt dabei vor allem
				auf Themen der auto&shy;matisierten Daten&shy;analyse und interaktiven Visualisierung. Außerdem liegen meine Stärken im Management und
				Marketing. Aktuell arbeite ich als <b>Data Engineer</b> bei
				<a href="https://credium.de" target="_blank" rel="noopener noreferrer" class="underline">credium</a>.
				<br /><br />Privat bin ich leidenschaftlicher <a href="https://singtonic.net" target="_blank" rel="noopener noreferrer" class="underline">Musiker</a> und
				<a
					href="https://anmeldung.sport.uni-augsburg.de/angebote/aktueller_zeitraum/_Kleine_Spiele.html"
					target="_blank"
					rel="noopener noreferrer"
					class="underline">Sportkursleiter</a
				>.
			</p>
			<!-- <p class="mt-2 text-pretty text-justify text-black"> -->
			<!-- Auf meiner Seite gebe ich Einblick in meine Projekte und erzähle von meinen Learnings der letzten fünf Jahren im Startup-Umfeld. -->
			<!-- </p> -->
		</div>
		<Socials />
	</div>
</section>
