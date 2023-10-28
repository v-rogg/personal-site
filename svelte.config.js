import adapter from "@sveltejs/adapter-node";
// import adapter from "svelte-adapter-bun";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import("@sveltejs/kit").Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		alias: {
			"@storyblok/svelte": "./node_modules/@storyblok/svelte",
			"$drizzle": "./src/drizzle"
		}
	}
};

export default config;
