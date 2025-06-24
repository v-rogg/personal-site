<script lang="ts">
	import * as d3 from "d3";
	import { onMount } from "svelte";

	let vis: HTMLElement;
	let { weekStats } = $props();

	let width: number;
	let height: number;

	const margin = {
		top: 30,
		right: 30,
		bottom: 60,
		left: 60,
	};

	console.log("weekStats:", weekStats);

	onMount(() => {
		window.addEventListener("resize", draw);
	});

	$effect(() => {
		draw();
	});

	function processData(weekStats: any[]) {
		// Group by month and aggregate action counts
		const monthlyData = new Map();

		weekStats.forEach((week) => {
			const date = new Date(week.week);
			const monthKey = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}`;

			if (!monthlyData.has(monthKey)) {
				monthlyData.set(monthKey, {
					month: monthKey,
					date: new Date(date.getFullYear(), date.getMonth(), 1),
					actions: new Map(),
				});
			}

			const monthData = monthlyData.get(monthKey);
			week.actions.forEach((action) => {
				const currentCount = monthData.actions.get(action.name) || 0;
				monthData.actions.set(action.name, currentCount + action.count);
			});
		});

		// Convert to array format suitable for D3 stack
		const processedData = Array.from(monthlyData.values()).map((month) => {
			const obj: any = { month: month.month, date: month.date };
			month.actions.forEach((count, actionName) => {
				obj[actionName] = count;
			});
			return obj;
		});

		// Sort by date
		processedData.sort((a, b) => a.date - b.date);

		return processedData;
	}

	function draw(): void {
		if (!weekStats || weekStats.length === 0) return;

		d3.select(vis).html(null);

		const node = d3.select(vis).node();
		if (!node) return;

		width = node.getBoundingClientRect().width - margin.left - margin.right;
		height = node.getBoundingClientRect().height - margin.top - margin.bottom;

		const data = processData(weekStats);

		// Get all unique action names
		const actionNames = Array.from(new Set(weekStats.flatMap((week) => week.actions.map((action) => action.name))));

		// Create color scale
		const color = d3.scaleOrdinal().domain(actionNames).range(d3.schemeCategory10.concat(d3.schemeSet3));

		// Create stack generator
		const stack = d3
			.stack()
			.keys(actionNames)
			.value((d, key) => d[key] || 0);

		const stackedData = stack(data);

		const svg = d3
			.select(vis)
			.append("svg")
			.attr("width", width + margin.left + margin.right)
			.attr("height", height + margin.top + margin.bottom)
			.append("g")
			.attr("transform", `translate(${margin.left}, ${margin.top})`);

		// Create scales
		const xScale = d3
			.scaleBand()
			.domain(data.map((d) => d.month))
			.range([0, width])
			.padding(0.1);

		const yScale = d3
			.scaleLinear()
			.domain([0, d3.max(stackedData[stackedData.length - 1], (d) => d[1]) || 0])
			.range([height, 0]);

		// Create and add the bars
		svg.selectAll("g.layer")
			.data(stackedData)
			.enter()
			.append("g")
			.attr("class", "layer")
			.attr("fill", (d) => color(d.key))
			.selectAll("rect")
			.data((d) => d)
			.enter()
			.append("rect")
			.attr("x", (d) => xScale(d.data.month) || 0)
			.attr("y", (d) => yScale(d[1]))
			.attr("height", (d) => yScale(d[0]) - yScale(d[1]))
			.attr("width", xScale.bandwidth())
			.append("title")
			.text((d) => `${d.data.month}: ${stackedData.find((s) => s.some((item) => item === d))?.key || ""} - ${d[1] - d[0]}`);

		// Add X axis
		svg.append("g")
			.attr("transform", `translate(0, ${height})`)
			.call(d3.axisBottom(xScale))
			.selectAll("text")
			.style("text-anchor", "end")
			.attr("dx", "-.8em")
			.attr("dy", ".15em")
			.attr("transform", "rotate(-45)");

		// Add Y axis
		svg.append("g").call(d3.axisLeft(yScale));

		// Add Y axis label
		svg.append("text")
			.attr("transform", "rotate(-90)")
			.attr("y", 0 - margin.left)
			.attr("x", 0 - height / 2)
			.attr("dy", "1em")
			.style("text-anchor", "middle")
			.text("Action Count");

		// Add legend
		const legend = svg
			.append("g")
			.attr("class", "legend")
			.attr("transform", `translate(${width - 150}, 20)`);

		const legendItems = legend
			.selectAll(".legend-item")
			.data(actionNames.slice(0, 10)) // Limit to first 10 for space
			.enter()
			.append("g")
			.attr("class", "legend-item")
			.attr("transform", (d, i) => `translate(0, ${i * 20})`);

		legendItems
			.append("rect")
			.attr("width", 15)
			.attr("height", 15)
			.attr("fill", (d) => color(d));

		legendItems
			.append("text")
			.attr("x", 20)
			.attr("y", 12)
			.style("font-size", "12px")
			.text((d) => (d.length > 20 ? d.substring(0, 20) + "..." : d));
	}
</script>

<section class="container">
	<div class="bg-white-600 rounded-xl p-8 min-h-128">
		<h3 class="text-balance text-3xl font-semibold">Schonmal gefragt, wie Nutzer meine Seite nutzen?</h3>
		<p>Wahrscheinlich nicht. Aber hier bekommst du die aktuelle Statistik ☺️</p>
		<div id="vis" bind:this={vis} style="width: 100%; height: 500px;"></div>
	</div>
</section>

<style>
	#vis {
		width: 100%;
		height: 500px;
	}
</style>
