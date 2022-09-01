import type { RequestHandler } from "./$types";
import { get } from "svelte/store";
import faunadb from "faunadb";
import { faunaDBStore } from "$lib/../stores";

const client = get(faunaDBStore);
const { query } = faunadb;
const q = query;

export const GET: RequestHandler = async ({ url }) => {

  // let query = q.Map(
  //   q.Paginate(q.Documents(q.Collection("signatures"))),
  //   q.Lambda("X", q.Get(q.Var("X")))
  // )

  let query = q.Paginate(q.Documents(q.Collection("signatures")), {size: 100_000})

  const ref = url.searchParams.get("ref");

  if (ref) {
    query = q.Get(q.Ref(q.Collection("signatures"), ref))
  }

  const res = await client
    .query(query)
    .catch((err) => {
      console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description);
    });

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
            "approved": false,
            "seen": false,
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
