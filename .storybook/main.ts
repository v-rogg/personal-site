import type { StorybookConfig } from "@storybook/sveltekit";

const config: StorybookConfig = {
	stories: ["../src/**/*.mdx", "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
	addons: [
        "@storybook/addon-links",
        "@storybook/addon-essentials",
        "@storybook/addon-interactions",
        "@storybook/addon-themes",
        "@storybook/themes"
    ],
	framework: {
		name: "@storybook/sveltekit",
		options: {}
	},
	docs: {
		autodocs: "tag"
	}
};
export default config;
