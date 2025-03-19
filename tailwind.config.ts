import type { Config } from 'tailwindcss';
import forms from '@tailwindcss/forms';

export default {
	darkMode: 'class',
	content: [
		"./app.html",
		"./src/**/*.{js,ts,svelte}",
	],
	theme: {
		extend: {}
	},
	plugins: [
		forms,
	]
} satisfies Config;