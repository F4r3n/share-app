<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMount, onDestroy } from 'svelte';
import { open } from '@tauri-apps/api/shell';


    let messageWrapper;
    export let content = ""

    async function clean(inContent : string) : Promise<string> {
        return invoke("sanitize_html", {message:content});
    }


    onMount(async () => {
        let html = await clean(content);
        messageWrapper.innerHTML = html;
        let externalLinks = messageWrapper.getElementsByTagName('a');


        for (let link of externalLinks) {
            link.addEventListener("click", function(event) {
                open(link.href);

                event.preventDefault();
            }, false);
        }
    });

</script>
<div bind:this={messageWrapper}>
</div>
<style>

</style>