import type { LayoutServerLoad } from "./$types";
import { sendServerEvent } from "$lib/posthog";
export const load: LayoutServerLoad = ({ url, cookies }) => {
	if (
		!(url.origin.includes("sveltekit") && url.origin.includes("storyblok") && url.origin.includes("cloudflare") && url.origin.includes("sitemap"))
	)
		sendServerEvent("$pageview", url, cookies);
	return;
};
