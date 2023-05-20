<script lang="ts">
    import SettingsIcon from "../assets/setting-line-icon.svelte"
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="settings-icon" on:click={()=>{dispatch("toggle")}}>
    <SettingsIcon></SettingsIcon>
</div>

<style>
    .settings-icon:hover {
        cursor: pointer;
    }

</style>