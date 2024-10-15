import { writable } from "svelte/store";
import { onMount } from "svelte";
import * as cookie from "cookie";

export const darkMode = writable(false, (set) => {
	onMount(() => {
		set(cookie.parse(document.cookie)["darkModeEnabled"] === "true");
	});
});
export const slugStore = writable("slug.default");
export const admin = writable(false);
