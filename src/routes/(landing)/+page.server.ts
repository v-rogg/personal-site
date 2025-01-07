import { APP_VERSION } from "$env/static/private";
import { checkSignature, getSignatures } from "$lib/d1";
import type { Metadata } from "$lib/types";
import type { PageServerLoad } from "./$types";

function getBlogData(): {
	route: string;
	metadata: Metadata;
}[] {
	const files = import.meta.glob("/src/routes/\\(content\\)/blog/**/*.md", { eager: true });
	const blogData = Object.entries(files)
		.map(([filePath, module]) => {
			const route = filePath.replace("/src/routes/(content)/", "").replace("/content", "").replace(".md", "");

			return {
				route,
				// @ts-expect-error unknown
				metadata: module.metadata
			};
		})
		.sort((a, b) => {
			if (a.metadata && b.metadata && a.metadata.date && b.metadata.date) {
				const dateA = new Date(a.metadata.date);
				const dateB = new Date(b.metadata.date);
				return dateB.getTime() - dateA.getTime();
			} else {
				return -1;
			}
		})
		.filter((blog) => {
			if (APP_VERSION === "production") {
				return blog.metadata?.published === true;
			}
			return true;
		});

	return blogData;
}

export const load: PageServerLoad = async ({ platform, url }) => {
	if (platform) {
		const signatures = (await getSignatures(platform)).sort(() => Math.random() - 0.5);

		const requestedSignature = url.searchParams.get("s");
		if (requestedSignature) {
			const check = await checkSignature(platform, requestedSignature);

			if (check) {
				const index = signatures.findIndex((signature) => signature.id === check.id);
				let updatedSignatures = [...signatures];
				if (index === -1) {
					updatedSignatures.unshift(check);
				} else {
					updatedSignatures = [
						updatedSignatures[index],
						...updatedSignatures.slice(0, index),
						...updatedSignatures.slice(index + 1)
					];
				}

				return { signatures: updatedSignatures, autoplay: false };
			}
		}

		const blogData = getBlogData();
		const allTags = blogData.reduce((tags: string[], blog) => {
			const blogTags = blog.metadata?.tags || [];
			return [...new Set([...tags, ...blogTags])];
		}, []);

		return {
			signatures,
			autoplay: true,
			blog: blogData,
			allTags
		};
	}
	return {
		signatures: [],
		autoplay: false,
		blog: [],
		allTags: []
	};
};
