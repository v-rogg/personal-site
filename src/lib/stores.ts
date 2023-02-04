import { derived, get, writable } from "svelte/store";
import faunadb from "faunadb";
import { onMount } from "svelte";
import * as cookie from "cookie";
import { v4 as uuid_v4, NIL as uuid_NIL } from "uuid";
import { PUBLIC_FAUNA_SECRET, PUBLIC_TELEMETRYDECK_APP_ID } from "$env/static/public";
import type { Signature } from "$lib/fauna-gql/schema";

export const darkMode = writable(false, (set) => {
	onMount(() => {
		set(cookie.parse(document.cookie)["darkModeEnabled"] === "true");
	});
});
export const currentSignatureStore = writable(<Signature>{});
export const signatureRefsStore = writable([]);
export const slugStore = writable("slug.default");
export const refIndexStore = writable(0);
export const faunaDBStore = writable(new faunadb.Client(), (set) => {
	const { Client } = faunadb;
	const client = new Client({
		secret: PUBLIC_FAUNA_SECRET,
		domain: "db.eu.fauna.com",
		scheme: "https",
		port: 443
	});
	set(client);
});
export const admin = writable(false);
export const identifier = writable(uuid_NIL, (set) => {
	onMount(() => {
		let id = cookie.parse(document.cookie)["id"];
		if (id === undefined) {
			id = uuid_v4();
			document.cookie = `id=${id}`;
		}
		set(id);
	});
});

// // @ts-ignore
// export const telemetry = derived(identifier, async ($identifier, set) => {
// 	console.log($identifier);
// 	if ($identifier != uuid_NIL) {
// 		// @ts-ignore
// 		const pkg = await import("@telemetrydeck/sdk");
// 		const td = new pkg.TelemetryDeck();
// 		td.app(PUBLIC_TELEMETRYDECK_APP_ID);
// 		td.user(get(identifier) ?? "anonymous");
// 		set(td);
// 	}
// });
