import type { PageServerLoad } from "./$types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { getPrivateFaunaClient } from "$lib/fauna/fauna.private";
import { fql } from "fauna";
import type { ID, Pagination, Signature } from "$lib/fauna/schema";

async function updateStatus(id: ID, status: "approved" | "declined") {
	const fauna = getPrivateFaunaClient();

	try {
		const update: Signature = await fauna
			.query<Signature>(
				fql`signatures.byId(${<ID>id})!.update({
			status: ${status},
			ts_moderated: ${Date.now() * 1000}
		})`,
				{ format: "simple" }
			)
			.then((res) => res.data);
		fauna.close();
		return update.status;
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
}

export const actions: Actions = {
	approve: async ({ request }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		const status = await updateStatus(<ID>id, "approved");

		return {
			status
		};
	},
	decline: async ({ request }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		const status = await updateStatus(<ID>id, "declined");

		return {
			status
		};
	},
	delete: async ({ request }) => {
		const form = await request.formData();
		const id = form.get("id")?.toString();

		if (!id) return fail(400, { id, missing: true });

		const fauna = getPrivateFaunaClient();

		try {
			await fauna.query(fql`signatures.byId(${id})!.delete()`);
			fauna.close();
			return {
				deleted: true
			};
		} catch (error) {
			return fail(500, { msg: String(error) });
		}
	}
};

export const load: PageServerLoad = async ({ url }) => {
	const fauna = getPrivateFaunaClient();
	const filter = url.searchParams.get("filter") || "new";

	try {

		let signatureRefs: Signature[];
		if (filter === "new") {
			signatureRefs =
				(await fauna
					.query<Pagination<Signature[]>>(
						fql`signatures.where(.status == "new").take(100) {
				id
			}`
					)
					.then((res) => res.data.data)) || [];
		} else {
			signatureRefs =
				(await fauna
					.query<Pagination<Signature[]>>(
						fql`signatures.all() {
				id
			}`
					)
					.then((res) => res.data.data)) || [];
		}

		if (signatureRefs.length > 0) {
			const firstResult = await fauna
				.query<Signature>(
					fql`
				signatures.where(.id == ${signatureRefs[0].id}).first()
			`,
					{ format: "simple" }
				)
				.then((res) => res.data);

			fauna.close();
			return {
				signatureRefs: signatureRefs,
				currentSignature: firstResult
			};
		} else {
			fauna.close();
			return {
				signatureRefs: signatureRefs
			};
		}
	} catch (error) {
		return fail(500, { msg: String(error) });
	}
};
