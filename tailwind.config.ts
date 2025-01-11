import type { Config } from "tailwindcss";
import typography from "@tailwindcss/typography";

export default {
	content: ["./src/**/*.{html,js,ts,svelte,mdx}"],
	theme: {
		colors: {
			transparent: "transparent",
			current: "currentColor",
			white: {
				DEFAULT: "#ffffff",
				500: "#ffffff",
				// 600: "#FAF7F5",
				// 700: "#eee8e6"
				600: "#f7f4f2",
				700: "#e0dbd9"
			},
			skin: {
				500: "#6B6665"
			},
			blue: {
				DEFAULT: "#18a0fb",
				500: "#18a0fb",
				600: "#0f81cb",
				650: "#2d3e80",
				700: "#253270",
				800: "#141b3d",
				900: "#080C21"
			},
			green: {
				light: "#aeed00",
				DEFAULT: "#93c700",
				dark: "#82ad00"
			},
			black: {
				DEFAULT: "#191919"
			},
			grey: {
				600: "#DEDEDE",
				800: "#3F3F3F",
				900: "#262626"
			},
			red: "#ff0000"
		},
		fontFamily: {
			sans: ["Pretendard Std Variable", "sans-serif"],
			mono: ["JetBrain Mono", "Red Hat Mono", "serif"],
			hand: ["Borel", "sans-serif"]
		},
		extend: {
			fontSize: {
				"md-lg": ["1.1rem", "1.6rem"]
			},
			spacing: {
				"128": "26rem"
			},
			typography: {
				DEFAULT: {
					css: {
						"--tw-prose-th-borders": "#e0dbd9",
						"--tw-prose-td-borders": "#f7f4f2"
					}
				}
			}
		}
	},
	plugins: [typography]
} satisfies Config;
