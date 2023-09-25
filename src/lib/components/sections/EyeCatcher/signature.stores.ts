import type { Signature } from "$lib/fauna-gql/schema";
import { writable } from "svelte/store";

export const currentSignatureStore = writable(<Signature>{});
export const signatureRefsStore = writable([]);
export const refIndexStore = writable(0);