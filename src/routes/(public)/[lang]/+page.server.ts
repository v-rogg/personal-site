import type { PageServerLoad } from "./$types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import drizzlePublic from "$lib/drizzle/public";
import drizzlePrivate from "$lib/drizzle/private";
import { signatures } from "$lib/drizzle/schema";
import type { ID, Signature } from "$lib/drizzle/types";
import { eq } from "drizzle-orm";

export const actions: Actions = {
	create: async ({ request }) => {
		const form = await request.formData();
		const data = <Signature>JSON.parse(form.get("data")?.toString() || "{}");

		if (!data.name) return fail(400, { name: data.name, missing: true });
		if (!data.signature) return fail(400, { signature: data.signature, missing: true });

		try {
			return await drizzlePrivate.insert(signatures).values({name: data.name, signature: data.signature}).returning().then(res => res[0])
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

export const load: PageServerLoad = async ({ url }) => {
	const requestedSID: ID | undefined = url.searchParams.get("sid") || undefined;

	try {
		let signatureRefs = await drizzlePublic
			.select({ id: signatures.id })
			.from(signatures)
			.where(eq(signatures.approved, true));

		signatureRefs = shuffle(signatureRefs)

		if (requestedSID) {
			signatureRefs = signatureRefs.filter((signature: Signature) => signature.id != requestedSID);
			signatureRefs.unshift({ id: requestedSID });
		}
		if (signatureRefs.length > 0) {
			const firstResult = await drizzlePublic
				.select({
					id: signatures.id,
					name: signatures.name,
					signature: signatures.signature,
					ts_created: signatures.tsCreated
				})
				.from(signatures)
				.where(eq(signatures.id, signatureRefs[0].id)).limit(1).then(res => res[0]);

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
		console.log(error);
		return {};
		// return fail(500, { msg: String(error) });
	}
};
