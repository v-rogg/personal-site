import type { GeocodingResult } from "./types";

// Bavaria bounding box
const BAVARIA_BOUNDS = {
	minLat: 47.27,
	maxLat: 50.56,
	minLng: 8.98,
	maxLng: 13.84,
};

export function isInBavaria(lat: number, lng: number): boolean {
	return (
		lat >= BAVARIA_BOUNDS.minLat &&
		lat <= BAVARIA_BOUNDS.maxLat &&
		lng >= BAVARIA_BOUNDS.minLng &&
		lng <= BAVARIA_BOUNDS.maxLng
	);
}

interface NominatimResult {
	lat: string;
	lon: string;
	display_name: string;
	address?: {
		road?: string;
		house_number?: string;
		postcode?: string;
		city?: string;
		town?: string;
		village?: string;
		municipality?: string;
	};
}

async function nominatimSearch(q: string, signal?: AbortSignal): Promise<NominatimResult[]> {
	const params = new URLSearchParams({
		q,
		format: "json",
		countrycodes: "de",
		bounded: "1",
		viewbox: `${BAVARIA_BOUNDS.minLng},${BAVARIA_BOUNDS.minLat},${BAVARIA_BOUNDS.maxLng},${BAVARIA_BOUNDS.maxLat}`,
		limit: "5",
		addressdetails: "1",
	});

	const resp = await fetch(`https://nominatim.openstreetmap.org/search?${params}`, {
		signal,
		headers: { "User-Agent": "valentinrogg.de GeoViewer" },
	});

	if (!resp.ok) throw new Error("Geocoding failed");
	return resp.json();
}

function formatResults(data: NominatimResult[], overrideDisplayName?: string): GeocodingResult[] {
	const seen = new Set<string>();
	return data
		.filter((r) => isInBavaria(parseFloat(r.lat), parseFloat(r.lon)))
		.map((r) => {
			const a = r.address;
			const parts: string[] = [];
			if (a?.road) parts.push(a.house_number ? `${a.road} ${a.house_number}` : a.road);
			const postcode = a?.postcode;
			const city = a?.city || a?.town || a?.village || a?.municipality;
			if (postcode && city) parts.push(`${postcode} ${city}`);
			else if (city) parts.push(city);
			const displayName = overrideDisplayName ?? (parts.length > 0 ? parts.join(", ") : r.display_name);
			return { lat: parseFloat(r.lat), lng: parseFloat(r.lon), displayName };
		})
		.filter((r) => {
			if (seen.has(r.displayName)) return false;
			seen.add(r.displayName);
			return true;
		});
}

export async function geocode(query: string, signal?: AbortSignal): Promise<GeocodingResult[]> {
	const data = await nominatimSearch(query, signal);
	const results = formatResults(data);
	if (results.length > 0) return results;

	// Fallback: try geocoding just the city/postcode part (after last comma)
	const parts = query.split(",");
	if (parts.length >= 2) {
		const cityPart = parts[parts.length - 1].trim();
		const streetPart = parts.slice(0, -1).join(",").trim();
		if (cityPart.length >= 2) {
			const fallbackData = await nominatimSearch(cityPart, signal);
			// Use city coordinates but keep the full original address as displayName
			return formatResults(fallbackData, `${streetPart}, ${cityPart}`).slice(0, 1);
		}
	}

	return [];
}
