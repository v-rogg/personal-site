import type { LayoutLoad } from "./$types";
import { loadTranslations, translations } from "$lib/_i18n";
import { slugStore } from "$lib/stores";
import { storyblokInit, apiPlugin, useStoryblokApi } from "@storyblok/svelte";
import { PUBLIC_STORYBLOK_TOKEN } from "$env/static/public";
import Memory from "$lib/components/sections/Memories/Memory.svelte";
import ContactMe from "$lib/components/sections/ContactMe/ContactMe.svelte";
import Project from "$lib/components/sections/Project/Project.svelte";
import LinkButton from "$lib/components/globals/LinkButton/LinkButton.svelte";

export const prerender = true;

export const load: LayoutLoad = async ({ url, fetch }) => {
	const loadUrl = new URL(url);
	const { pathname } = loadUrl;

	const route = pathname;

	await loadTranslations("en", pathname);

	let slug = "slugs.default";

	if (route != "") {
		const trans = translations.get()["en"];
		for (const [key, value] of Object.entries(trans)) {
			if (value.includes(route)) {
				slug = key;
			}
		}
	}

	slugStore.set(slug);

	storyblokInit({
		accessToken: PUBLIC_STORYBLOK_TOKEN,
		use: [apiPlugin],
		apiOptions: {
			cache: { type: "memory" },
			https: true,
			fetch
		},
		components: {
			Memory: Memory,
			ContactMe: ContactMe,
			Project: Project,
			LinkButton: LinkButton
		}
	});

	const storyblokApi = await useStoryblokApi();

	return { route, lang: "en", storyblokApi };
};
