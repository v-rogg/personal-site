<script lang="ts">
	import { t, locale } from "$lib/_i18n";
</script>

<section>
	<h1>Valentin Rogg</h1>
	<p>
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html $t("bio.description").replace(
			"project_placeholder",
			`<a href='/${$locale}' class="bold">Parley</a>`
		)}
	</p>
</section>

<style>
	section {
		text-align: center;
	}

	h1 {
		font-family: var(--ff-display);
		font-size: 32px;
		font-weight: 600;
		margin: 0 0 2rem;
	}
</style>
