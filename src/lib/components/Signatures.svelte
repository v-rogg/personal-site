<script lang="ts">
	import SignatureCarousel from "$lib/components/Signatures/SignatureCarousel.svelte";
	import SignatureEditor from "$lib/components/Signatures/SignatureEditor.svelte";
	import posthog from "posthog-js";

	let { signatures, autoplay } = $props();

	let editMode = $state(false);

	function openEditMode() {
		editMode = true;
		posthog.capture("click.signatures.editor.open");
	}

	function closeEditMode() {
		editMode = false;
		posthog.capture("click.signatures.editor.close");
	}
</script>

<section class="container mx-auto max-sm:overflow-hidden max-sm:pb-4">
	<div class="relative h-[650px] min-w-full bg-white-600 max-sm:mb-12 sm:mt-8 sm:rounded-xl">
		<img
			src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/c6f4adce-577c-406a-f795-6b0892730a00/h=610"
			alt="Valentin Rogg"
			class="absolute bottom-0 left-[50%] mx-auto block h-auto max-w-[unset] translate-x-[-50%] pt-10 brightness-105"
			height="650"
		/>
		{#if !editMode}
			<SignatureCarousel bind:signatures={signatures} autoplay={autoplay} openEditMode={openEditMode} />
		{:else}
			<SignatureEditor bind:signatures={signatures} closeEditMode={closeEditMode} />
		{/if}
	</div>
</section>
