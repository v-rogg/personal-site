import type { LayoutServerLoad } from "./$types";
import { sendServerEvent } from "$lib/posthog";
export const load: LayoutServerLoad = ({url, cookies}) => {
	sendServerEvent('$pageview', url, cookies)
	return;
}