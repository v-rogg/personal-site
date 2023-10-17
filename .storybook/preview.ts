import type { Preview } from "@storybook/svelte";
import "../src/app.scss";

import { withThemeByClassName } from "@storybook/addon-themes";

const preview: Preview = {
    parameters: {
		actions: { argTypesRegex: "^on[A-Z].*" },
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/
			}
		}
	},

    decorators: [withThemeByClassName({
        themes: {
            // nameOfTheme: 'classNameForTheme',
            light: 'light',
            dark: 'dark',
        },
        defaultTheme: 'light',
    })]
};

export default preview;
