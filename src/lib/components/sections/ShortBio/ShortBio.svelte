<script lang="ts">
	import { t, locale } from "$lib/_i18n";
</script>

<section>
	<h1>Valentin Rogg</h1>

	<p>
		{@html $t("bio.description").replace(
			"project_placeholder",
			`<a href='/${$locale}' class="bold">Parley</a>`
		)}
	</p>
	<!--	<a-->
	<!--		id="contact"-->
	<!--		href="https://www.linkedin.com/in/vrogg/"-->
	<!--		rel="noreferrer nofollow"-->
	<!--		target="_blank">{$t("bio.contact")}<i class="icon fa-solid fa-comment-smile"></i></a>-->
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

	#contact {
		display: block;
		width: max-content;
		margin: 2.5rem auto;
		font-size: 16px;
		border-radius: 4px;
		padding: 0.5rem 1rem;
		border: none;
		background: var(--c-secondary);
	}

	#contact .icon {
		margin-left: 0.5rem;
		color: var(--c-primary);
	}

	#contact:hover {
		cursor: pointer;
		background: var(--c-grey-10);
	}

	#contact:active {
		cursor: default;
		background: var(--c-grey-30);
	}
</style>
