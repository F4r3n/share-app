<script lang="ts">
    import SettingsIcon from "../assets/setting-line-icon.svelte"
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<button class="p-1 border-none hover:text-primary-800" on:click={()=>{dispatch("toggle")}}>
    <SettingsIcon></SettingsIcon>
</button>

<style>

</style>