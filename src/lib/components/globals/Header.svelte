<script lang="ts">
	import { afterNavigate } from "$app/navigation";
	import { blur as fly } from "svelte/transition";
	import { page } from "$app/stores";
	import type { NavigationTarget } from "@sveltejs/kit";
	import { expoIn, expoOut } from "svelte/easing";

	let slugs = $derived.by(() => {
		if ($page.url) {
			const path = $page.url.pathname;
			const segments = path.split("/").filter(Boolean);

			const possibleRoutes = segments.map((_, index) => {
				return "/" + segments.slice(0, index + 1).join("/");
			});

			return possibleRoutes;
		} else return [];
	});

	let navigationHistory: NavigationTarget[] = $state([]);

	afterNavigate((e) => {
		if (e.from) navigationHistory.push(e.from);
	});
</script>

<header class="absolute z-50 w-full -translate-y-0.5 transition sm:top-8">
	<div class="container mx-auto mb-10 mt-8 flex items-center justify-between px-10">
		<div class="flex h-10 items-center gap-4">
			{#if $page.url.pathname == "/"}
				<span class="-mx-2 size-14 rounded p-2">
					<img src="/favicon.svg" alt="vr" width="56" height="56" />
				</span>
			{:else}
				<a class="-mx-2 size-14 rounded p-2 hover:bg-white-700 active:bg-white-600" href="/" aria-label="Home">
					<img src="/favicon.svg" alt="vr" width="56" height="56" />
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
								<a href={page} class="mx-1 rounded px-1 py-0.5 hover:bg-white-700 active:bg-white-600">
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
		{#if navigationHistory.length > 0}
			<a href={navigationHistory.at(-1)?.url.href} in:fly={{ duration: 600, amount: 10 }}>
				<i class="fa-solid fa-arrow-left mr-2 rounded px-2 py-1 hover:bg-white-700 active:bg-white-600"></i>
				Zurück
			</a>
		{/if}
	</div>
</header>
