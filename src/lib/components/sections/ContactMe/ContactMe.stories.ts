import { defaultLocale, loadTranslations } from "$lib/_i18n";
import type { Meta, StoryObj } from "@storybook/svelte";
import ContactMe from "$lib/components/sections/ContactMe/ContactMe.svelte";

const meta = {
	title: "Sections/ContactMe",
	component: ContactMe,
	tags: ["autodocs"],
	render: ({ locale }) => {
		loadTranslations(locale || defaultLocale, "/");
		return { Component: ContactMe };
	},
	parameters: {
		docs: {
			story: {
				inline: false,
				iframeHeight: 200
			}
		}
	}
} satisfies Meta<ContactMe>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {};
