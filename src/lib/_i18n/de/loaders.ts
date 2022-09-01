export const de_loaders =  [
  {
    locale: "de",
    key: "common",
    loader: async () => (await import("./common.json")).default
  },
  {
    locale: "de",
    key: "slugs",
    loader: async () => (await import("./slugs.json")).default
  },
  {
    locale: "de",
    key: "bio",
    routes: ["/de"],
    loader: async () => (await import("./bio.json")).default
  },
  {
    locale: "de",
    key: "signature",
    routes: ["/de"],
    loader: async () => (await import("./signature.json")).default
  },
  {
    locale: "de",
    key: "legal",
    routes: ["/de/rechtliches"],
    loader: async () => (await import("./legal.json")).default
  },
]
