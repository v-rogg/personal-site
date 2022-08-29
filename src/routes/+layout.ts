import type { LayoutLoad } from "./$types";
import { loadTranslations } from "$lib/_i18n";

export const load: LayoutLoad = async ({ url }) => {
  const loadUrl = new URL(url);
  const { pathname } = loadUrl;

  const lang = `${pathname.match(/[^/]+?(?=\/|$)/) || ""}`;

  if (!pathname.includes("api") && !pathname.includes(".css")) {
    const route = pathname.replace(new RegExp(`^/${lang}`), "");
    await loadTranslations(lang, pathname);
    return { route, lang };
  }

  return {};
};
