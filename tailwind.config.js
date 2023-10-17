/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,ts,svelte}"],
  darkMode: "class",
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      white: {
        500: "#ffffff",
        600: "#FAF7F5",
        700: "#eee8e6",
      },
      skin: {
        500: "#6B6665"
      },
      blue: {
        500: "#18a0fb",
        600: "#608CFF",
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
      }
    },
    fontFamily: {
      sans: ["Two Candid", "sans-serif"],
      handwriting: ["Two Caveat", "sans-serif"],
      serif: ["Two PT Serif", "serif"],
      display: ["Two Pulp Display", "sans-serif"],
      mono: ["Two JetBrains Mono", "monospace"]
    },
    extend: {
      fontSize: {
        'md-lg': ["1.1rem", "1.6rem"]
      }
    },
  },
  plugins: [],
}

