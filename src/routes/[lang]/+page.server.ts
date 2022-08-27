import type { PageServerLoad } from "./$types";
import { get } from "svelte/store";
import { faunaDBStore, postgresDB, refIndexStore } from "$lib/../stores";
import faunadb from "faunadb";

function shuffle(array: []) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

export const load: PageServerLoad = async ({ url }) => {

  const db = get(postgresDB)
  console.log(await db.query("SELECT * FROM signatures").then(res => res.rows));

  const fauna = get(faunaDBStore);

  const { query } = faunadb;
  const q = query;
  const res = await fauna
    .query(q.Paginate(q.Documents(q.Collection("signatures")), {size: 100_000}))
    .then((res) => {
      console.log(res);
      return res;
    })
    .catch((err) => {
      console.error("Error: [%s] %s: %s", err.name, err.message, err.errors()[0].description);
    });

  const refs = await fetch(`${url.origin}/api/signature`, {
    method: "GET",
  })
    .then(res => res.json())
    .then(json => {
      // console.log(json);
      return json;
    })

  const shuffledSigRefs = refs.data
  shuffle(shuffledSigRefs)

  let signature;

  if (shuffledSigRefs.length) {
    signature = await fetch(`${url.origin}/api/signature?ref=${shuffledSigRefs[get(refIndexStore)]["@ref"].id}`, {
      method: "GET",
    })
      .then(res => res.json())
      .then(json => {
        console.log(json);
        return json;
      })
  }

  if (shuffledSigRefs && signature) {
    return { signatureRefs: shuffledSigRefs, currentSignature: signature }
  } else {
    return { signatureRefs: [], currentSignature: {}}
  }
  // return { signatureRefs: [], currentSignature: {}}
}
