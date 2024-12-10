<script lang="ts">
	import { enhance } from "$app/forms";
	import { fade, scale } from "svelte/transition";
	import { PUBLIC_CF_TURNSTILE_SITE_KEY } from "$env/static/public";

	let mailSent = false;
	let email: string;
	let message: string;
</script>

<section class="flex min-h-80 min-w-[25rem] flex-col bg-black p-8 text-white-500 sm:rounded-2xl">
	<h3 class="text-center text-3xl font-medium">
		Du benötigst eine speziell angefertigte Website, Plattform oder Service?
	</h3>
	<form
		out:scale={{ duration: 300 }}
		method="POST"
		action="/actions/request?/sendMail"
		class="mt-8 flex flex-grow flex-col items-center gap-3"
		use:enhance={({ formData }) => {
			email = formData.get("email")?.toString() || "";
			message = formData.get("message")?.toString() || "";
			return async ({ result }) => {
				if (result.status === 200) {
					mailSent = true;
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
			class="text-white min-w-full rounded-lg bg-grey-800 px-4 py-2 placeholder-grey-600 transition"
			class:bg-grey-900={mailSent}
			placeholder="Deine E-Mail"
		/>
		<textarea
			name="message"
			required
			disabled={mailSent}
			bind:value={message}
			class="text-white max-h-fit min-w-full flex-grow resize-none rounded-lg bg-grey-800 px-4 py-2 placeholder-grey-600 transition"
			class:bg-grey-900={mailSent}
			placeholder="Deine Nachricht"
		></textarea>
		<div class="cf-turnstile" data-sitekey={PUBLIC_CF_TURNSTILE_SITE_KEY}></div>
		<div class="flex gap-3">
			{#if !mailSent}
				<button
					class="rounded-lg border border-white-500 bg-white-500 px-2 py-1 text-black hover:bg-white-700 active:bg-white-600"
					>Nachricht senden</button
				>
			{:else}
				<p class="self-center border border-transparent px-2 py-1" in:fade>
					Nachricht gesendet <i class="fa-solid fa-paper-plane ml-2"></i>
				</p>
			{/if}
		</div>
	</form>
</section>
