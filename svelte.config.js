import { preprocessMeltUI, sequence } from "@melt-ui/pp";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import adapter from "@sveltejs/adapter-cloudflare";
import { mdsvex } from "mdsvex";
import { createHighlighter } from "shiki";
import tokio from "shiki/themes/tokyo-night.mjs";

/** @type {import('@sveltejs/kit').Config}*/
const config = {
	extensions: [".svelte", ".md"],
	preprocess: sequence([
		vitePreprocess(),
		preprocessMeltUI(),
		mdsvex({
			extension: ".md",
			smartypants: {
				dashes: "oldschool"
			},
			layout: "./src/lib/mdx/BlogLayout.svelte",
			highlight: {
				highlighter: async (code, lang) => {
					const highlighter = await createHighlighter({ theme: tokio, langs: [lang] });
					const html = highlighter.codeToHtml(code, { lang: lang, theme: tokio });
					return "{@html `" + html + "`}";
				}
			}
		})
	]),
	kit: {
		adapter: adapter()
	}
};
export default config;
