<script lang="ts">
	import { enhance } from "$app/forms";
	import { scale } from "svelte/transition";
	import { storyblokEditable } from "@storyblok/svelte";
	import { t } from "$lib/_i18n";
	export let blok;

	let mailSent = false;
	let email: string;
	let message: string;
</script>

<section class="grid-settings bg-black dark:bg-blue-900 sm:rounded-2xl p-8 text-white-500 flex flex-col h-[25rem] min-w-[25rem]"  style="--sm: {blok.grid_sm}; --md: {blok.grid_md}; --lg: {blok.grid_lg}; --xl: {blok.grid_xl};" use:storyblokEditable={blok}>
	<h3 class="text-3xl font-display text-center">{blok.title}</h3>
	<form out:scale={{duration: 300}} method="POST" action="/actions/mail?/sendMail" class="mt-8 flex flex-col items-center gap-3 flex-grow" use:enhance={({formData}) => {
		email = formData.get("email")?.toString() || "";
		message = formData.get("message")?.toString() || "";
		return async ({result}) => {
			if (result.status === 200) {
				mailSent = true;
			}
		}
	}}>
		<input name="email" type="email" required disabled="{mailSent}" bind:value={email} class="min-w-full bg-grey-800 dark:bg-blue-800 py-2 px-4 rounded-lg text-white dark:text-white-500 placeholder-grey-600" placeholder="{$t('contactme.yourEmail')}">
		<textarea name="message" required disabled="{mailSent}" bind:value={message} class="min-w-full h-4 bg-grey-800 flex-grow dark:bg-blue-800 py-2 px-4 rounded-lg text-white dark:text-white-500 placeholder-grey-600 resize-none" placeholder="{$t('contactme.yourMessage')}" />
		<div class="flex gap-3">
<!--			<a class="dark:bg-white-700 dark:text-blue-900 text-white-500 py-2 px-4 rounded-lg border border-white-500" href="{learnMoreURL.toString()}">Learn more</a>-->
			{#if !mailSent}
				<button type="submit" class="bg-white-500 dark:bg-blue-800 text-black dark:text-white-500 px-2 py-1 rounded-lg border border-white-500 dark:border-blue-700">{$t("contactme.leaveMessage")}</button>
			{:else}
				<p class="self-center px-2 py-1 border border-transparent">{$t("contactme.messageSent")} <i class="fa-solid fa-paper-plane"></i></p>
			{/if}
		</div>
	</form>
</section>

<style lang="postcss">
  .grid-settings {
	@media (min-width: theme("screens.sm")) {
	  grid-column: span var(--sm) / span var(--sm);
	}
	@media (min-width: theme("screens.md")) {
	  grid-column: span var(--md) / span var(--md);
	}
	@media (min-width: theme("screens.lg")) {
	  grid-column: span var(--lg) / span var(--lg);
	}
	@media (min-width: theme("screens.xl")) {
	  grid-column: span var(--xl) / span var(--xl);
	}
  }
</style>
