import { getSessionId } from "./session";

const API_BASE = import.meta.env.PUBLIC_API_URL || "";

/**
 * Track a custom event (e.g., signature editor actions, button clicks)
 * Page views are tracked server-side via middleware
 */
export async function trackEvent(
	eventName: string,
	properties?: Record<string, unknown>
): Promise<void> {
	const sessionId = getSessionId();
	if (!sessionId) return;

	try {
		await fetch(`${API_BASE}/api/tracking/events`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				session_id: sessionId,
				event_name: eventName,
				properties
			})
		});
	} catch (error) {
		console.error("Event tracking error:", error);
	}
}
