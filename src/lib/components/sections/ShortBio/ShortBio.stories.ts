import { defaultLocale, loadTranslations } from "$lib/_i18n";
import type { Meta, StoryObj } from "@storybook/svelte";
import ShortBio from "./ShortBio.svelte";

const meta = {
	title: "Sections/ShortBio",
	component: ShortBio,
	tags: ["autodocs"],
	argTypes: {
		short_bio: { control: "text" },
		align: {
			control: { type: "inline-radio" },
			options: ["left", "center", "right"]
		},
		flags: {
			control: { type: "boolean" }
		},
		locale: {
			control: { type: "select" },
			options: ["en", "de"]
		}
	},
	args: {
		short_bio:
			"Hey there, I am Valentin, and my professional passion is crafting data experiences. I specialize in automated data collection, analysis, and interactive data visualization. On my own time, I am an aspiring musician and globetrotter.",
		align: "center",
		locale: "en",
		flags: false
	},
	render: ({ short_bio, align, locale, flags }) => {
		loadTranslations(locale || defaultLocale, "/");
		const blok = { blok: { content: { short_bio, align, flags } } };
		return { Component: ShortBio, props: blok };
	}
} satisfies Meta<ShortBio>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/svelte/writing-stories/args
export const Left: Story = {
	args: {
		align: "left"
	}
};
export const Center: Story = {
	args: {
		align: "center",
		flags: true
	}
};

export const Right: Story = {
	args: {
		align: "right"
	}
};
