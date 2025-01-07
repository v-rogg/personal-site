<script module>
	import { createHighlighterCoreSync } from "shiki/core";
	import { createJavaScriptRegexEngine } from "shiki/engine/javascript";

	import themeTokioNight from "shiki/themes/tokyo-night.mjs";

	import console from "shiki/langs/console.mjs";
	import html from "shiki/langs/html.mjs";
	import svelte from "shiki/langs/svelte.mjs";
	import css from "shiki/langs/css.mjs";
	import js from "shiki/langs/javascript.mjs";

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
		// Base Style Props
		base?: string;
		rounded?: string;
		shadow?: string;
		classes?: string;
		// Pre Style Props
		preBase?: string;
		prePadding?: string;
		preClasses?: string;
	} = $props();

	const generatedHtml = shiki.codeToHtml(code, { lang, theme });
</script>

<div
	class="{base} {rounded} {shadow} {classes} {preBase} {prePadding} {preClasses}"
	style="font-family: serif !important;"
>
	<!-- eslint-disable-next-line -->
	{@html generatedHtml}
</div>
