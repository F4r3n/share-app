<script lang="ts">
import PlusSign from '../assets/plus-sign-svg.svelte';
import { createEventDispatcher } from 'svelte';
import { onMount, onDestroy } from 'svelte';
import { invoke } from '@tauri-apps/api'
import { fetch, ResponseType } from '@tauri-apps/api/http';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import {config} from './config'
import { Body } from "@tauri-apps/api/http"

const dispatch = createEventDispatcher();
let messageToSend : string;
let input : HTMLInputElement;

type Image = {
    base64 : string,
    url : string,
    name : string
}

let listImages : Image[] = []

    async function removeImage(event : Event) {
        invoke("get_image_clipboard").then(base64 => {
                let image : Image =  {
                    base64 : base64 as string,
                    url:"",
                    name:"#_image_" + listImages.length
                }
                console.log(image)
                listImages.push( image);
                listImages = listImages;

                messageToSend += image.name;
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


    async function uploadImage(image : Image) : Promise<string> {
        let imageData : Uint8Array = await invoke("decode_base64", {message:image.base64});
        
        const response = await fetch(`${config.getConfig().uploadImage.url_post}`, {
            method: 'POST',
            responseType: ResponseType.Text,
            headers:{"Content-Type": "multipart/form-data"},
            body: Body.form({ 
              upload: { 
                file: imageData,
                mime: 'image/png', // optional
                fileName: 'image.png' // optional
              },
              duration:"86400",
              title:"title"
            })
        });
        return config.getConfig().uploadImage.url_get + response.data as string;
    }
</script>

<main>

{#each listImages as image}
    <img width=150px src={"data:image/png;base64,"+ image.base64}>
{/each}

<div class="main-input">
    <input type="text" bind:this={input} bind:value={messageToSend}
    on:keyup={async (e)=>{
        if(e.key==='Enter') {
            for(let image of listImages) {
                let url = await uploadImage(image);
                messageToSend = messageToSend.replace(image.name, url);
            }
            dispatch("send_message", messageToSend)
            listImages = []
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

