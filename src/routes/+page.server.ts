import { getSignature, getSignatures } from "$lib/d1";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
	const signatures = (await getSignatures()).sort(() => Math.random() - 0.5);
	const first = await getSignature(signatures[0].id);
	return { signatures, first };
};
