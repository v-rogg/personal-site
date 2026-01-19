<script lang="ts">
import { browser } from "$app/environment";
import type { AnalyticsStats } from "./analytics";

let { stats: initialStats }: { stats: AnalyticsStats } = $props();
let timeframe = "d180";
let stats = $state(initialStats);

// Format numbers nicely
function formatNumber(num: number): string {
	if (num === null || num === undefined || Number.isNaN(num)) return "0";
	return (Math.round(num * 100) / 100).toLocaleString("de-DE");
}

function formatInteger(num: number): string {
	if (num === null || num === undefined || Number.isNaN(num)) return "0";
	return Math.floor(num).toLocaleString("de-DE");
}

function formatPercentage(num: number): string {
	if (num === null || num === undefined || Number.isNaN(num)) return "0";
	return (Math.round(num * 10) / 10).toLocaleString("de-DE");
}

function formatTime(seconds: number): string {
	if (seconds === null || seconds === undefined || Number.isNaN(seconds) || seconds <= 0)
		return "0s";
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = Math.floor(seconds % 60);
	if (minutes > 0) {
		return `${minutes}m ${remainingSeconds}s`;
	}
	return `${remainingSeconds}s`;
}
</script>



<section id="analytics" class="mt-20">
	<div class="mb-4 flex flex-col items-end gap-12 justify-end lg:flex-row">
		<!-- <div class="flex h-max flex-wrap items-end justify-between gap-2 max-md:px-10 lg:justify-end"> -->
		<p class="text-black">
			Als Data Engineer heißt es nicht nur Daten sammeln, sondern auch verstehen!
			<br/>
			Hier siehst du, was mein anonym gesammelten Zeichen-Events über das Verhalten der Künstler verrät.
		</p>
		<!-- </div> -->
		<h2
			class="-mb-[0.5em] block w-max text-[4rem] font-bold leading-none tracking-tight text-white-600 max-lg:-mt-10 max-md:pr-10 sm:text-[6rem] md:-mb-[0.14em] md:text-[10rem] lg:-ml-[8px] xl:-ml-[7px] 2xl:-ml-[12px] 2xl:text-[16rem]"
		>
			Statistik
		</h2>
	</div>



	<div class="flex grid-cols-1 flex-col gap-4 xl:grid xl:grid-cols-8">
		<!-- Editor Conversion Rate -->
		<div class="group col-span-2 relative flex flex-col bg-white-600 pl-10 pr-10 pt-9 pb-10 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl">
			<div class="text-center">
				<div class="text-xl font-semibold text-black mb-2">
					Editor Conversion
				</div>
				<div class="text-5xl font-bold text-black mb-3">
					{formatPercentage(stats.conversion_rate || 0)}%
				</div>
				<div class="text-sm text-black opacity-70">
					öffnen → speichern
				</div>
			</div>

				<!-- Fun annotation - only visible on hover -->
				<span
					class="absolute w-max rounded-lg border-4 border-white bg-white-700 px-4 py-3 text-center font-[450] leading-tight text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none z-50 max-sm:hidden"
					style="top: -90px; right: -100px; transform: rotateZ(8deg);">
					<span class="relative z-10 whitespace-pre-line text-black">
						{stats.conversion_rate > 0
							? `Von ${stats.conversion_step_1 || 0} Editor-Öffnungen\nwurden ${stats.conversion_step_2 || 0} gespeichert! 🎯`
							: "Editor wird fleißig\ngenutzt! 🎨"}
					</span>
				</span>
				<!-- Arrow -->
				<span
					class="pointer-events-none absolute opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-50 max-sm:hidden"
					style="top: 70px; right: 80px; transform: rotate(-45deg);">
					<svg
						width="28"
						viewBox="0 0 67 32"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						class="fill-gray-600"
						><path
							d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
						></path
						></svg>
				</span>
			</div>

		<!-- Longest Painting Duration -->
		{#if stats.has_drawing_data && stats.drawing_durations.length > 0}
			<a
				href="/?s={stats.drawing_durations[0].id}"
				class="group col-span-2 relative flex flex-col bg-white-600 pl-10 pr-10 pt-9 pb-10 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl no-underline cursor-pointer"
				target="_blank"
			>
				<div class="text-center">
					<div class="text-xl font-semibold text-black mb-2">
						Längste Mal-Session
					</div>
					<div class="text-5xl font-bold text-black mb-3">
						{formatTime(stats.drawing_durations[0].duration_seconds)}
					</div>
					<div class="text-sm text-black opacity-70">
						in den letzten 180 Tagen
					</div>
				</div>

				<!-- Fun annotation - only visible on hover -->
				<span
					class="absolute w-max rounded-lg border-4 border-white bg-white-700 px-4 py-3 text-center font-[450] leading-tight text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none z-50 max-sm:hidden"
					style="top: -80px; left: -80px; transform: rotateZ(-10deg);">
					<span class="relative z-10 whitespace-pre-line text-black">
						Klick mich! Hier ist die
längste Mal-Session! 🎨👆
					</span>
				</span>
				<!-- Arrow -->
				<span
					class="pointer-events-none absolute opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-50 max-sm:hidden"
					style="top: 65px; left: 60px; transform: rotate(45deg);">
					<svg
						width="28"
						viewBox="0 0 67 32"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						class="fill-gray-600"
						><path
							d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
						></path
						></svg>
				</span>
			</a>
		{:else}
			<div class="group col-span-2 relative flex flex-col bg-white-600 pl-10 pr-10 pt-9 pb-10 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl">
				<div class="text-center">
					<div class="text-xl font-semibold text-black mb-2">
						Längste Mal-Session
					</div>
					<div class="text-5xl font-bold text-black mb-3">
						(-)
					</div>
					<div class="text-sm text-black opacity-70">
						in den letzten 180 Tagen
					</div>
				</div>

				<!-- Fun annotation - only visible on hover -->
				<span
					class="absolute w-max rounded-lg border-4 border-white bg-white-700 px-4 py-3 text-center font-[450] leading-tight text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none z-50 max-sm:hidden"
					style="top: -80px; left: -80px; transform: rotateZ(-10deg);">
					<span class="relative z-10 whitespace-pre-line text-black">
						Noch keine Daten
verfügbar! 📊
					</span>
				</span>
				<!-- Arrow -->
				<span
					class="pointer-events-none absolute opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-50 max-sm:hidden"
					style="top: 65px; left: 60px; transform: rotate(45deg);">
					<svg
						width="28"
						viewBox="0 0 67 32"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						class="fill-gray-600"
						><path
							d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
						></path
						></svg>
				</span>
			</div>
		{/if}

		<!-- Average Drawing Time -->
		<div class="group col-span-2 relative flex flex-col bg-white-600 pl-10 pr-10 pt-9 pb-10 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl">
			<div class="text-center">
				<div class="text-xl font-semibold text-black mb-2">
					Zeichenzeit
				</div>
				<div class="text-5xl font-bold text-black mb-3">
					{formatTime(stats.average_drawing_time_seconds || 0)}
				</div>
				<div class="text-sm text-black opacity-70">
					durchschnittlich pro Mal-Einheit
				</div>
			</div>

				<!-- Fun annotation - only visible on hover -->
				<span
					class="absolute w-max rounded-lg border-4 border-white bg-white-700 px-4 py-3 text-center font-[450] leading-tight text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none z-50 max-sm:hidden"
					style="bottom: -75px; left: -85px; transform: rotateZ(15deg);">
					<span class="relative z-10 whitespace-pre-line text-black">
						{stats.average_drawing_time_seconds > 120
							? "Kunstwerke brauchen\nihr Zeit! ⏰"
							: stats.average_drawing_time_seconds > 0
								? "Schnelle Künstler\nam Werk! ⚡"
								: "Zeit spielt keine\nRolle bei Kunst! 🎨"}
					</span>
				</span>
				<!-- Arrow -->
				<span
					class="pointer-events-none absolute opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-50 max-sm:hidden"
					style="bottom: 60px; left: 65px; transform: rotate(135deg);">
					<svg
						width="28"
						viewBox="0 0 67 32"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						class="fill-gray-600"
						><path
							d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
						></path
						></svg>
				</span>
			</div>

		<!-- Average Eraser Uses -->
		<div class="group col-span-2 relative flex flex-col bg-white-600 pl-10 pr-10 pt-9 pb-10 backdrop-blur transition duration-500 hover:shadow-xl sm:rounded-xl">
			<div class="text-center">
				<div class="text-xl font-semibold text-black mb-2">
					Radierer-Nutzungen
				</div>
				<div class="text-5xl font-bold text-black mb-3">
					{formatNumber(stats.average_eraser_uses || 0)}
				</div>
				<div class="text-sm text-black opacity-70">
					pro Zeichnung durchschnittlich
				</div>
			</div>

				<!-- Fun annotation - only visible on hover -->
				<span
					class="absolute w-max rounded-lg border-4 border-white bg-white-700 px-4 py-3 text-center font-[450] leading-tight text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none z-50 max-sm:hidden"
					style="bottom: -80px; right: -90px; transform: rotateZ(-12deg);">
					<span class="relative z-10 whitespace-pre-line text-black">
						{stats.average_eraser_uses > 1
							? "Perfektionisten unter\nsich entdeckt! ✏"
							: stats.average_eraser_uses > 0
								? "Kleine Korrekturen\ngehören dazu! 🎯"
								: "Perfekt auf Anhieb\noder noch keine Daten! ✨"}
					</span>
				</span>
				<!-- Arrow -->
				<span
					class="pointer-events-none absolute opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-50 max-sm:hidden"
					style="bottom: 65px; right: 70px; transform: rotate(-135deg);">
					<svg
						width="28"
						viewBox="0 0 67 32"
						version="1.1"
						xmlns="http://www.w3.org/2000/svg"
						style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
						class="fill-gray-600"
						><path
							d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
						></path
						></svg>
				</span>
			</div>
		</div>

	<div class="mt-8 text-center text-base text-black opacity-70">
		{#if stats.has_drawing_data && stats.drawing_durations.length > 0}
			<p>Basierend auf Daten der letzten 180 Tage • {formatInteger(stats.conversion_step_1 || 0)} Editor-Öffnungen insgesamt</p>
		{:else}
			<p>Zeichen-Sessions werden noch gesammelt...</p>
		{/if}
	</div>
</section>

<style>
	/* Analytics specific styles if needed */
</style>
