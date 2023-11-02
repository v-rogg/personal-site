import type { PageLoad } from "./$types";
import { PUBLIC_NODE_ENV } from "$env/static/public";

export const prerender = false;

export const load: PageLoad = async ({ data, parent, params }) => {
	const { storyblokApi } = await parent();

	const version = PUBLIC_NODE_ENV === "production" ? "published" : "draft";

	const [cv, bio, page] = await Promise.all([
		storyblokApi.get(`cdn/stories/cv/${params.lang}`, { version }),
		storyblokApi.get(`cdn/stories/bio/${params.lang}`, { version }),
		storyblokApi.get(`cdn/stories/${params.lang}`, { version }),
	]);

	return {
		cv: cv.data.story,
		bio: bio.data.story,
		page: page.data.story,
		...data
	};
};
