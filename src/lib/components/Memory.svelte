<script lang="ts">
	let {
		image,
		alt,
		icon,
		tag,
		cls,
		position = "top-right"
	}: {
		image: string;
		alt: string;
		tag: string;
		icon?: string;
		cls?: string;
		position?: "top-left" | "top-right" | "bottom-left" | "bottom-right" | "none";
	} = $props();
</script>

<div class="relative max-h-128 w-full overflow-hidden rounded-xl {cls}">
	<img src={image} alt={alt} class="h-full w-full object-cover transition-transform duration-1000 hover:scale-105" />
	<div
		class="pointer-events-none absolute z-10 flex items-center gap-2 rounded-md bg-white-500 px-2 py-1 font-medium text-black"
		class:top-left={position === "top-left"}
		class:top-right={position === "top-right"}
		class:bottom-left={position === "bottom-left"}
		class:bottom-right={position === "bottom-right"}
		class:hidden={position === "none"}
	>
		<i class="{icon} text-skin-500"></i>
		{tag}
	</div>
</div>

<style lang="postcss">
	.top-left {
		@apply left-10 top-8;
	}

	.top-right {
		@apply right-10 top-8;
	}

	.bottom-left {
		@apply bottom-8 left-10;
	}

	.bottom-right {
		@apply bottom-8 right-10;
	}
</style>
