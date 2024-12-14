<script lang="ts">
	// import Memory from "$lib/components/Memory.svelte";
	// import Project from "$lib/components/Project.svelte";
	import SendRequest from "$lib/components/SendRequest.svelte";
	import ShortBio from "$lib/components/ShortBio.svelte";
	import Signatures from "$lib/components/Signatures.svelte";
	import type { PageData } from "./$types";

	let { data }: { data: PageData } = $props();
</script>

<Signatures signatures={data.signatures} />
<div class="grid grid-cols-9 gap-4">
	<ShortBio cls="col-span-6" />
	<SendRequest cls="col-span-3" />
	<!-- <Project />
	<Memory
		image="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/69e7aabe-3fdf-4839-d45b-c3f612a69000/h=500"
		alt="Valentin und Alicia für AVAID bei Augsburg gründet!"
		tag="Augsburg gründet!"
		icon="fa-solid fa-comments"
		position="top-right"
		cls="col-span-3"
	/>
	<Memory
			image="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/df11e483-dc0c-42fb-f0a1-6782eb1e7200/h=500"
			alt="Quadfahren in Spanien"
			tag="San Miguel del Salinas | Spanien"
			icon="fa-solid fa-person-biking-mountain"
			position="top-right"
			cls="col-span-3"
		/>
	<Memory
		image="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/f012824f-34be-4135-7546-80dacfff3200/h=500"
		alt="Entspannen in Frankreich"
		tag="Paris | Frankreich"
		icon="fa-solid fa-child-reaching"
		position="bottom-left"
		cls="col-span-3"
	/>
	<Memory
		image="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/57f8d2ac-9d5a-497a-b51d-c2f997271f00/h=500"
		alt="Spazieren in der Niederlande"
		tag="Amsterdam | Niederlande"
		icon="fa-solid fa-person-walking"
		position="bottom-right"
		cls="col-span-2"
	/> -->
</div>
