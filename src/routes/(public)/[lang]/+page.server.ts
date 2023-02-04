import type { PageServerLoad } from "./$types";
import { gql } from "graphql-request";
import type { Signature, SignaturesResponse } from "$lib/fauna-gql/schema";
import { fail } from "@sveltejs/kit";
import { getPrivateClient } from "$lib/fauna-gql/private.client";

function shuffle(array: Signature[]): Signature[] {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
	return array;
}

export const load: PageServerLoad = async ({ fetch }) => {
	const client = getPrivateClient(fetch);

	const approvedSignatures: SignaturesResponse = await client
		.request(
			gql`
				query {
					allApprovedSignatures {
						data {
							_id
						}
					}
				}
			`
		)
		.then((res) => res.allApprovedSignatures)
		.catch((err) => {
			console.log(err);
		});

	if (!approvedSignatures.data) return fail(500, { msg: "query failed" });

	if (approvedSignatures.data.length > 0) {
		const firstResult = await client
			.request(
				gql`
					query ($id: ID!) {
						findSignatureByID(id: $id) {
							_id
							_ts
							user_identifier
							name
							status
							ts_created
							ts_moderated
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
				{ id: shuffle(approvedSignatures.data)[0]._id }
			)
			.then((res) => res.findSignatureByID)
			.catch((err) => {
				console.log(err);
			});
		return {
			signatureRefs: approvedSignatures.data,
			currentSignature: firstResult
		};
	} else {
		return {
			signatureRefs: approvedSignatures.data
		};
	}
};
