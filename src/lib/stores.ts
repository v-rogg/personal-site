import { writable } from "svelte/store";
import { onMount } from "svelte";
import * as cookie from "cookie";

export const darkMode = writable(false, (set) => {
	onMount(() => {
		set(cookie.parse(document.cookie)["darkModeEnabled"] === "true");
	});
});
// export const currentSignatureStore = writable(<Signature>{});
// export const signatureRefsStore = writable([]);
export const slugStore = writable("slug.default");
// export const refIndexStore = writable(0);

export const admin = writable(false);


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
