export async function get<T>(url: RequestInfo | URL, f = fetch): Promise<T> {
	return f(url).then((res) => {
		console.log(res.status, res.statusText, res.text());
		return res.json()
	});
}