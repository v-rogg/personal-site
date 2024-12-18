<script lang="ts" context="module">
	export type ToastData = {
		title: string;
		description: string;
	};

	const {
		elements: { content, title },
		helpers,
		states: { toasts },
		actions: { portal }
	} = createToaster<ToastData>();

	export const addToast = helpers.addToast;
</script>

<script lang="ts">
	import { createToaster } from "@melt-ui/svelte";
	import { fly } from "svelte/transition";
</script>

<div use:portal class="absolute bottom-4 right-0 z-[60] flex flex-col items-end gap-2 overflow-hidden">
	{#each $toasts as { id, data } (id)}
		<div
			{...$content(id)}
			use:content
			class="mr-4 rounded-lg bg-white p-4 shadow-md"
			in:fly={{ duration: 150, x: "100%" }}
			out:fly={{ duration: 150, x: "100%" }}
		>
			<div>
				<div>
					<h3 {...$title(id)} use:title class="flex items-center">
						<i class="{data.description} mr-2"></i>
						{data.title}
					</h3>
				</div>
			</div>
		</div>
	{/each}
</div>
