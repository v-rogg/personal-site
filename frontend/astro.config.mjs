import path from "path";
import { fileURLToPath } from "url";
import mdx from "@astrojs/mdx";
import svelte from "@astrojs/svelte";
import deno from "@deno/astro-adapter";
import tailwindcss from "@tailwindcss/vite";
// @ts-check
import { defineConfig } from "astro/config";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// https://astro.build/config
export default defineConfig({
	output: "server",
	adapter: deno(),
	integrations: [svelte(), mdx()],
	vite: {
		plugins: [tailwindcss()],
		resolve: {
			alias: {
				$lib: path.resolve(__dirname, "./src/lib")
			}
		},
		server: {
			proxy: {
				// In dev mode, proxy /files/* to Caddy running in Docker
				"/files": {
					target: "http://localhost:8080",
					changeOrigin: true
				}
			}
		}
	}
});
