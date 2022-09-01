import { writable } from "svelte/store";
import type { signatureData } from "$lib/types";
import faunadb from "faunadb";
import { onMount } from "svelte";
import * as cookie from "cookie";

export const dark_mode = writable(false, set => {
  onMount(() => {
    set(cookie.parse(document.cookie)["darkModeEnabled"] === "true");
  })
});
export const currentSignatureStore = writable(<signatureData>{});
export const signatureRefsStore = writable([]);
export const slugStore = writable("slug.default");
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
