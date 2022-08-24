import i18n from "sveltekit-i18n";
import lang from "./lang.json";

const config = {
  translations: {
    en: { lang },
    de: { lang }
  },
  loaders: [
    {
      locale: "en",
      key: "common",
      route: [""],
      loader: async () => (await import("./en/common.json")).default
    },
    {
      locale: "en",
      key: "bio",
      route: [""],
      loader: async () => (await import("./en/bio.json")).default
    },
    {
      locale: "en",
      key: "signature",
      route: [""],
      loader: async () => (await import("./en/signature.json")).default
    },
    {
      locale: "de",
      key: "common",
      route: [""],
      loader: async () => (await import("./de/common.json")).default
    },
    {
      locale: "de",
      key: "bio",
      route: [""],
      loader: async () => (await import("./de/bio.json")).default
    },
    {
      locale: "de",
      key: "signature",
      route: [""],
      loader: async () => (await import("./de/signature.json")).default
    },
  ]
};

export const defaultLocale = "en";

export const { t, locale, locales, loading, loadTranslations, translations } = new i18n(config);

loading.subscribe(async ($loading) => {
  if ($loading) {
    console.log("Loading translations...");

    await loading.toPromise();
    console.log("Updated translations", translations.get());
  }
});
