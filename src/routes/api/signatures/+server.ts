import { error, json, type RequestHandler } from "@sveltejs/kit";
import drizzle from "$lib/drizzle/public";
import { signatures } from "$lib/drizzle/schema";
import { eq } from "drizzle-orm";

export const GET: RequestHandler = async ({url}) => {
	const id = url.searchParams.get("id");
	if (id == null) throw error(400);

	const res = await drizzle
		.select({
			id: signatures.id,
			name: signatures.name,
			ts_created: signatures.tsCreated,
			signature: signatures.signature
		})
		.from(signatures)
		.where(eq(signatures.id, id))
		.limit(1)
		.then((res) => res[0]);
	return json(res)
}