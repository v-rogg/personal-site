<script lang="ts">
	import { afterNavigate } from "$app/navigation";
	import { blur as fly } from "svelte/transition";
	import { page } from "$app/stores";
	import type { NavigationTarget } from "@sveltejs/kit";
	import { expoIn, expoOut } from "svelte/easing";
	import { appState } from "$lib/stores.svelte";

	let slugs = $derived.by(() => {
		if ($page.url) {
			const path = $page.url.pathname;
			const segments = path.split("/").filter(Boolean);

			const possibleRoutes = segments.map((_, index) => {
				return "/" + segments.slice(0, index + 1).join("/");
			});

			const currentSlug = appState.metadata?.slug;
			if (currentSlug) {
				possibleRoutes.pop();
				possibleRoutes.push(currentSlug);
			}

			return possibleRoutes;
		} else return [];
	});

	let navigationHistory: NavigationTarget[] = $state([]);

	afterNavigate((e) => {
		if (e.from) navigationHistory.push(e.from);
	});
</script>

<header
	class="pointer-events-none absolute z-50 w-full -translate-y-0.5 transition sm:top-8"
	class:floating-header={$page.url.pathname != "/"}
>
	<div class="container mx-auto mb-10 mt-8 flex items-center justify-between px-10">
		<div class="flex h-10 items-center gap-4">
			{#if $page.url.pathname == "/"}
				<span class="flex size-14 items-center justify-center rounded">
					<img
						src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/ae0fea1a-febe-4b43-b8ed-58718c0a2100/public"
						alt="VR"
						width="40"
						height="40"
					/>
				</span>
			{:else}
				<a
					class="pointer-events-auto flex size-14 items-center justify-center rounded hover:bg-white-700 active:bg-white-600"
					href="/"
					aria-label="Home"
				>
					<img
						src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/ae0fea1a-febe-4b43-b8ed-58718c0a2100/public"
						alt="VR"
						width="40"
						height="40"
					/>
				</a>
			{/if}
			{#if $page.status === 200}
				<div class="flex items-center gap-1">
					{#each slugs as page, i}
						{@const name = page.split("/").at(-1) || ""}
						{@const delay = 100}
						{@const duration = 500}
						{@const amount = 10}
						<div
							id={i + ""}
							in:fly={{ delay: i * delay, duration, amount, easing: expoOut }}
							out:fly={{ delay: (slugs.length + 1 - i) * delay, duration, amount, easing: expoIn }}
						>
							<i class="fa-solid fa-slash-forward"></i>
							{#if slugs.length - 1 === i}
								<span class="mx-1 rounded px-1 py-0.5">
									{name.charAt(0).toUpperCase() + name.slice(1)}
								</span>
							{:else}
								<a href={page} class="pointer-events-auto rounded px-2 py-1 hover:bg-white-700 active:bg-white-600">
									{name.charAt(0).toUpperCase() + name.slice(1)}
								</a>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<div>
					Error {$page.status}: {$page.error?.message}
				</div>
			{/if}
		</div>
		{#if navigationHistory.length > 0 && $page.url.pathname != "/"}
			<a
				href={navigationHistory.at(-1)?.url.href}
				in:fly={{ duration: 600, amount: 10 }}
				class="pointer-events-auto rounded px-2 py-1 hover:bg-white-700 active:bg-white-600"
			>
				<i class="fa-solid fa-arrow-left mr-2"></i>
				Zurück
			</a>
		{/if}
	</div>
</header>

<style lang="postcss">
	.floating-header {
		position: fixed;
		@apply bg-white/40;
		@apply backdrop-blur;
		@apply top-0;
		@apply pt-8;

		> div {
			@apply mt-0;
			@apply mb-0;
			@apply py-8;
		}
	}
</style>
