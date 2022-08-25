// import * as faunadb from "faunadb";
// import type { RequestHandler } from "./$types";
// import { get } from "svelte/store";
// import { faunaDBStore } from "$lib/../stores";
// import type * as fauna from "faunadb";
//
// export const GET: RequestHandler = async ({ url }) => {
//
//   const q = faunadb.query;
//
//   const client: fauna.Client = get(faunaDBStore);
//
//   // let query = q.Map(
//   //   q.Paginate(q.Documents(q.Collection("signatures"))),
//   //   q.Lambda("X", q.Get(q.Var("X")))
//   // )
//
//   let query = q.Paginate(q.Documents(q.Collection("signatures")), {size: 100_000})
//
//   const ref = url.searchParams.get("ref");
//
//   if (ref) {
//     query = q.Get(q.Ref(q.Collection("signatures"), ref))
//   }
//
//   const res = await client
//     .query(query)
//     .then((res) => {
//       console.log(res);
//       return res;
//     })
//     .catch((err) => {
//       console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description);
//     });
//
//   return new Response(JSON.stringify(res), {
//     headers: {
//       "Content-Type": "application/json"
//     }
//   });
//
// };
//
// export const POST: RequestHandler = async ({ request }) => {
//   const json = await request.json();
//
//   // console.log("new POST");
//
//   const q = faunadb.query;
//   const client: fauna.Client = get(faunaDBStore);
//
//   const res = await client
//     .query([
//       q.Create(
//         q.Ref(q.Collection("signatures"), q.Add(q.Count(q.Documents(q.Collection("signatures"))), 1)),
//         {
//         data: { ...json },
//         }
//       )
//     ])
//     .then((res) => {
//       console.log(res);
//       return res
//     })
//     .catch((err) =>
//       console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description)
//     );
//
//   return new Response(JSON.stringify(res), {
//     headers: {
//       "Content-Type": "application/json"
//     }
//   });
// };
