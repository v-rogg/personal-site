import { writable } from "svelte/store";
import type { Signature } from "$lib/drizzle/types";

export const currentSignatureStore = writable(<Signature>{});
export const signatureRefsStore = writable([]);
export const refIndexStore = writable(0);