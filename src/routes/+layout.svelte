<script lang="ts">
	import { page } from "$app/stores";
	import { blur } from "svelte/transition";
	import Footer from "$lib/components/globals/Footer.svelte";
	import Header from "$lib/components/globals/Header.svelte";
	import "../app.css";
	import "@awesome.me/kit-fb271958c9/icons/css/all.min.css";
	import { expoOut } from "svelte/easing";

	let { children } = $props();
</script>

<Header />
{#key $page.url.pathname}
	<div in:blur={{ delay: 100, duration: 500, amount: 10, easing: expoOut }}>
		{@render children()}
	</div>
{/key}
<Footer />
