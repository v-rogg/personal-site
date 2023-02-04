import type { LayoutLoad } from "../../../.svelte-kit/types/src/routes/$types";
import { loadTranslations, translations, locales } from "$lib/_i18n";
import { admin, slugStore } from "$lib/stores";
import { config } from "$lib/_i18n";

export const load: LayoutLoad = async ({ url }) => {
	admin.set(false);
	const loadUrl = new URL(url);
	const { pathname } = loadUrl;

	let lang: string = (pathname.match(/[^/]+?(?=\/|$)/) || [])[0];

	if (Object.keys(config.translations).includes(lang) || pathname.includes("admin")) {
		const route = pathname.replace(new RegExp(`^/${lang}`), "");

		for (const locale of locales.get()) {
			await loadTranslations(locale, "/");
		}

		if (pathname.includes("admin")) {
			lang = "en";
			admin.set(true);
		}

		await loadTranslations(lang, pathname);

		let slug = "slugs.default";

		if (route != "") {
			const trans = translations.get()[lang];
			for (const [key, value] of Object.entries(trans)) {
				if (value === route && key.includes("slugs")) {
					slug = key;
				}
			}
		}

		slugStore.set(slug);

		return { route, lang };
	}

	return {};
};
