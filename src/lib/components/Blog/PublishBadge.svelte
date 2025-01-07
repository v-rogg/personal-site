<script lang="ts">
	let { published } = $props();
</script>

{#if !published}
	<div
		class="pointer-events-none absolute right-8 top-8 z-50 rotate-12 rounded-xl border border-red bg-white-500 px-4 py-2 text-2xl font-light text-red"
	>
		Draft
	</div>
{/if}
