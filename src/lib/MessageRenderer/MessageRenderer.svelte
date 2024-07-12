<script lang="ts">
    import ATag from "./ATag.svelte";
    import LinkPreview from "./LinkPreview.svelte";
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import { afterUpdate } from "svelte";
    import ColorTag from "./ColorTag.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { MetaData } from "./metaData";

    const dispatch = createEventDispatcher();

    type Token = {
        type: string;
        value: any;
    };

    afterUpdate(() => {
        dispatch("message_formatted");
    });

    onMount(async () => {
        for (let token of tokens) {
            if (token.type === "ATag") {
                checkLinks.push(token.value.href);
            }
        }
        checkLinks = checkLinks;
    });

    function getPreview(inURL: string): Promise<MetaData> {
        return invoke("get_url_preview", { endpoint: inURL });
    }

    export let tokens: Token[] = [];
    let checkLinks: string[] = [];
</script>

<main>
    <div>
        {#each tokens as token}
            {#if token.type == "ATag"}
                <ATag href={token.value.href}>{token.value.content}</ATag>
            {:else if token.type == "ColorTag" && token.value.content != ""}
                <ColorTag
                    background={token.value.background}
                    foreground={token.value.color}
                    >{token.value.content}</ColorTag
                >
            {:else if token.type == "RAW" && token.value.content != ""}
                <span>{token.value.content}</span>
            {/if}
        {/each}
    </div>

    {#each checkLinks as link}
        {#await getPreview(link) then preview}
            <LinkPreview
                on:message_formatted={() => {
                    dispatch("message_formatted");
                }}
                {preview}
                {link}
            ></LinkPreview>
        {/await}
    {/each}
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
    }
</style>
