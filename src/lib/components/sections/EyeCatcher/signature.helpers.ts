import {
	currentSignatureStore,
	refIndexStore,
	signatureRefsStore
} from "$lib/components/sections/EyeCatcher/signature.stores";
import { get } from "svelte/store";
import type { Signature } from "$lib/drizzle/types";


export async function loadDelta(delta: number) {
	const deltaIndex = get(refIndexStore) + delta;

	if (deltaIndex < 0 || deltaIndex >= get(signatureRefsStore).length) return;

	try {
		const signatureRefs: Partial<Signature>[] = get(signatureRefsStore);
		const id = signatureRefs[deltaIndex].id;

		if (id === undefined) {
			return Error("ID is undefined");
		}

		const res = await fetch("/api/signatures?" + new URLSearchParams({id})).then(res => res.json())

		console.log(res);

		refIndexStore.set(deltaIndex);
		currentSignatureStore.set(res);
	} catch (error) {
		return error;
	}
}
