import type { PageLoad } from "../../../../../.svelte-kit/types/src/routes";
import { get } from "svelte/store";
import { currentSignatureStore, refIndexStore, signatureRefsStore } from "$lib/stores";

function shuffle(array: []) {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
}

export const load: PageLoad = async ({ url }) => {
	const g = get(signatureRefsStore);

	if (g != undefined && g.length === 0) {
		console.log("Load signatures...");

		const refs = await fetch(`${url.origin}/_api/signature`, {
			method: "GET"
		})
			.then((res) => res.json())
			.then((json) => {
				// console.log(json);
				return json;
			});

		const shuffledSigRefs = refs.data;
		shuffle(shuffledSigRefs);

		let signature;

		if (shuffledSigRefs.length) {
			signature = await fetch(
				`${url.origin}/_api/signature?ref=${shuffledSigRefs[get(refIndexStore)]["@ref"].id}`,
				{
					method: "GET"
				}
			)
				.then((res) => res.json())
				.then((json) => {
					// console.log(json);
					return json;
				});
		}

		// currentSignatureStore.set(signature);
		// signatureRefsStore.set(shuffledSigRefs);

		if (shuffledSigRefs && signature)
			return { signatureRefs: shuffledSigRefs, currentSignature: signature };
	}

	return { signatureRefs: get(signatureRefsStore), currentSignature: get(currentSignatureStore) };
};
