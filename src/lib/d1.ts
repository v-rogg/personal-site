import { CF_ACCOUNT_ID, CF_D1_API_KEY, CF_D1_DATABASE } from "$env/static/private";
import type { SignatureMeta, Signature } from "$lib/types";

const cache = new Map();

export async function getSignatures() {
	return await fetch(
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
}

export async function getSignature(id: string) {
	if (cache.has(id)) {
		console.log("cache triggered");
		return cache.get(id);
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

	cache.set(id, res);
	return res;
}

export function clearSignatureCache() {
	cache.clear();
}
