import { getSignatures } from "$lib/d1";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
	const signatures = (await getSignatures()).sort(() => Math.random() - 0.5);
	return { signatures };
};
