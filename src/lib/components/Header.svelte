<script lang="ts">
	import { locale } from "$lib/_i18n";
	import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
	import LanguageSelect from "$lib/components/LanguageSelect.svelte";

	export let noLang = false;
</script>

<header class="container">
	<h1 class="overlay">
		<a href="/{$locale}"> Valentin Rogg </a>
	</h1>

	<!--	<nav class="overlay">-->
	<!--		<ul>-->

	<!--			      <li class:active={$page.url.pathname === "/posts"}>-->
	<!--			        <a href="/{$locale}" aria-disabled="true" class="strikethrough disabled"-->
	<!--			          >{$t("common.posts")}</a-->
	<!--			        >-->
	<!--			        <span class="coming_soon">{$t("common.coming_soon")}</span>-->
	<!--			      </li>-->
	<!--			      <li class:active={$page.url.pathname === "/projects"}>-->
	<!--			        <a href="/{$locale}" aria-disabled="true" class="strikethrough disabled"-->
	<!--			          >{$t("common.projects")}</a-->
	<!--			        >-->
	<!--			        <span class="coming_soon">{$t("common.coming_soon")}</span>-->
	<!--			      </li>-->
	<!--		</ul>-->
	<!--	</nav>-->

	<div class="options overlay">
		<DarkModeToggle />
		{#if !noLang}
			<LanguageSelect />
		{/if}
	</div>
</header>

<style>
	header {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		justify-content: space-between;
		align-items: center;
		font-size: 16px;
		/*padding: 10px 2rem 60px;*/
		/*width: 100vw;*/
		position: fixed;
		/*padding: 2rem;*/
		/*margin: 0 1rem;*/
		left: 50%;
		transform: translate(-50%);
		top: 4rem;
		z-index: 900;
	}

	@media (max-width: 540px) {
		h1 {
			display: none;
		}

		header {
			top: 2rem;
			display: flex;
			flex-direction: column;
			gap: 1rem;
		}
	}

	/*.hamburger {*/
	/*	display: none;*/
	/*}*/

	/*@media (max-width: 399px) {*/
	/*	.options {*/
	/*		display: none !important;*/
	/*	}*/

	/*	.hamburger {*/
	/*		display: block;*/
	/*	}*/
	/*}*/

	/*@media (max-width: 575.98px) {*/
	/*	header {*/
	/*		grid-template-columns: 1fr 1fr;*/
	/*		grid-template-rows: 1fr auto;*/
	/*	}*/

	/*	nav {*/
	/*		grid-area: 2/1/3/4;*/
	/*	}*/

	/*	nav ul {*/
	/*		flex-direction: column;*/
	/*		align-items: center;*/
	/*		font-size: 2.5rem;*/
	/*		margin-top: 60px !important;*/
	/*		margin-bottom: 1em !important;*/
	/*		gap: 2em !important;*/
	/*	}*/
	/*}*/

	.options {
		justify-self: flex-end;
	}

	/*nav {*/
	/*	justify-self: center;*/
	/*}*/

	/*nav ul {*/
	/*	margin: 0;*/
	/*	padding: 0;*/
	/*	display: flex;*/
	/*	gap: 4em;*/
	/*}*/

	/*nav ul li {*/
	/*	list-style: none;*/
	/*	display: flex;*/
	/*	position: relative;*/
	/*}*/

	/*.coming_soon {*/
	/*	font-size: 0.625em;*/
	/*	position: absolute;*/
	/*	top: 100%;*/
	/*	left: 50%;*/
	/*	transform: translateX(-50%);*/
	/*	width: max-content;*/
	/*	text-align: center;*/
	/*	color: var(--c-primary);*/
	/*}*/

	/*.new {*/
	/*  font-size: 8px;*/
	/*  position: absolute;*/
	/*  bottom: 70%;*/
	/*  right: 0;*/
	/*  transform: translateX(8px);*/
	/*  color: var(--c-green);*/
	/*}*/

	.options {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	h1 {
		font-family: var(--ff-display);
		font-size: 18px;
		font-weight: 700;
		white-space: nowrap;
		padding: 0;
		margin: 0;
	}
</style>
