import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { getSignature } from "$lib/d1";

export const GET: RequestHandler = async ({ params, platform }) => {
	if (!platform) return new Response("Platform not connected", { status: 500 });
	return json(await getSignature(params.id, platform));
};
