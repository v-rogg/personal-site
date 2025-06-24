import { env } from "$env/dynamic/private";

export async function getWeekStats(platform: App.Platform) {
	console.log(env.PH_ANALYTICS_WORKER_KEY);

	const weekStats = await platform.env.PH_ANALYTICS_WORKER.fetch("https://worker/week", {
		headers: {
			Authorization: `Bearer ${env.PH_ANALYTICS_WORKER_KEY}`
		}
	}).then(async (res) => await res.json());
	return weekStats;
}
