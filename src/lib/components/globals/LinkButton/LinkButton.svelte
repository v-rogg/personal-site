<script lang="ts">
	export let blok;
</script>

<div>
	<a
		href="{blok.url.url}"
		class="block max-w-max px-2 py-1 rounded-lg bg-white-500 border border-white-700 backdrop-blur-2xl dark:bg-blue-700 dark:border-blue-650 dark:text-white-500"
		>{blok.text}</a>
</div>
