import type { RequestHandler } from "./$types";
import { get } from "svelte/store";
import faunadb from "faunadb";
import { faunaDBStore } from "$lib/stores";

const client = get(faunaDBStore);
const { query } = faunadb;
const q = query;

export const GET: RequestHandler = async ({ url }) => {

	const admin = url.searchParams.get("admin");
	const term = admin ? "new" : "approved";

	let query = q.Paginate(q.Match(q.Index("approved_signatures"), term), {size: 100_000})

	const ref = url.searchParams.get("ref");

	if (ref) {
		query = q.Get(q.Ref(q.Collection("signatures"), ref))
	}

	const res = await client
		.query(query)
		.catch((err) => {
			console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description);
		});

	// console.log(res);

	return new Response(JSON.stringify(res), {
		headers: {
			"Content-Type": "application/json"
		}
	});

};

export const POST: RequestHandler = async ({ request }) => {
	const json = await request.json();

	const res = await client
		.query([
			q.Create(
				q.Ref(q.Collection("signatures"), q.Add(q.Count(q.Documents(q.Collection("signatures"))), 1)) ,
				{
					data: {
						"status": "new",
						"ts_created": Date.now(),
						...json
					},
				}
			)
		])
		.catch((err) =>
			console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description)
		);

	return new Response(JSON.stringify(res), {
		headers: {
			"Content-Type": "application/json"
		}
	});
};

export const PUT: RequestHandler = async ({ request }) => {
	const json = await request.json();

	const res = await client
		.query([
			q.Update(
				q.Ref(q.Collection("signatures"), json.ref),
				{
					data: {
						status: json.status
					}
				}
			)
		])
		.catch((err) =>
			console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description)
		);
	console.log(json);

	return new Response(JSON.stringify(res), {
		headers: {
			"Content-Type": "application/json"
		}
	});
}
