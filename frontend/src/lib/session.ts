/**
 * Get or create a session ID stored in sessionStorage.
 * No cookies = no GDPR banner needed.
 * Session only persists for the browser tab lifetime.
 */
export function getSessionId(): string | null {
	if (typeof sessionStorage === "undefined") return null;

	let sessionId = sessionStorage.getItem("session_id");
	if (!sessionId) {
		sessionId = crypto.randomUUID();
		sessionStorage.setItem("session_id", sessionId);
	}
	return sessionId;
}
