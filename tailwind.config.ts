import type { Config } from 'tailwindcss';
import { skeleton, contentPath } from '@skeletonlabs/skeleton/plugin';
import * as themes from '@skeletonlabs/skeleton/themes';
import forms from '@tailwindcss/forms';

export default {
	darkMode: 'class',
	content: [
		"./app.html",
		"./src/**/*.{js,ts,svelte}",
		contentPath(import.meta.url, "svelte"),
	],
	theme: {
		extend: {}
	},
	plugins: [
		forms,
		skeleton({
			themes: [ themes.modern ]
		})
	]
} satisfies Config;