<script lang="ts">
	let { children } = $props();
</script>

<li class="my-0 inline-flex items-center gap-0.5 rounded-md bg-white-700 px-3 py-2 text-sm font-medium leading-none">
	{@render children?.()}
</li>
