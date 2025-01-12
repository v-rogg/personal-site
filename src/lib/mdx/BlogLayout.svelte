<script lang="ts">
	import TagList from "$lib/components/globals/TagList.svelte";

	let { title, subtitle, tags, date, timeframe = [], children } = $props();
</script>

<div class="-mx-10 rounded-xl bg-white-600 px-10 py-10">
	<h1>{title}</h1>

	<h2 class="text-wrap">{subtitle}</h2>

	<TagList tags={tags} />

	<table class="mb-0 w-max py-0 text-sm prose-td:py-1">
		<tbody>
			<tr>
				<td class="w-min whitespace-nowrap"> Veröffentlichung: </td>
				<td>
					<i class="fa-solid fa-calendar mr-1 text-white-700"></i>
					{new Date(date).toLocaleDateString("de-DE", { day: "numeric", month: "short", year: "numeric" })}
				</td>
			</tr>
			{#if timeframe.length > 0}
				<tr>
					<td class="w-min whitespace-nowrap">Projektzeitraum: </td>
					<td>
						<i class="fa-solid fa-calendar mr-1 text-white-700"></i>
						{new Date(timeframe[0]).toLocaleDateString("de-DE", { month: "short", year: "numeric" })}
						<i class="fa-regular fa-arrow-right-long mx-1"></i>
						<i class="fa-solid fa-calendar mr-1 text-white-700"></i>
						{new Date(timeframe[1]).toLocaleDateString("de-DE", { month: "short", year: "numeric" })}
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>

<div class="mt-10">
	{@render children()}
</div>
