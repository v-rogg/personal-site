<script lang="ts">
	export let blok;
</script>

<div>
	<a
		target="_blank"
		href="{blok.url.url}"
		class="block max-w-max px-2 py-1 rounded-lg bg-white-500 border border-white-700 backdrop-blur-2xl dark:bg-blue-900 dark:border-blue-700 dark:text-white-500 hover:bg-white-700 active:bg-white-600 dark:hover:bg-blue-700 dark:active:bg-blue-800"
		>{blok.text}</a>
</div>
