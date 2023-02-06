import type { LayoutLoad } from "./$types";
import { loadTranslations, translations } from "$lib/_i18n";
import { slugStore } from "$lib/stores";

export const prerender = true;

export const load: LayoutLoad = async ({ url }) => {
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

	return { route, lang: "en" };
};
