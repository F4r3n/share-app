<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMount, onDestroy } from 'svelte';
import { open } from '@tauri-apps/api/shell';
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

    let messageWrapper : Element;
    let isASCII = false;
    export let content = ""

    async function clean(inContent : string) : Promise<string> {
        return invoke("sanitize_html", {message:inContent});
    }

    function isASCIIArt(inLine : string) {
        let isNormal = false;
        for(let c of inLine) {
            const charCode = c.charCodeAt(0);
            isNormal ||= !( (charCode >= 32 && charCode <= 47) 
            ||(charCode >= 58 && charCode < 64) 
            ||(charCode >= 91 && charCode < 96)  
            ||(charCode >= 123 && charCode < 126)  
            )
        }
        return !isNormal;
    }

    function parseContent(inContent : string) : string {
        //add a tag as link
        let regex : RegExp = /(?<!>)(http:\/\/|https:\/\/){1}(www.)?.[^\s]+(?!<\/a>)/g
        const content = inContent.replaceAll(regex, (inValue : string)=> {
            return `<a target="_blank" href="${inValue}">${inValue}</a>`
        })
        return content;
    }

    onMount(async () => {
        const html = await clean(parseContent(content));
        messageWrapper.innerHTML = html;
        const externalLinks = messageWrapper.getElementsByTagName('a');


        for (let link of externalLinks) {
            link.addEventListener("click", function(event) {
                let isURL = /^(htpp:\/\/|https:\/\/)/
                if(link.href.match(isURL))
                    open(link.href);
                event.preventDefault();
            }, false);
        }
        isASCII = isASCIIArt(content);
        dispatch("message-formatted");
    });

</script>
<div class="message" class:message-ascii={isASCII} bind:this={messageWrapper}>
</div>
<style>
    .message {
        padding-left: 20px;
        word-wrap: break-word;
        white-space: break-spaces;
    }

    .message-ascii {
        white-space: pre;
        font-family: monospace;
    }
</style>