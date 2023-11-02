import type { Meta, StoryObj } from "@storybook/svelte";
import Project from "$lib/components/sections/Project/Project.svelte";
import data from "./sample.data.json";

const meta = {
	title: "Sections/Project",
	component: Project,
	tags: ["autodocs"],
	render: () => {
		const blok = data;
		return { Component: Project, props: {blok} };
	}
} satisfies Meta<Project>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {};
