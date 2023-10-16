import type { Meta, StoryObj } from "@storybook/svelte";
import ShortBio from './ShortBio.svelte';

const meta = {
	title: "Blok/ShortBio",
	component: ShortBio,
	tags: ["autodocs"],
} satisfies Meta<ShortBio>

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/svelte/writing-stories/args
export const Primary: Story = {
	args: {
		blok: {
			content: {
				short_bio: "Hey there, I am Valentin, and my professional passion is crafting data experiences. I specialize in automated data collection, analysis, and interactive data visualization. On my own time, I am an aspiring musician and globetrotter."
			}
		}
	}
};