<script lang="ts">
	let { src, alt, maxH = "500" } = $props();
</script>

<div class="flex justify-center">
	<div class="relative">
		<img src={src} class="max-h-[{maxH}px]" alt={alt} />
		<span class="absolute bottom-2 left-0 rounded-md bg-white/70 p-1 text-xs backdrop-blur">{alt}</span>
	</div>
</div>
