import type { PageLoad } from "./$types";
import { admin, currentSignatureStore, refIndexStore, signatureRefsStore } from "$lib/../stores";
import { get } from "svelte/store";

export const load: PageLoad = async ({ url }) => {
  admin.set(true);

  console.log("Loading new signatures...");

  const refs = await fetch(`${url.origin}/_api/signature?admin=true`, {
    method: "GET",
  })
    .then(res => res.json())
    .then(json => {
      return json;
    })

  const shuffledSigRefs = refs.data

  let signature;

  if (shuffledSigRefs.length) {
    signature = await fetch(`${url.origin}/_api/signature?ref=${shuffledSigRefs[get(refIndexStore)]["@ref"].id}`, {
      method: "GET",
    })
      .then(res => res.json())
      .then(json => {
        return json;
      })
  }

  if (shuffledSigRefs && signature) return { signatureRefs: shuffledSigRefs, currentSignature: signature }

  return { signatureRefs: get(signatureRefsStore), currentSignature: get(currentSignatureStore)}
}
