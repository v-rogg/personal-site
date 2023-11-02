import { defaultLocale, loadTranslations } from "$lib/_i18n";
import type { Meta, StoryObj } from "@storybook/svelte";
import CV from "$lib/components/sections/CV/CV.svelte";
import data from "./sample.data.json";

const meta = {
	title: "Sections/CV",
	component: CV,
	tags: ["autodocs"],
	render: ({ locale }) => {
		loadTranslations(locale || defaultLocale, "/en");
		const blok = data.story;
		return { Component: CV, props: {blok} };
	}
} satisfies Meta<CV>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {};
