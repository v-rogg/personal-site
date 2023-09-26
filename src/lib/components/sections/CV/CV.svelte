<script lang="ts">
	import { t } from "$lib/_i18n";

	import * as one from "./content/xentral.json";

	let companies = [one];

</script>

<h2 class="text-2xl font-semibold text-center mb-10">{$t("cv.title")}</h2>

<div class="grid grid-cols-3 gap-6">
	<div>

	</div>
	<div class="text-blue-900 dark:text-blue-700">
		<svg width="16" height="872" viewBox="0 0 16 872" xmlns="http://www.w3.org/2000/svg">
			<path d="M7.2926 0.292893C7.68312 -0.0976311 8.31629 -0.0976311 8.70681 0.292893L15.0708 6.65685C15.4613 7.04738 15.4613 7.68054 15.0708 8.07107C14.6802 8.46159 14.0471 8.46159 13.6566 8.07107L8.9997 3.41421L8.99967 872H6.99967L6.9997 3.41421L2.34285 8.07107C1.95232 8.46159 1.31916 8.46159 0.928635 8.07107C0.538111 7.68054 0.538111 7.04738 0.928635 6.65685L7.2926 0.292893Z" fill="currentColor"/>
		</svg>
	</div>
	<div class="flex-col">
		{#each companies as company}
			<div>
				<div class="font-semibold mb-6">{company.name}</div>
				<div class="flex flex-col gap-6">
					{#each company.experiences as experience}
						<div class="experiences flex flex-col gap-1">
							<div>{experience.name}</div>
							<div class="text-sm">{experience.from} <i class="fa-solid fa-arrow-right"></i> {experience.to || "present"}</div>
							<div class="text-sm">{experience.employmentType}</div>
							<div class="flex flex-wrap gap-2 mt-1">
								{#each experience.skills as skill}
									<div class="rounded-lg text-xs px-1 py-0.5 border dark:border-blue-700 text-dark_blue">{skill}</div>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
