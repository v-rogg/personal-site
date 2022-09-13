import type { LayoutLoad } from "./$types";
import { loadTranslations, translations, locales } from "$lib/_i18n";
import { slugStore } from "$lib/../stores";

export const load: LayoutLoad = async ({ url }) => {
  const loadUrl = new URL(url);
  const { pathname } = loadUrl;

  let lang = `${pathname.match(/[^/]+?(?=\/|$)/) || ""}`;

  if (!pathname.includes("api") && !pathname.includes(".css")) {
    const route = pathname.replace(new RegExp(`^/${lang}`), "");

    // console.log(pathname);

    for (const locale of locales.get()) {
      await loadTranslations(locale, "/");
    }

    if (lang == "admin") {
      lang = "en"
    }

    await loadTranslations(lang, pathname);

    let slug = "slugs.default";

    if (route != '') {
      const trans = translations.get()[lang];
      for (const [key, value] of Object.entries(trans)) {
        if (value.includes(route)) {
          slug = key;
        }
      }
    }

    slugStore.set(slug);

    return { route, lang };
  }

  return {};
};
