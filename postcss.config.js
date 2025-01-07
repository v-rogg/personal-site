export default {
	plugins: {
		tailwindcss: {},
		autoprefixer: {},
		...(process.env.NODE_ENV === "productiion" ? { cssnano: {} } : {})
	}
};
