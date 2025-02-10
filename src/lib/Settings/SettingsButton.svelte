<script lang="ts">
    import SettingsIcon from "../../assets/setting-line-icon.svelte"

    let {onToggle} : {onToggle : ()=>void} = $props()

</script>
<button class="btn p-1 border-none text-primary-500" onclick={()=>{onToggle()}}>
    <SettingsIcon></SettingsIcon>
</button>

<style>

</style>