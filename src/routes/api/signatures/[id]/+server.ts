import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { getSignature } from "$lib/d1";

export const GET: RequestHandler = async ({ params }) => {
	return json(await getSignature(params.id));
};
