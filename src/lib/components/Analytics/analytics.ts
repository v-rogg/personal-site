import { env } from "$env/dynamic/private";

interface ConversionData {
	conversion_step_1?: number;
	conversion_step_2?: number;
	conversion_rate?: number;
}

interface DrawingDurationsData {
	drawing_durations?: Array<{
		id: string;
		duration_seconds: number;
		ts_created: number;
	}>;
	has_data?: boolean;
}

interface EraserData {
	average_eraser_uses?: number;
}

interface DrawingTimeData {
	average_drawing_time_seconds?: number;
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
	has_drawing_data: boolean;
	average_eraser_uses: number;
	average_drawing_time_seconds: number;
}

export async function getStats(
	platform: App.Platform,
	timeframe = "d180"
): Promise<AnalyticsStats> {
	console.log(env.PH_ANALYTICS_WORKER_KEY);

	const headers = {
		Authorization: `Bearer ${env.PH_ANALYTICS_WORKER_KEY}`
	};

	try {
		// Fetch data from all 4 endpoints in parallel
		const [conversionResponse, drawingDurationsResponse, eraserResponse, drawingTimeResponse] =
			await Promise.all([
				platform.env.PH_ANALYTICS_WORKER.fetch(
					`https://worker/conversion-rate?timeframe=${timeframe}`,
					{ headers }
				),
				platform.env.PH_ANALYTICS_WORKER.fetch(
					`https://worker/drawing-durations?timeframe=${timeframe}`,
					{ headers }
				),
				platform.env.PH_ANALYTICS_WORKER.fetch(
					`https://worker/eraser-uses?timeframe=${timeframe}`,
					{ headers }
				),
				platform.env.PH_ANALYTICS_WORKER.fetch(
					`https://worker/drawing-durations?timeframe=${timeframe}&aggregate=average`,
					{ headers }
				)
			]);

		// Parse all responses with proper typing
		const [conversionData, drawingDurationsData, eraserData, drawingTimeData] = await Promise.all([
			conversionResponse.json() as Promise<ConversionData>,
			drawingDurationsResponse.json() as Promise<DrawingDurationsData>,
			eraserResponse.json() as Promise<EraserData>,
			drawingTimeResponse.json() as Promise<DrawingTimeData>
		]);

		// Combine all data into a single stats object
		const stats: AnalyticsStats = {
			// Conversion rate data
			conversion_step_1: conversionData.conversion_step_1 || 0,
			conversion_step_2: conversionData.conversion_step_2 || 0,
			conversion_rate: conversionData.conversion_rate || 0,

			// Drawing durations data
			drawing_durations: drawingDurationsData.drawing_durations || [],
			has_drawing_data: drawingDurationsData.has_data || false,

			// Eraser uses data
			average_eraser_uses: eraserData.average_eraser_uses || 0,

			// Drawing time data
			average_drawing_time_seconds: drawingTimeData.average_drawing_time_seconds || 0
		};

		return stats;
	} catch (error) {
		console.error("Error fetching analytics stats:", error);
		// Return default values if any request fails
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
