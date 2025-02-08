<script lang="ts">
    import { theme_name } from '$lib/theme.svelte';
	import '../app.postcss';

	theme_name.subscribe(setBodyThemeAttribute)
	function setBodyThemeAttribute(value): void {
		document.body.setAttribute('data-theme', value);
	} 

</script>

<slot />