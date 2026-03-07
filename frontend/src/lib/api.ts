const API_BASE = import.meta.env.PUBLIC_API_URL || "";

export interface SignatureMeta {
	id: string;
	name: string;
	ts_created: number;
}

export interface Signature {
	id: string;
	name: string;
	signature: string;
	ts_created: number;
	approved: boolean;
}

export interface AnalyticsStats {
	conversion_step_1: number;
	conversion_step_2: number;
	conversion_rate: number;
	drawing_durations: Array<{
		id: string;
		duration_seconds: number;
		ts_created: number;
	}>;
	average_eraser_uses: number;
	average_drawing_time_seconds: number;
}

export async function getSignatures(): Promise<SignatureMeta[]> {
	const response = await fetch(`${API_BASE}/api/signatures`);
	if (!response.ok) throw new Error("Failed to fetch signatures");
	const data = await response.json();
	return data.signatures;
}

export async function getSignature(id: string): Promise<Signature | null> {
	const response = await fetch(`${API_BASE}/api/signatures/${id}`);
	if (response.status === 404) return null;
	if (!response.ok) throw new Error("Failed to fetch signature");
	return await response.json();
}

export async function createSignature(data: {
	session_id: string;
	name: string;
	email?: string;
	signature: string;
}): Promise<{ id: string; url: string }> {
	const response = await fetch(`${API_BASE}/api/signatures`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data)
	});
	if (!response.ok) throw new Error("Failed to create signature");
	return await response.json();
}

export async function getStats(days = 180): Promise<AnalyticsStats> {
	try {
		const response = await fetch(`${API_BASE}/api/tracking/stats?days=${days}`);
		if (!response.ok) throw new Error("Failed to fetch stats");
		return await response.json();
	} catch (error) {
		console.error("Analytics fetch error:", error);
		return {
			conversion_step_1: 0,
			conversion_step_2: 0,
			conversion_rate: 0,
			drawing_durations: [],
			average_eraser_uses: 0,
			average_drawing_time_seconds: 0
		};
	}
}

export async function submitContact(data: {
	session_id: string;
	email: string;
	message: string;
	captcha_solution: string;
}): Promise<{ success: boolean }> {
	const response = await fetch(`${API_BASE}/api/contact`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data)
	});
	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.error || "Failed to send message");
	}
	return await response.json();
}
