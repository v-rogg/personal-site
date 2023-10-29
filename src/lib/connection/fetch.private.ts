import { WORKER_PSK } from "$env/static/private";

const headers: HeadersInit = {
	Authorization: "Bearer " + WORKER_PSK
};

export async function getPrivate<T>(url: RequestInfo | URL, f = fetch): Promise<T> {
	return f(url, {
		headers: {
			...headers
		}
	}).then((res) => res.json());
}

export async function postPrivate<T>(url: RequestInfo | URL, body: object, f = fetch): Promise<T> {
	return f(url, {
		method: "POST",
		body: JSON.stringify(body),
		headers: {
			...headers
		}
	}).then((res) => res.json());
}

export async function putPrivate<T>(url: RequestInfo | URL, body: object, f = fetch): Promise<T> {
	return f(url, {
		method: "PUT",
		body: JSON.stringify(body),
		headers: {
			...headers
		}
	}).then((res) => res.json());
}

export async function deletePrivate<T>(url: RequestInfo | URL, f = fetch): Promise<T> {
	return f(url, {
		method: "DELETE",
		headers: {
			...headers
		}
	}).then((res) => res.json());
}
