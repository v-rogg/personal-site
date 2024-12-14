<script lang="ts">
	import { goto } from "$app/navigation";
</script>

<button
	class="col-span-4 block rounded-xl bg-green p-10"
	role="link"
	onclick={() => {
		goto("projekte/wassererbe");
	}}
>
	<h3 class="text-2xl text-white-500">Wassererbe Augsburg</h3>
</button>
