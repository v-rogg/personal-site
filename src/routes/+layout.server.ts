import type { LayoutServerLoad } from "./$types";
import { sendServerEvent } from "$lib/posthog";
export const load: LayoutServerLoad = ({url, cookies}) => {
	if (!url.origin.includes("sveltekit")) sendServerEvent('$pageview', url, cookies);
	return;
}