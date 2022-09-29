<script lang="ts">
import ATag from "./ATag.svelte";
import ImgTag from "./IMGTag.svelte";
import { fetch } from '@tauri-apps/api/http';

    type Token = {
        type: string,
        href: string,
        content: string
    }

    function isImage(url : string) : Promise<boolean> {
        return fetch(url, {method: 'HEAD'}).then(res => {
                let value : boolean = false;
                if(res != null)
                {
                    let content = res.headers['content-type']
                    console.log(res.headers)
                    if(content)
                    {
                        value = content.startsWith('image');
                    }
                }
                console.log(value)
                return value;
        })
    }
export let tokens : Token[] = [];
</script>

{#each tokens as token}
    {#if token.type == 'ATag'}
    {#await isImage(token.href)}
        
    {:then value}
    {#if value}
    <p>
        <ImgTag href={token.href}></ImgTag>
    </p>
    {:else}
    <ATag href={token.href}>{token.content}</ATag>
    {/if}
    {/await }
    {:else}
    <span>{token.content}</span>
    {/if}
{/each}
