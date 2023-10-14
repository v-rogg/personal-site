<script lang="ts">
	import { t } from "$lib/_i18n";
	import type { Company, Experience } from "$lib/storyblok/schema";

	export let blok;

	const heightPerYear = 150;
	const titleOffset = 40;
	const barWidth = 40;

	let present = new Date();

	$: companies = <Company[]>filterHidden(blok.content.companies) || <Company[]>[];
	$: startDate = getStartDate(companies);
	$: graphHeight = ((present - startDate) / (1000 * 60 * 60 * 24 * 365.25)) * heightPerYear + titleOffset;
	$: legendYears = getLegendYears(present, startDate);

	function filterHidden(companies: Company[]) {
		let data = [];
		companies.forEach((company) => {
			if (!company.hidden) data.push(company);
		});
		return data;
	}

	function getLegendYears(present: Date, startDate: Date) {
		let years = [];
		let nextYear: Date = new Date(startDate.getFullYear() + 1, 0, 1, 0, 0);
		while (nextYear < present) {
			years.push(nextYear);
			nextYear = new Date(nextYear.getFullYear() + 1, 0, 1, 0, 0);
		}
		return years;
	}

	function getStartDate(companies: Company[]) {
		let minDate = present;
		companies.forEach((company) => {
			company.jobs.forEach((job) => {
				const date = new Date(job.from);
				if (date < minDate) minDate = date;
			});
		});
		return minDate;
	}

	function getDuration(job: Experience) {
		const to = job.to ? new Date(job.to) : present;
		const from = new Date(job.from);
		return Math.round(((to - from) / (1000 * 60 * 60 * 24 * 365.25)) * heightPerYear);
	}

	function getStartPoint(job: Experience) {
		const to = job.to ? new Date(job.to) : present;
		return Math.round(((present - to) / (1000 * 60 * 60 * 24 * 365.25)) * heightPerYear + titleOffset);
	}

	function getYearOffset(year) {
		return Math.round((present - year) / (1000 * 60 * 60 * 24 * 365.25)) * heightPerYear;
	}
</script>

<h2 class="text-2xl font-semibold text-center mb-10">{$t("cv.title")}</h2>
<div
	class="graph container flex mx-auto justify-center gap-4 mt-28 mb-28 relative"
	style="height: {graphHeight}px; width: {(companies.length + 1) * (barWidth + 16) + 140}px">
	<div class="timetable relative font-semibold">
		<div class="absolute" style="top: {titleOffset}px; left: -80px; transform: translateY(-50%)">{$t("cv.now")}</div>
		{#each legendYears as year}
			<div
				class="absolute legend-year"
				style="top: {getYearOffset(year)}px; left: -80px; transform: translateY(-50%); --companies: {companies.length}">
				{year.getFullYear()}
			</div>
		{/each}
	</div>
	{#each companies as company}
		<div class="relative company" style="width: {barWidth}px">
			<h4 class="absolute text-center company-title font-semibold">
				{@html company.title}
			</h4>
			<div class="absolute jobs z-0" style="width: {barWidth}px">
				{#each company.jobs as job}
					<div
						class="bg-white-700 dark:bg-blue-800 rounded-lg absolute job"
						style="height: {getDuration(job)}px; width: 40px; top: {getStartPoint(job)}px"
						class:studies="{job.tags !== undefined ? job.tags.includes('studies') > 0 : false}">
						<div
							class="job-title pointer-events-none absolute text-sm overflow-ellipsis overflow-hidden text-skin-500 dark:text-blue-600 font-sans">
						</div>
					</div>
				{/each}
			</div>
			<div class="absolute jobs z-50" style="width: {barWidth}px">
				{#each company.jobs as job}
					<div
						class="absolute job"
						style="height: {getDuration(job)}px; width: 40px; top: {getStartPoint(job)}px">
						<div
							class="job-title pointer-events-none absolute text-sm overflow-ellipsis overflow-hidden text-skin-500 dark:text-blue-600 font-sans">
							{@html job.title}
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>

<style lang="postcss">
	.graph {
		overflow-x: clip;
		overflow-y: visible;
	}

	.legend-year:before {
		@apply border;
		@apply border-white-700;
		@apply dark:border-blue-800;
		content: attr(data-companies);
		position: absolute;
		height: 1px;
		width: calc((var(--companies) + 1) * (40px + 1rem));
		border-width: 1px;
		top: 50%;
		left: calc(100% + 20px);
		transform: translateY(-50%);
	}

	.timetable {
		height: 100%;
	}

	.studies {
		--color: theme("colors.white.500");
		background-image: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 15px,
			var(--color) 15px,
			var(--color) 30px
		);
		background-blend-mode: normal;
	}

	:global(.dark) .studies {
		--color: theme("colors.blue.900");
	}

	.hidden {
		opacity: 1;
	}

	.jobs {
		left: 50%;
		transform: translateX(-50%);
	}

	.company:hover .company-title {
		opacity: 1;
	}

	.job:hover .job-title {
		opacity: 1;
	}

	.company-title,
	.job-title {
		transition: opacity ease-in-out 500ms;
		width: max-content;
		transform: translateX(-50%);
	}

	.company-title {
		position: absolute;
		left: 50%;
		display: block;
		transform: rotateZ(-60deg);
		transform-origin: left;
	}

	.job .job-title {
		width: 104px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%) !important;
		text-align: center;
	}
</style>
