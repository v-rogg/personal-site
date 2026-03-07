<script module>
import { createHighlighterCoreSync } from "shiki/core";
import { createJavaScriptRegexEngine } from "shiki/engine/javascript";

import themeTokioNight from "shiki/themes/tokyo-night.mjs";

import console from "shiki/langs/console.mjs";
import css from "shiki/langs/css.mjs";
import html from "shiki/langs/html.mjs";
import js from "shiki/langs/javascript.mjs";
import svelte from "shiki/langs/svelte.mjs";

const shiki = createHighlighterCoreSync({
	engine: createJavaScriptRegexEngine(),
	themes: [themeTokioNight],
	langs: [console, html, css, js, svelte]
});
</script>

<script lang="ts">
	let {
		code = "",
		lang = "console",
		theme = "tokyo-night",
		base = " overflow-hidden",
		rounded = "rounded-container",
		shadow = "",
		classes = "",
		preBase = "",
		prePadding = "[&>pre]:p-4",
		preClasses = ""
	}: {
		code?: string;
		lang?: "console" | "html" | "css" | "js" | "svelte";
		theme?: "tokyo-night";
		base?: string;
		rounded?: string;
		shadow?: string;
		classes?: string;
		preBase?: string;
		prePadding?: string;
		preClasses?: string;
	} = $props();

	const generatedHtml = shiki.codeToHtml(code, { lang, theme });
</script>

<div
	class="{base} {rounded} {shadow} {classes} {preBase} {prePadding} {preClasses} w-max sm:min-w-80"
	style="font-family: serif !important;"
>
	{@html generatedHtml}
</div>
