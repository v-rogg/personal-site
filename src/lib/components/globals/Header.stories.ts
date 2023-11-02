import { defaultLocale, loadTranslations } from "$lib/_i18n";
import Header from "$lib/components/globals/Header.svelte";
import type { Meta, StoryObj } from "@storybook/svelte";

const meta = {
	title: "Globals/Header",
	component: Header,
	tags: ["autodocs"],
	render: ({ locale }) => {
		loadTranslations(locale || defaultLocale, "/");
		return { Component: Header };
	},
	parameters: {
		docs: {
			story: {
				inline: false,
				iframeHeight: 200,
			},
		},
	}
} satisfies Meta<Header>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
};