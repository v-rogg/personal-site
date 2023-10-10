import type { PageServerLoad } from "./$types";
import type { Pagination, Signature } from "$lib/fauna-gql/schema";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { Client, fql } from "fauna";
import { PUBLIC_FAUNA_SECRET } from "$env/static/public";
import { FAUNA_SECRET } from "$env/static/private";

export const actions: Actions = {
	create: async ({ request }) => {
		const form = await request.formData();
		const data = <Signature>JSON.parse(form.get("data")?.toString() || "{}");

		if (!data.name) return fail(400, { name: data.name, missing: true });
		if (!data.signature) return fail(400, { signature: data.signature, missing: true });

		data.ts_created = Date.now() * 1000;
		data.status = "new";

		console.log(data);

		const fClient = new Client({ secret: FAUNA_SECRET });
		// const client = getPrivateClient(fetch);

		try {
			return await fClient
				.query<Signature>(fql`signatures.create(${data})`, { format: "simple" })
				.then((res) => res.data);
		} catch (error) {
			return fail(500, { msg: String(error) });
		}
	}
};

function shuffle(array: Signature[]): Signature[] {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
	return array;
}

export const load: PageServerLoad = async () => {
	const fClient = new Client({ secret: PUBLIC_FAUNA_SECRET });

	try {
		const signatureRefs =
			(await fClient
				.query<Pagination<Signature[]>>(
					fql`
			signatures.where(.status == "approved").take(100) {
				id
			}`
				)
				.then((res) => shuffle(res.data.data))) || [];

		if (signatureRefs.length > 0) {
			const firstResult = await fClient
				.query<Signature>(
					fql`
				signatures.where(.id == ${signatureRefs[0].id}).first()
			`,
					{ format: "simple" }
				)
				.then((res) => res.data);

			return {
				signatureRefs: signatureRefs,
				currentSignature: firstResult
			};
		} else {
			return {
				signatureRefs: []
			};
		}
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
};
