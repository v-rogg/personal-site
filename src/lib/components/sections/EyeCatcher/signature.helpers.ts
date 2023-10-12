import {
	currentSignatureStore,
	refIndexStore,
	signatureRefsStore
} from "$lib/components/sections/EyeCatcher/signature.stores";
import { fql, type Client } from "fauna";
import { get } from "svelte/store";
import { getPublicFaunaClient } from "$lib/fauna/fauna.public";
import type { Signature } from "$lib/fauna/schema";

export async function loadDelta(delta: number, fauna: Client|undefined = undefined) {
	const deltaIndex = get(refIndexStore) + delta;

	if (deltaIndex < 0 || deltaIndex >= get(signatureRefsStore).length) return;

	let clientCreated = false;
	if (!fauna) {
		fauna = getPublicFaunaClient();
		clientCreated = true;
	}

	try {

		const signatureRefs: Partial<Signature>[] = get(signatureRefsStore)
		const id = signatureRefs[deltaIndex].id;

		if (id === undefined) {
			return Error("ID is undefined")
		}

		const res = await fauna
			.query<Signature>(
				fql`
				signatures.where(.id == ${id}).first()
			`
			)
			.then((res) => res.data);
		refIndexStore.set(deltaIndex);
		currentSignatureStore.set(res);
	} catch (error) {
		return error;
	}

	if (clientCreated) fauna.close()
}