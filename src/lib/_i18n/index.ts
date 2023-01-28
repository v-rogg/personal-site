import i18n from "sveltekit-i18n";
import lang from "./lang.json";
import { en_loaders } from "./en/loaders";
import { de_loaders } from "./de/loaders";

export const config = {
	translations: {
		en: { lang },
		de: { lang }
	},
	loaders: [...en_loaders, ...de_loaders]
};

export const defaultLocale = "en";

export const { l, t, locale, locales, loading, loadTranslations, translations } = new i18n(config);

loading.subscribe(async ($loading) => {
	if ($loading) {
		console.log("Loading translations...");

		await loading.toPromise();
		console.log("Updated translations", translations.get());
	}
});
