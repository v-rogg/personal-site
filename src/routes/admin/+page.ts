import type { PageLoad } from "./$types";
import { admin } from "$lib/stores";

export const prerender = false;

export const load: PageLoad = async ({ data }) => {
	admin.set(true);
	return data;
};
