<script lang="ts">
    import SettingsIcon from "../assets/setting-line-icon.svelte"

    let {onToggle} : {onToggle : ()=>void} = $props()

</script>
<button class="p-1 border-none hover:text-primary-800" onclick={()=>{onToggle()}}>
    <SettingsIcon></SettingsIcon>
</button>

<style>

</style>