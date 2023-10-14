import type { PageServerLoad } from "./$types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { fql } from "fauna";
import { getPublicFaunaClient } from "$lib/fauna/fauna.public";
import { getPrivateFaunaClient } from "$lib/fauna/fauna.private";
import type { ID, Pagination, Signature } from "$lib/fauna/schema";

export const actions: Actions = {
	create: async ({ request }) => {
		const form = await request.formData();
		const data = <Signature>JSON.parse(form.get("data")?.toString() || "{}");

		if (!data.name) return fail(400, { name: data.name, missing: true });
		if (!data.signature) return fail(400, { signature: data.signature, missing: true });

		data.ts_created = Date.now() * 1000;
		data.status = "new";

		const fauna = getPrivateFaunaClient();

		try {
			const res = await fauna
				.query<Signature>(fql`signatures.create(${data})`, { format: "simple" })
				.then((res) => res.data);
			fauna.close()
			return res;
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

export const load: PageServerLoad = async ({url}) => {
	const fauna = getPublicFaunaClient();

	const requestedSID: ID | undefined = url.searchParams.get("sid") || undefined;

	try {

		let signatureRefs =
			(await fauna
				.query<Pagination<Signature[]>>(
					fql`
			signatures.where(.status == "approved").take(1000) {
				id
			}`
				)
				.then((res) => shuffle(res.data.data))) || [];

		if (requestedSID) {
			signatureRefs = signatureRefs.filter(signature => signature.id != requestedSID);
			signatureRefs.unshift({id: requestedSID});

			const firstResult = await fauna
				.query<Signature>(
					fql`
				signatures.where(.id == ${signatureRefs[0].id}).first()
			`,
					{ format: "simple" }
				)
				.then((res) => res.data);

			fauna.close()
			return {
				signatureRefs: signatureRefs,
				currentSignature: firstResult
			};
		} else {
			if (signatureRefs.length > 0) {
				const firstResult = await fauna
					.query<Signature>(
						fql`
				signatures.where(.id == ${signatureRefs[0].id}).first()
			`,
						{ format: "simple" }
					)
					.then((res) => res.data);

				fauna.close()
				return {
					signatureRefs: signatureRefs,
					currentSignature: firstResult
				};
			} else {
				fauna.close()
				return {
					signatureRefs: []
				};
			}
		}
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
};
