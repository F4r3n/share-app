<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMount } from 'svelte';
import { createEventDispatcher } from 'svelte';
import MessageRenderer from "./MessageRenderer/MessageRenderer.svelte"

type Token = {
        type: string,
        href: string,
        content: string
    }
    const dispatch = createEventDispatcher();

    let isASCII = false;
    export let content = ""
    let regex : RegExp = /(http:\/\/|https:\/\/){1}(www.)?.[^\s]+/g

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


    onMount(async () => {
        isASCII = isASCIIArt(content);
        dispatch("message-formatted");
    });

    function createTokens(inContent : string) : Token[] {
        let tokens : Token[] = [] as Token[];
        let match = null;
        let start = 0;
        let end = inContent.length;
        while((match = regex.exec(inContent)) != null) {
            if(match.index != start) {
                tokens.push({type:"RAW", content:inContent.substring(start, match.index), href:""})
            }
            start = match.index;
            const url = inContent.substring(match.index, match.index + match[0].length)
            start = match.index + match[0].length;

            tokens.push({type:"ATag", content:url, href:url})
        }

        if(end !== start) {
            tokens.push({type:"RAW", content:inContent.substring(start, end), href:""})
        }
        return tokens;
    }

</script>
<div class="message" class:message-ascii={isASCII}>
    <MessageRenderer tokens={createTokens(content)}></MessageRenderer>
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