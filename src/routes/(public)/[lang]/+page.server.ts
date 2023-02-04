import type { PageServerLoad } from "./$types";
import { gql, GraphQLClient } from "graphql-request";
import type { Signature, SignaturesResponse } from "$lib/fauna-gql/schema";
import { PUBLIC_FAUNA_GQL_ENDPOINT } from "$env/static/public";
import { FAUNA_SECRET } from "$env/static/private";
import { fail } from "@sveltejs/kit";

function shuffle(array: Signature[]): Signature[] {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
	return array;
}

export const load: PageServerLoad = async ({ fetch }) => {
	const client = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
		fetch,
		headers: {
			Authorization: `Bearer ${FAUNA_SECRET}`
		}
	});

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
