<script lang="ts">
	import { enhance } from "$app/forms";
	import { fade, scale } from "svelte/transition";
	import { env } from "$env/dynamic/public";
	import posthog from "posthog-js";

	let { cls }: { cls?: string } = $props();

	let mailSent = $state(false);
	let mailFormSent = $state(false);
	let email: string | undefined = $state();
	let message: string | undefined = $state();
</script>

<section class="flex min-h-128 flex-col bg-black p-8 text-white-500 sm:rounded-2xl {cls}">
	<h3 class=" text-balance text-center text-3xl font-medium">
		Sie benötigen eine speziell angefertigte Website, Plattform oder Service?
	</h3>
	<form
		out:scale={{ duration: 300 }}
		method="POST"
		action="/api/actions/request?/sendMail"
		class="mt-8 flex flex-grow flex-col items-center gap-3"
		use:enhance={({ formData }) => {
			email = formData.get("email")?.toString() || "";
			message = formData.get("message")?.toString() || "";
			mailFormSent = true;
			return async ({ result }) => {
				if (result.status === 200) {
					posthog.identify(email, { email: email });
					mailSent = true;
					mailFormSent = false;
				}
			};
		}}
	>
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
		<textarea
			name="message"
			required
			disabled={mailSent}
			bind:value={message}
			class="max-h-fit min-h-24 w-full flex-grow resize-none rounded-lg bg-grey-800 px-4 py-2 text-white placeholder-grey-600 transition"
			class:bg-grey-900={mailSent}
			placeholder="Deine Nachricht"
		></textarea>
		<div class="cf-turnstile" data-sitekey={env.PUBLIC_CF_TURNSTILE_SITE_KEY}></div>
		<div class="flex gap-3 text-md-lg">
			{#if !mailSent}
				<button
					class="rounded-lg border border-white-500 bg-white-500 px-3 py-1 text-black hover:bg-white-700 active:bg-white-600"
					>Nachricht senden
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
