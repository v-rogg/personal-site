<script lang="ts">
import { submitContact } from "$lib/api";
import { getSessionId } from "$lib/session";
import { trackEvent } from "$lib/tracking";
import { onMount } from "svelte";
import { fade, scale } from "svelte/transition";

let { cls }: { cls?: string } = $props();

let mailSent = $state(false);
let mailFormSent = $state(false);
let email: string | undefined = $state();
let message: string | undefined = $state();
let captchaSolution = $state("");
let captchaWidget: any = $state(null);
let errorMessage = $state("");
let isLocalhost = $state(false);

onMount(async () => {
	isLocalhost = window.location.hostname.includes("localhost");
	const sitekey = import.meta.env.PUBLIC_FRIENDLY_CAPTCHA_SITEKEY;

	if (!isLocalhost && sitekey) {
		const { WidgetInstance } = await import("friendly-challenge");
		const container = document.getElementById("captcha-widget");
		if (container) {
			captchaWidget = new WidgetInstance(container, {
				sitekey,
				doneCallback: (solution: string) => {
					captchaSolution = solution;
				},
				errorCallback: (err: Error) => {
					console.error("Captcha error:", err);
					errorMessage = "Captcha-Fehler. Bitte Seite neu laden.";
				}
			});
		}
	}

	return () => {
		captchaWidget?.destroy();
	};
});

async function handleSubmit(e: Event) {
	e.preventDefault();
	if (!email || !message) return;

	if (!isLocalhost && !captchaSolution) {
		errorMessage = "Bitte Captcha lösen.";
		return;
	}

	mailFormSent = true;
	errorMessage = "";

	try {
		await submitContact({
			session_id: getSessionId(),
			email,
			message,
			captcha_solution: captchaSolution || "localhost-bypass"
		});

		await trackEvent("contact.submit");
		mailSent = true;
	} catch (err) {
		console.error("Contact form error:", err);
		errorMessage = "Fehler beim Senden. Bitte erneut versuchen.";
		captchaWidget?.reset();
		captchaSolution = "";
	} finally {
		mailFormSent = false;
	}
}
</script>

<section class="flex min-h-128 flex-col bg-black p-8 text-white-500 sm:rounded-2xl {cls}">
	<h3 class="text-balance text-center text-3xl font-medium">
		Du hast eine Idee für ein spannendes neue Website, Projekt oder Service?
	</h3>
	<form
		out:scale={{ duration: 300 }}
		class="mt-8 flex grow flex-col items-center gap-3"
		onsubmit={handleSubmit}
	>
		<div class="w-full relative">
			<p class="text-right text-sm text-grey-600 opacity-30 mb-1 absolute right-2 top-1/2 translate-y-[-50%]"><sup>*</sup>optional für Rückmeldung</p>

			<input
				name="email"
				type="email"
				required
				disabled={mailSent}
				bind:value={email}
				class="w-full rounded-lg bg-grey-800 px-4 py-2 text-white placeholder-grey-600 transition"
				class:bg-grey-900={mailSent}
				placeholder="Deine E-Mail"
			/>
		</div>
		<textarea
			name="message"
			required
			disabled={mailSent}
			bind:value={message}
			class="max-h-fit min-h-36 w-full grow resize-none rounded-lg bg-grey-800 px-4 py-2 text-white placeholder-grey-600 transition"
			class:bg-grey-900={mailSent}
			placeholder="Deine Nachricht"
		></textarea>
		<div id="captcha-widget" class="frc-captcha"></div>
		{#if errorMessage}
			<p class="text-red-400 text-sm">{errorMessage}</p>
		{/if}
		<div class="flex gap-3 text-md-lg">
			{#if !mailSent}
				<button
					type="submit"
					class="rounded-lg border border-white-500 bg-white-500 px-3 py-1 text-black hover:bg-white-700 active:bg-white-600"
					>Melde dich bei mir
					{#if mailFormSent}
						<i class="fa-solid fa-spinner-third fa-spin ml-2"></i>
					{:else}
						<i class="fa-solid fa-envelope ml-2"></i>
					{/if}
				</button>
			{:else}
				<p class="self-center border border-transparent px-3 py-1" in:fade>
					Nachricht gesendet <i class="fa-solid fa-paper-plane ml-2"></i>
				</p>
			{/if}
		</div>
	</form>
</section>
