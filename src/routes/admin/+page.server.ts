import type { PageServerLoad } from "./$types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { getPrivate, putPrivate, deletePrivate } from "$lib/connection/fetch.private";
import { PUBLIC_SIGNATURES_WORKER } from "$env/static/public";
import { get } from "$lib/connection/fetch.pulic";
import type { ID, Signature } from "$drizzle/types";
import type { Fetch } from "$lib/connection/fetch";

async function updateStatus(id: ID, approved: true | false | null, fetch: Fetch) {

	try {
		const update = await putPrivate<Partial<Signature>>(`${PUBLIC_SIGNATURES_WORKER}/${id}`, {approved, ts_modified: new Date()}, fetch)
		console.log(update);
		return update.approved;
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
}

export const actions: Actions = {
	approve: async ({ request, fetch }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		const approved = await updateStatus(<ID>id, true, fetch);

		return {
			approved
		};
	},
	decline: async ({ request, fetch }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		const approved = await updateStatus(<ID>id, false, fetch);

		return {
			approved
		};
	},
	delete: async ({ request }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		try {
			await deletePrivate(`${PUBLIC_SIGNATURES_WORKER}/${id}`)
			return {
				deleted: true
			};
		} catch (error) {
			return fail(500, { msg: String(error) });
		}
	}
};

export const load: PageServerLoad = async ({ url, fetch }) => {
	const filter = url.searchParams.get("filter") || "new";

	try {
		let signatureRefs: Partial<Signature>[];
		if (filter === "new") {
			signatureRefs = await getPrivate<Partial<Signature>[]>(
				PUBLIC_SIGNATURES_WORKER + "?" + new URLSearchParams({ filter: "unreviewed" }),
				fetch
			);
		} else {
			signatureRefs = await getPrivate<Partial<Signature>[]>(PUBLIC_SIGNATURES_WORKER, fetch);
		}

		console.log(signatureRefs.length);

		if (signatureRefs.length > 0) {
			const firstResult = await get<Signature>(`${PUBLIC_SIGNATURES_WORKER}/${signatureRefs[0].id}`);

			return {
				signatureRefs: signatureRefs,
				currentSignature: firstResult
			};
		} else {
			return {
				signatureRefs: signatureRefs
			};
		}
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
};
