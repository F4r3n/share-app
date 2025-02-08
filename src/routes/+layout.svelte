<script lang="ts">
	import { theme_name, theme_mode } from "$lib/theme.svelte";
	import "../app.postcss";

	theme_name.subscribe(setBodyThemeAttribute);
	function setBodyThemeAttribute(value): void {
		document.body.setAttribute("data-theme", value);
	}

	theme_mode.subscribe(setModeCurrent);
	export function setModeCurrent(value: string) {
		const elemHtmlClasses = document.documentElement.classList;
		const classDark = `dark`;
		value === "light"
			? elemHtmlClasses.remove(classDark)
			: elemHtmlClasses.add(classDark);
	}
</script>

<slot />
