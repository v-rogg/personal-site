import type { PageLoad } from "./$types";
import { PUBLIC_NODE_ENV } from "$env/static/public";

export const prerender = false;

export const load: PageLoad = async ({ data, parent, params }) => {
	const { storyblokApi } = await parent();

	const version = PUBLIC_NODE_ENV === "production" ? "published" : "draft";

	const [page] = await Promise.all([
		storyblokApi.get(`cdn/stories/${params.lang}`, { version }),
	]);

	return {
		page: page.data.story,
		...data
	};
};
