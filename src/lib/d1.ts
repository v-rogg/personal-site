import { env } from "$env/dynamic/private";
import type { SignatureMeta, Signature } from "$lib/types";

let listCache: SignatureMeta[] | null = null;
const singleCache = new Map();

export async function getSignatures(platform: App.Platform) {
	if (listCache) {
		console.log("list cache triggered");
		return listCache;
	}

	const signatures = await platform.env.SIGNATURES_WORKER.fetch("https://worker/", {
		headers: {
			AUTHORIZATION: `Bearer ${env.SIGNATURES_WORKER_KEY}`
		}
	}).then(async (res) => (await res.json()) as SignatureMeta[]);

	listCache = signatures;
	return signatures;
}

export async function getSignature(id: string, platform: App.Platform) {
	if (singleCache.has(id)) {
		console.log("cache triggered");
		return singleCache.get(id);
	}

	const signature = await platform.env.SIGNATURES_WORKER.fetch(`https://worker/${id}`, {
		headers: {
			AUTHORIZATION: `Bearer ${env.SIGNATURES_WORKER_KEY}`
		}
	}).then(async (res) => (await res.json()) as Signature);

	singleCache.set(id, signature);
	return signature;
}

export function clearSignatureCache() {
	listCache = null;
	singleCache.clear();
}
