import type { PageServerLoad } from "./$types";
import { get } from "svelte/store";
import { refIndexStore } from "$lib/../stores";

function shuffle(array: []) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

export const load: PageServerLoad = async ({ url }) => {
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
}
