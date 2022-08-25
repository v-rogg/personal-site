import { writable } from "svelte/store";
import type { signatureData } from "$lib/types";

export const dark_mode = writable(false);
export const currentSignatureStore = writable(<signatureData>{});
export const signatureRefsStore = writable([]);
export const refIndexStore = writable(0);
export const faunaDBStore= writable();

