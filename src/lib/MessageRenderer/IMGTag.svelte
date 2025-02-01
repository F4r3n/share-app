<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
    let {href, onMessageFormatted} : {
        href : string,
        onMessageFormatted : ()=>void
    } = $props();

</script>

<div class="image">
    <img width="80%" onload={()=> {onMessageFormatted()}} src={href} alt={href}>
</div>

<style>

</style>