<script lang="ts">
    import { open } from '@tauri-apps/api/shell';
    import { onMount, onDestroy } from 'svelte';

    export let href : string;

    onMount(async ()=> {
        link.addEventListener("click", function(event) {
                open(link.href);
                event.preventDefault();
            }, false);
    })

    onDestroy(async ()=> {

    })

    let link : HTMLAnchorElement;
</script>

<a target="_blank" bind:this={link} href={href}><slot></slot></a>