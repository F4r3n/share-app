<script lang="ts">
	import { theme_name, theme_mode } from "$lib/theme.svelte";

	theme_name.subscribe(setBodyThemeAttribute);
	function setBodyThemeAttribute(value): void {
		document.documentElement.setAttribute("data-theme", value);
	}

	theme_mode.subscribe(setModeCurrent);
	export function setModeCurrent(value: string) {
		document.documentElement.setAttribute("data-mode", value);

	}
</script>

<slot />
