<script>
	export let color;
	export let seperateDarkmode = true;
</script>

<!--<div class="mt-4 inline-flex items-center rounded-md backdrop-blur-sm px-2 py-1 text-xs font-medium ring-1 ring-inset ring-current max-w-max" style="color: {color}">-->
<!--	<slot/>-->
<!--</div>-->

<div
	class="relative z-10 pointer-events-none backdrop-blur-md px-2 py-1 rounded-lg ring-1 ring-inset ring-current flex items-center overflow-hidden" style="color: {color}" class:dark:ring-blue-650={seperateDarkmode}>
	<div class="absolute top-0 left-0 bg-current opacity-10" style="width: 100%; height: 100%" class:dark:bg-blue-650={seperateDarkmode}>
	</div>
	<div class="text-xs" class:dark:text-blue-600={seperateDarkmode}>
		<slot/>
	</div>
</div>