<script lang="ts">
	import { theme_name, theme_mode } from "$lib/theme.svelte";

	theme_name.subscribe(setBodyThemeAttribute);
	function setBodyThemeAttribute(value): void {
		document.documentElement.setAttribute("data-theme", value);
		document.documentElement.setAttribute('data-mode', value);
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
