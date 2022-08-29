import { writable } from "svelte/store";
import type { signatureData } from "$lib/types";
import faunadb from "faunadb";

export const dark_mode = writable(false);
export const currentSignatureStore = writable(<signatureData>{});
export const signatureRefsStore = writable([]);
export const refIndexStore = writable(0);
export const faunaDBStore = writable(new faunadb.Client(), set => {
  const { Client } = faunadb;
  const client = new Client({
    secret: `${import.meta.env.VITE_FAUNA_SECRET}`,
    domain: "db.eu.fauna.com",
    scheme: "https",
    port: 443
  });
  set(client)
});
