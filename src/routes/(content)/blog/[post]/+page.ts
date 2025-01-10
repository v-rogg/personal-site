import type { Component } from "svelte";
import type { PageLoad } from "./$types";
import type { Metadata } from "$lib/types";
import { redirect } from "@sveltejs/kit";

export const load: PageLoad = async ({ params }) => {
	const post = params.post;

	const files = import.meta.glob("../content/*.mdx");
	const filePath = `../content/${post}.mdx`;

	if (files[filePath]) {
		const fileContent = (await files[filePath]()) as {
			default: Component;
			metadata: Metadata;
		};

		return {
			content: fileContent.default,
			metadata: fileContent.metadata
		};
	}

	redirect(404, "/#blog");
};
