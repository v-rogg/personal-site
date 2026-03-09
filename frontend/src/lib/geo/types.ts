export type PipelineStep =
	| "idle"
	| "geocoding"
	| "fetching"
	| "parsing"
	| "building"
	| "ready"
	| "error";

export interface GeocodingResult {
	lat: number;
	lng: number;
	displayName: string;
}

export const SURFACE_LABELS: Record<string, string> = {
	RoofSurface: "Dach",
	WallSurface: "Wand",
	GroundSurface: "Boden",
};

export const STEP_LABELS = ["Geokodierung", "Download", "Verarbeitung", "GeoParquet"] as const;
