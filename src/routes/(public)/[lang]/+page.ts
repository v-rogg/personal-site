import type { PageLoad } from "./$types";

export const prerender = false;

export const load: PageLoad = async ({ data, parent, params }) => {
	const { storyblokApi } = await parent();

	const cv = await storyblokApi.get(`cdn/stories/cv/${params.lang}`, {
		version: "draft",
	});

	return {
		cv: cv.data.story,
		...data
	};
};
