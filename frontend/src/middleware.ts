import { defineMiddleware } from "astro:middleware";

const API_URL = import.meta.env.API_URL || process.env.API_URL || "http://api:8080";

export const onRequest = defineMiddleware(async (context, next) => {
	const { request, url } = context;

	// Skip non-page requests (assets, API calls, etc.)
	if (
		url.pathname.startsWith("/api") ||
		url.pathname.startsWith("/files") ||
		url.pathname.startsWith("/_") ||
		url.pathname.includes(".")
	) {
		return next();
	}

	// Generate anonymous session ID per request (no cookies = no banner needed)
	// Sessions are not tracked across page loads - this is a GDPR trade-off
	const sessionId = crypto.randomUUID();

	// Track page view (fire and forget - don't await)
	const referrer = request.headers.get("referer") || "";
	const userAgent = request.headers.get("user-agent") || "";

	fetch(`${API_URL}/api/tracking/pageviews`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			session_id: sessionId,
			path: url.pathname,
			referrer,
			user_agent: userAgent
		})
	}).catch(() => {}); // Ignore errors, don't block response

	return next();
});
