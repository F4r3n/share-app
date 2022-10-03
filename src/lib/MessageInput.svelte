<script lang="ts">
import PlusSign from '../assets/plus-sign-svg.svelte';
import { createEventDispatcher } from 'svelte';
import { onMount, onDestroy } from 'svelte';
import { invoke } from '@tauri-apps/api'
import { fetch, ResponseType } from '@tauri-apps/api/http';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const dispatch = createEventDispatcher();
let messageToSend : string;
let input : HTMLInputElement;

let listImages : string[] = []

    async function removeImage(event : Event) {
        invoke("get_image_clipboard").then(image => {
                console.log("IMAGE", image)
                /*fetch(image as string, {method: 'GET', responseType : ResponseType.Binary})
                .then((res) => {
                    let blob = res.data as Blob;
                   
                    listImages.push( URL.createObjectURL(blob));
                    listImages = listImages;
                })*/
                const assetUrl = convertFileSrc(image as string);
                listImages.push( assetUrl);
                listImages = listImages;

           })
           .catch(e => {
            console.error(e)
           })
    }

    onMount(async()=> {
        input.focus();
        addEventListener('paste', removeImage)

    })

    onDestroy(()=> {
        removeEventListener("paste", removeImage);
    })

    
</script>

<main>
<div class="main-input">
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
</div>

</main>


<style>


    main {
        display: flex;
        flex-direction: column;

        width: 100%;
        margin: auto;
    }

    .main-input {
        width: 100%;
        display: flex;
        flex-direction: row;
    }

    input {
        flex-grow: 1;
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

