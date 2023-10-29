import type { PageServerLoad } from "./$types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import type { ID, Signature } from "$drizzle/types";
import { PUBLIC_SIGNATURES_WORKER } from "$env/static/public";
import { get } from "$lib/connection/fetch.pulic";
import { postPrivate } from "$lib/connection/fetch.private";

export const actions: Actions = {
	create: async ({ request }) => {
		const form = await request.formData();
		const data = <Signature>JSON.parse(form.get("data")?.toString() || "{}");

		if (!data.name) return fail(400, { name: data.name, missing: true });
		if (!data.signature) return fail(400, { signature: data.signature, missing: true });

		try {
			return await postPrivate<Signature>(PUBLIC_SIGNATURES_WORKER, { name: data.name, signature: data.signature }, fetch);
		} catch (error) {
			return fail(500, { msg: String(error) });
		}
	}
};

function shuffle(array: Partial<Signature>[]): Partial<Signature>[] {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
	return array;
}

export const load: PageServerLoad = async ({ url, fetch }) => {
	const requestedSID: ID | undefined = url.searchParams.get("sid") || undefined;

	try {
		let signatureRefs = await get<Partial<Signature>[]>(PUBLIC_SIGNATURES_WORKER, fetch);

		console.log(signatureRefs);

		signatureRefs = shuffle(signatureRefs);

		console.log(signatureRefs);
		console.log(requestedSID);
		if (requestedSID) {
			signatureRefs = signatureRefs.filter((signature) => signature.id != requestedSID);
			signatureRefs.unshift({ id: requestedSID });
		}
		if (signatureRefs.length > 0) {

			console.log(signatureRefs[0].id);

			const firstResult = await get<Signature>(`${PUBLIC_SIGNATURES_WORKER}/${signatureRefs[0].id}`);

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
