/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,ts,svelte}"],
  darkMode: "class",
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      white: "#ffffff",
      dark_blue: "#080C21",
      green: {
        light: "#aeed00",
        DEFAULT: "#93c700",
        dark: "#82ad00"
      }
    },
    fontFamily: {
      sans: ["Candid", "sans-serif"],
      serif: ["PT Serif", "serif"],
    },
    extend: {},
  },
  plugins: [],
}

