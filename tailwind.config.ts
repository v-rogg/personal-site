import type { Config } from "tailwindcss";

export default {
	content: ["./src/**/*.{html,js,ts,svelte}"],
	theme: {
		colors: {
			transparent: "transparent",
			current: "currentColor",
			white: {
				500: "#ffffff",
				600: "#FAF7F5",
				700: "#eee8e6"
			},
			skin: {
				500: "#6B6665"
			},
			blue: {
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
				800: "#3F3F3F"
			}
		},
		fontFamily: {
			sans: ["Pretendard Std Variable", "sans-serif"]
		},
		extend: {
			fontSize: {
				"md-lg": ["1.1rem", "1.6rem"]
			}
		}
	},
	plugins: []
} satisfies Config;
