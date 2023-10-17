import type { PageLoad } from "./$types";
import { PUBLIC_NODE_ENV } from "$env/static/public";

export const prerender = false;

export const load: PageLoad = async ({ data, parent, params }) => {
	const { storyblokApi } = await parent();

	const version = PUBLIC_NODE_ENV === "production" ? "published" : "draft";

	const [cv, bio, memoriesSettings, memories] = await Promise.all([
		storyblokApi.get(`cdn/stories/cv/${params.lang}`, { version }),
		storyblokApi.get(`cdn/stories/bio/${params.lang}`, { version }),
		storyblokApi.get(`cdn/stories/memories/settings`, { version }),
		storyblokApi.get(`cdn/stories`, { version, starts_with: "memories/", content_type: "Memory", per_page: 6, sort_by: "content.date:desc" })
	]);

	return {
		cv: cv.data.story,
		bio: bio.data.story,
		memoriesSettings: memoriesSettings.data.story,
		memories: memories.data.stories,
		...data
	};
};
