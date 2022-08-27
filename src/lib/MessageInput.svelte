<script lang="ts">
import PlusSign from '../assets/plus-sign-svg.svelte';
import { createEventDispatcher } from 'svelte';
import { onMount, onDestroy } from 'svelte';

const dispatch = createEventDispatcher();
let messageToSend : string;
let input : HTMLInputElement;

    onMount(async()=> {
        input.focus();
    })
</script>

<input type="text" bind:this={input} bind:value={messageToSend} 
on:keyup={(e)=>{
    if(e.key==='Enter') {
        dispatch("send_message", messageToSend)
        messageToSend = "";
    }
    }}>
<button on:click={(event)=> {
        dispatch("send_message", messageToSend)
        messageToSend = ""
}}>
    <PlusSign width=15 height=15></PlusSign>
</button>

<style>

input {
        width: 100%;
    }

    button {
        padding: 0px;
        border-radius: 4px;
        padding-left: 4px;
        padding-right: 4px;
        margin-left: 2px;
        color: var(--text-color-control);
    }


</style>

