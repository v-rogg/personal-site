import { CF_ACCOUNT_ID, CF_D1_API_KEY, CF_D1_DATABASE } from "$env/static/private";
import type { SignatureMeta, Signature } from "$lib/types";

let listCache: SignatureMeta[] | null = null;
const singleCache = new Map();

export async function getSignatures() {
	if (listCache) {
		console.log("list cache triggered");
		return listCache;
	}

	const res = await fetch(
		`https://api.cloudflare.com/client/v4/accounts/${CF_ACCOUNT_ID}/d1/database/${CF_D1_DATABASE}/query`,
		{
			method: "POST",
			headers: {
				Authorization: `Bearer ${CF_D1_API_KEY}`
			},
			body: JSON.stringify({
				sql: "SELECT id, name FROM signatures WHERE approved = true"
			})
		}
	)
		.then((res) => res.json())
		.then((res) => <SignatureMeta[]>res.result[0].results);

	listCache = res;
	return res;
}

export async function getSignature(id: string) {
	if (singleCache.has(id)) {
		console.log("cache triggered");
		return singleCache.get(id);
	}

	const res = await fetch(
		`https://api.cloudflare.com/client/v4/accounts/${CF_ACCOUNT_ID}/d1/database/${CF_D1_DATABASE}/query`,
		{
			method: "POST",
			headers: {
				Authorization: `Bearer ${CF_D1_API_KEY}`
			},
			body: JSON.stringify({
				sql: "SELECT id, name, ts_created, ts_modified, approved, signature FROM signatures WHERE id = ?",
				params: [id]
			})
		}
	)
		.then((res) => res.json())
		.then((res) => <Signature>res.result[0].results[0]);

	singleCache.set(id, res);
	return res;
}

export function clearSignatureCache() {
	listCache = null;
	singleCache.clear();
}
