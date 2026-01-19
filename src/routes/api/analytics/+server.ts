import { getStats } from "$lib/components/Analytics/analytics";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ platform, url }) => {
	if (!platform) {
		return json(
			{
				error: "Platform not available"
			},
			{ status: 500 }
		);
	}

	try {
		const timeframe = url.searchParams.get("timeframe") || "d180";
		const stats = await getStats(platform, timeframe);

		return json(stats);
	} catch (error) {
		console.error("Error fetching analytics data:", error);
		return json(
			{
				error: "Failed to fetch analytics data",
				details: error instanceof Error ? error.message : "Unknown error"
			},
			{ status: 500 }
		);
	}
};
