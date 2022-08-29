export const en_loaders = [
  {
    locale: "en",
    key: "common",
    loader: async () => (await import("./common.json")).default
  },
  {
    locale: "en",
    key: "slugs",
    loader: async () => (await import("./slugs.json")).default
  },
  {
    locale: "en",
    key: "bio",
    routes: ["/en"],
    loader: async () => (await import("./bio.json")).default
  },
  {
    locale: "en",
    key: "signature",
    routes: ["/en"],
    loader: async () => (await import("./signature.json")).default
  },
  {
    locale: "en",
    key: "legal",
    routes: ["/en/legal"],
    loader: async () => (await import("./legal.json")).default
  },
]
