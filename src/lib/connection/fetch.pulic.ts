export async function get<T>(url: RequestInfo | URL, f = fetch): Promise<T> {
	return f(url).then(async (res) => {
		console.log(res.status, res.statusText);
		if (res.status != 200) {
			console.log(await res.text());
		}
		return res.json()
	});
}