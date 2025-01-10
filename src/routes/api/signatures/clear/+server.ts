import { clearSignatureCache } from "$lib/d1";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = () => {
	clearSignatureCache();

	return new Response(
		JSON.stringify({
			status: 200,
			body: {
				message: "Cache cleared"
			}
		})
	);
};
