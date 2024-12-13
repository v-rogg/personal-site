import { getSignatures } from "$lib/d1";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ platform }) => {
	if (platform) {
		const signatures = (await getSignatures(platform.env.D1)).sort(() => Math.random() - 0.5);
		return { signatures };
	} else {
		return {};
	}
};
