<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    export let href : string;
</script>

<div class="image">
    <img width="80%" on:load={()=> {dispatch("message_formatted")}} src={href} alt={href}>
</div>

<style>

</style>