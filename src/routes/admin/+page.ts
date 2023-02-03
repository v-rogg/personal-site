import type { PageLoad } from "./$types";
// import { admin, currentSignatureStore, refIndexStore, signatureRefsStore } from "$lib/stores";
import { admin } from "$lib/stores";
// import { get } from "svelte/store";
import { gql, GraphQLClient } from "graphql-request";
import type { newSignaturesResponse } from "$lib/fauna-gql/schema";
import { PUBLIC_FAUNA_GQL_ENDPOINT, PUBLIC_FAUNA_SECRET } from "$env/static/public";

export const load: PageLoad = async ({ fetch }) => {
	admin.set(true);

	const publicFGQLClient = new GraphQLClient(PUBLIC_FAUNA_GQL_ENDPOINT, {
		fetch,
		headers: {
			Authorization: `Bearer ${PUBLIC_FAUNA_SECRET}`
		}
	});

	const newSignatures: newSignaturesResponse = await publicFGQLClient
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

	if (!newSignatures.data) return;

	const firstResult = await publicFGQLClient.request(
		gql`
			query ($id: ID!) {
				findSignatureByID(id: $id) {
					name
					status
				}
			}
		`,
		{ id: newSignatures.data[0]._id }
	);

	console.log(firstResult);

	// Loading new signatures...
	//
	// const refs = await fetch(`${url.origin}/_api/signature?admin=true`, {
	// 	method: "GET"
	// })
	// 	.then((res) => res.json())
	// 	.then((json) => {
	// 		return json;
	// 	});
	//
	// const shuffledSigRefs = refs.data;
	//
	// let signature;
	//
	// if (shuffledSigRefs.length) {
	// 	signature = await fetch(
	// 		`${url.origin}/_api/signature?ref=${shuffledSigRefs[get(refIndexStore)]["@ref"].id}`,
	// 		{
	// 			method: "GET"
	// 		}
	// 	)
	// 		.then((res) => res.json())
	// 		.then((json) => {
	// 			return json;
	// 		});
	// }
	//
	// if (shuffledSigRefs && signature)
	// 	return { signatureRefs: shuffledSigRefs, currentSignature: signature };

	// return { signatureRefs: get(signatureRefsStore), currentSignature: get(currentSignatureStore) };
};
