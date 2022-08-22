import type { LayoutLoad } from "./$types";
import { loadTranslations } from "$lib/_i18n";

export const load: LayoutLoad = async ({ url }) => {
  const loadUrl = new URL(url);
  const { pathname } = loadUrl;

  const lang = `${pathname.match(/[^/]+?(?=\/|$)/) || ""}`;

  const route = pathname.replace(new RegExp(`^/${lang}`), "");

  await loadTranslations(lang, route);

  return { route, lang };
};
