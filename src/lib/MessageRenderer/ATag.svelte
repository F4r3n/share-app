<script lang="ts">
    import { onMount, onDestroy } from 'svelte';

    export let href : string;

    onMount(async ()=> {

    })

    onDestroy(async ()=> {

    })

    let link : HTMLAnchorElement;
</script>

<a target="_blank" bind:this={link} href={href}><slot></slot></a>