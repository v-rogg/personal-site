<script>
	import CVGraph from "$lib/components/sections/CV/CVGraph.svelte";
	import { t } from "$lib/_i18n";

	export let blok;
</script>


<section class="bg-white-600 dark:bg-blue-800 rounded-2xl p-8 col-span-4 h-[25rem] overflow-hidden">
	<h2 class="text-2xl font-semibold text-center mb-10">{$t("cv.title")}</h2>
	<CVGraph blok="{blok}"/>
</section>
