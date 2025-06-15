<script lang="ts">
	import { version } from "../../../../package.json";
	import Socials from "./Socials.svelte";
</script>

<footer class="container relative mx-auto mb-16 mt-16 flex flex-col-reverse items-center justify-between gap-8 px-10 sm:flex-row sm:items-end">
	<div class="mb-0.5 flex w-fit flex-col gap-1 text-center text-sm sm:text-left">
		<div>
			<i class="fa-regular fa-copyright"></i>
			Valentin Rogg
			<i class="fa-regular fa-code-commit ml-2"></i>
			{version}
		</div>
	</div>

	<Socials cls="text-md-lg" />
</footer>
