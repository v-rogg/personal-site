import type { PageLoad } from "./$types";
import { PUBLIC_NODE_ENV } from "$env/static/public";

export const prerender = false;

export const load: PageLoad = async ({ data, parent, params }) => {
	const { storyblokApi } = await parent();

	console.log(PUBLIC_NODE_ENV);

	const cv = await storyblokApi.get(`cdn/stories/cv/${params.lang}`, {
		version: PUBLIC_NODE_ENV === "production" ? "published" : "draft",
	});

	return {
		cv: cv.data.story,
		...data
	};
};
