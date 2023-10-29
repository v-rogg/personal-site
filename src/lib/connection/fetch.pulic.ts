export async function get<T>(url: RequestInfo | URL, f = fetch): Promise<T> {
	return f(url).then((res) => res.json());
}