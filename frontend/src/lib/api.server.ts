import type { AnalyticsStats, Signature, SignatureMeta } from "./api";

// Server-side API URL (uses Docker network name in containers)
// In production: API_URL=http://api:8080 for internal Docker communication
// In development: defaults to empty string (same-origin)
const API_BASE = import.meta.env.API_URL || process.env.API_URL || "http://api:8080";

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
			has_drawing_data: false,
			average_eraser_uses: 0,
			average_drawing_time_seconds: 0
		};
	}
}

// Pre-render first N signatures server-side for faster initial load
export async function getSignaturesWithFirstData(
	requestedSignatureId?: string | null,
	preloadCount = 3
): Promise<{
	signatures: SignatureMeta[];
	preloadedSignatures: Signature[];
	autoplay: boolean;
}> {
	try {
		const signaturesData = await getSignatures();
		let signatures: SignatureMeta[] = signaturesData.sort(() => Math.random() - 0.5);
		let autoplay = true;

		// If a specific signature was requested, move it to front
		if (requestedSignatureId) {
			const requestedSig = await getSignature(requestedSignatureId);
			if (requestedSig) {
				const signatureMeta: SignatureMeta = {
					id: requestedSig.id,
					name: requestedSig.name,
					ts_created: requestedSig.ts_created
				};
				const index = signatures.findIndex((s) => s.id === requestedSig.id);
				if (index === -1) {
					signatures.unshift(signatureMeta);
				} else {
					signatures = [
						signatures[index],
						...signatures.slice(0, index),
						...signatures.slice(index + 1)
					];
				}
				autoplay = false;
			}
		}

		// Preload first N signatures server-side
		const preloadedSignatures: Signature[] = [];
		for (let i = 0; i < Math.min(preloadCount, signatures.length); i++) {
			const sig = await getSignature(signatures[i].id);
			if (sig) {
				preloadedSignatures.push(sig);
			}
		}

		return { signatures, preloadedSignatures, autoplay };
	} catch (error) {
		console.error("Error loading signatures:", error);
		return { signatures: [], preloadedSignatures: [], autoplay: false };
	}
}
