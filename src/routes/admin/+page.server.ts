import type { PageServerLoad } from "./$types";
import { gql } from "graphql-request";
import type { ID, SignaturesResponse, Signature } from "$lib/fauna-gql/schema";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { getPrivateClient } from "$lib/fauna-gql/private.client";

async function updateStatus(
	_id: ID,
	status: "approved" | "declined",
	fetch: (input: RequestInfo | URL, init?: RequestInit | undefined) => Promise<Response>
) {
	const client = getPrivateClient(fetch);

	const update: Signature = await client
		.request(
			gql`
				mutation ($id: ID!, $data: PartialUpdateSignatureInput!) {
					partialUpdateSignature(id: $id, data: $data) {
						_id
						_ts
						status
					}
				}
			`,
			{
				id: _id,
				data: <Signature>{
					status,
					ts_moderated: Date.now() * 1000
				}
			}
		)
		.then((res) => res.partialUpdateSignature)
		.catch((err) => {
			console.log(err);
		});

	return update.status;
}

export const actions: Actions = {
	approve: async ({ request, fetch }) => {
		const form = await request.formData();
		const _id = form.get("id")?.toString();

		if (!_id) return fail(400, { _id, missing: true });

		const status = await updateStatus(<ID>_id, "approved", fetch);

		return {
			status
		};
	},
	decline: async ({ request, fetch }) => {
		const form = await request.formData();
		const _id = form.get("id")?.toString();

		if (!_id) return fail(400, { _id, missing: true });

		const status = await updateStatus(<ID>_id, "declined", fetch);

		return {
			status
		};
	}
};

export const load: PageServerLoad = async ({ fetch }) => {
	const client = getPrivateClient(fetch);

	const newSignatures: SignaturesResponse = await client
		.request(
			gql`
				query {
					allNewSignatures {
						data {
							_id
						}
					}
				}
			`
		)
		.then((res) => res.allNewSignatures)
		.catch((err) => {
			console.log(err);
		});

	if (!newSignatures.data) return fail(500, { msg: "query failed" });

	if (newSignatures.data.length > 0) {
		const firstResult = await client
			.request(
				gql`
					query ($id: ID!) {
						findSignatureByID(id: $id) {
							_id
							_ts
							name
							status
							ts_created
							signature {
								penColor
								minWidth
								maxWidth
								dotSize
								points {
									x
									y
									time
									pressure
								}
							}
						}
					}
				`,
				{ id: newSignatures.data[0]._id }
			)
			.then((res) => res.findSignatureByID)
			.catch((err) => {
				console.log(err);
			});
		return {
			signatureRefs: newSignatures.data,
			currentSignature: firstResult
		};
	} else {
		return {
			signatureRefs: newSignatures.data
		};
	}
};
