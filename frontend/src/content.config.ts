import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const blog = defineCollection({
	loader: glob({ pattern: "**/*.mdx", base: "./src/content/blog" }),
	schema: z.object({
		title: z.string(),
		subtitle: z.string().optional(),
		date: z.coerce.date(),
		published: z.boolean().default(false),
		preview: z.string(),
		previewImageUrl: z.string().optional(),
		slug: z.string().optional(),
		tags: z.array(z.string()).optional(),
		timeframe: z.array(z.string()).optional()
	})
});

export const collections = { blog };
