<script lang="ts">
	import SendRequest from "$lib/components/SendRequest.svelte";
	import ShortBio from "$lib/components/ShortBio.svelte";
	import Signatures from "$lib/components/Signatures.svelte";
	import type { PageData } from "./$types";

	let { data }: { data: PageData } = $props();
</script>

<main class="container mx-auto flex flex-col gap-4">
	<Signatures signatures={data.signatures} />
	<div class="grid grid-cols-3 gap-4">
		<div class="col-span-2">
			<ShortBio />
		</div>
		<SendRequest />
	</div>
</main>
