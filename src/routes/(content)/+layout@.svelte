<script lang="ts">
	const { children } = $props();
</script>

<main class="container mx-auto mt-24 px-2 sm:mt-40 sm:px-10">
	<article
		class="rounded-images prose mx-auto max-w-4xl text-wrap text-justify prose-headings:text-left prose-h1:mb-2 prose-h2:mb-2
		prose-h2:mt-0 prose-h3:mt-14 prose-h3:border-b prose-h3:border-b-white-700 prose-h3:pb-2 prose-h3:text-2xl prose-h4:text-lg
		prose-li:marker:text-white-700 prose-hr:my-4 prose-hr:border-white-700"
	>
		{@render children()}
	</article>
</main>

<style lang="postcss">
	:global(.rounded-images img, .rounded-images video, .rounded-images iframe) {
		border-radius: 4px;
	}
</style>
