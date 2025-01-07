<script lang="ts">
	const { children } = $props();
</script>

<main class="container mx-auto mt-32 px-10">
	<article class="rounded-images prose mx-auto max-w-4xl">
		{@render children()}
	</article>
</main>

<style lang="postcss">
	:global(.rounded-images img, .rounded-images video, .rounded-images iframe) {
		border-radius: 4px;
	}
</style>
