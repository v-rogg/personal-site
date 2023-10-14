import { PostHog } from "posthog-node";
import { PUBLIC_POSTHOG_TOKEN } from "$env/static/public";
import * as cookie from "cookie";
import type { Cookies } from "@sveltejs/kit";

export const postHogClient = new PostHog(PUBLIC_POSTHOG_TOKEN, { host: "https://app.posthog.com" });

export function sendClientEvent(event: string, document: Document, properties?: object) {
	postHogClient.capture({
		distinctId: cookie.parse(document.cookie)["identifier"] || "development",
		event,
		properties: { $current_url: document.URL, ...properties }
	});
	postHogClient.flush();
}

export function sendServerEvent(event: string, url: URL, cookies: Cookies, properties?: object) {
	postHogClient.capture({
		distinctId: cookies.get("identifier") || "development",
		event,
		properties: {
			$current_url: url,
			...properties
		}
	});
	postHogClient.flush();
}
