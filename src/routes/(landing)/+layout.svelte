<script lang="ts">
	let { children } = $props();
	import { page } from "$app/stores";
</script>

<main class="container relative mx-auto flex flex-col gap-4" class:m-32={$page.url.pathname !== "/"}>
	{@render children()}
</main>
