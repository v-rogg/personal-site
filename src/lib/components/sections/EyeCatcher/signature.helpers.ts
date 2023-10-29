import {
	currentSignatureStore,
	refIndexStore,
	signatureRefsStore
} from "$lib/components/sections/EyeCatcher/signature.stores";
import { get as getStore } from "svelte/store";
import type { Signature } from "$drizzle/types";
import { PUBLIC_SIGNATURES_WORKER } from "$env/static/public";
import { get } from "$lib/connection/fetch.pulic";


export async function loadDelta(delta: number) {
	const deltaIndex = getStore(refIndexStore) + delta;

	if (deltaIndex < 0 || deltaIndex >= getStore(signatureRefsStore).length) return;

	try {
		const signatureRefs: Partial<Signature>[] = getStore(signatureRefsStore);
		const id = signatureRefs[deltaIndex].id;

		if (id === undefined) {
			return Error("ID is undefined");
		}

		const res = await get<Signature>(`${PUBLIC_SIGNATURES_WORKER}/${id}`)

		console.log(res);

		refIndexStore.set(deltaIndex);
		currentSignatureStore.set(res);
	} catch (error) {
		return error;
	}
}
