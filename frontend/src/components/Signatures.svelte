<script lang="ts">
import type { Signature, SignatureMeta } from "$lib/api";
import { trackEvent } from "$lib/tracking";
import SignatureCarousel from "./Signatures/SignatureCarousel.svelte";
import SignatureEditor from "./Signatures/SignatureEditor.svelte";

let {
	signatures,
	preloadedSignatures = [],
	autoplay
}: {
	signatures: SignatureMeta[];
	preloadedSignatures?: Signature[];
	autoplay: boolean;
} = $props();

let editMode = $state(false);

function openEditMode() {
	editMode = true;
	trackEvent("click.signatures.editor.open");
}

function closeEditMode() {
	editMode = false;
	trackEvent("click.signatures.editor.close");
}
</script>

<section class="container mx-auto max-sm:overflow-hidden max-sm:pb-4">
	<div class="relative h-[650px] min-w-full bg-white-600 max-sm:mb-12 sm:mt-8 sm:rounded-xl">
		<img
			src="/files/images/c6f4adce-577c-406a-f795-6b0892730a00.jpg"
			alt="Valentin Rogg"
			class="absolute bottom-0 left-[50%] mx-auto block max-w-[unset] translate-x-[-50%] pt-10 brightness-105"
			style="max-height: 650px; width: auto;"
		/>
		{#if !editMode}
			<SignatureCarousel
				bind:signatures={signatures}
				{preloadedSignatures}
				{autoplay}
				{openEditMode}
			/>
		{:else}
			<SignatureEditor bind:signatures={signatures} {closeEditMode} />
		{/if}
	</div>
</section>
