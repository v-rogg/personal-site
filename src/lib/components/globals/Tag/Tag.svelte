<script>
	export let color;
</script>

<!--<div class="mt-4 inline-flex items-center rounded-md backdrop-blur-sm px-2 py-1 text-xs font-medium ring-1 ring-inset ring-current max-w-max" style="color: {color}">-->
<!--	<slot/>-->
<!--</div>-->

<div
	class="relative z-10 pointer-events-none backdrop-blur-md px-2 py-1 rounded-lg ring-1 ring-inset ring-current flex items-center overflow-hidden dark:ring-white-500" style="color: {color}">
	<div class="absolute top-0 left-0 bg-current dark:bg-white-500 opacity-10" style="width: 100%; height: 100%">
	</div>
	<div class="text-xs dark:text-white-500">
		<slot/>
	</div>
</div>