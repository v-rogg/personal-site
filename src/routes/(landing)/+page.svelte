<script lang="ts">
	import Analytics from "$lib/components/Analytics/Analytics.svelte";
	// import Blog from "$lib/components/Blog/Blog.svelte";
	// import Memory from "$lib/components/Memory.svelte";
	import SendRequest from "$lib/components/SendRequest.svelte";
	import ShortBio from "$lib/components/ShortBio.svelte";
	import Signatures from "$lib/components/Signatures.svelte";
	import type { PageData } from "./$types";

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Valentin Rogg</title>
	<meta
		name="description"
		content="Als studierter Entwickler und Digital Product Designer spezialisiere ich mich auf die Konzeption, Umsetzung und Vermarktung von digitalen Produkten, Services und hardwarebasierter Sensortechnologie. Mein Fokus liegt dabei vor allem auf Themen der automatisierten Datenanalyse und interaktiven Visualisierung. Außerdem liegen meine Stärken im Management und Marketing.
Privat bin ich Sportkursleiter und leidenschaftlicher Musiker."
	/>
</svelte:head>

<Signatures signatures={data.signatures} autoplay={data.autoplay} />
<div class="grid grid-cols-1 gap-4 lg:grid-cols-2 xl:grid-cols-9">
	<ShortBio cls="xl:col-span-5" />
	<SendRequest cls="xl:col-span-4" />
</div>

<Analytics weekStats={data.weekStats} />

<!-- {#if data.blog && data.blog.length > 0}
	<Blog blog={data.blog} tags={data.allTags} />
{/if} -->
