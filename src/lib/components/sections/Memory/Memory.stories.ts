import type { Meta, StoryObj } from "@storybook/svelte";
import Memory from "$lib/components/sections/Memory/Memory.svelte";
import data from "./sample.data.json";

const meta = {
	title: "Sections/Memory",
	component: Memory,
	tags: ["autodocs"],
	render: () => {
		const blok = data.story.content;
		return { Component: Memory, props: { blok } };
	}
} satisfies Meta<Memory>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {};
