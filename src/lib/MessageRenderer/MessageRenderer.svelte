<script lang="ts">
    import ATag from "./ATag.svelte";
    import LinkPreview from "./LinkPreview.svelte";
    import { onMount } from "svelte";
    import ColorTag from "./ColorTag.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { MetaData } from "./metaData";
    import { tick } from "svelte";
    let {
        tokens = [],
        onMessageFormatted,
    }: { tokens: Token[]; onMessageFormatted: () => void } = $props();
    type Token = {
        type: string;
        value: any;
    };
    let checkLinks: string[] = $state([]);

    onMount(async () => {
        for (let token of tokens) {
            if (token.type === "ATag") {
                checkLinks.push(token.value.href);
            }
        }
        checkLinks = checkLinks;
    });

    $effect.pre(() => {
        console.log("the component is about to update");
        tick().then(() => {
            console.log("the component just updated");
            onMessageFormatted();
        });
    });

    function getPreview(inURL: string): Promise<MetaData> {
        return invoke("get_url_preview", { endpoint: inURL });
    }
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
                onMessageFormatted={() => {
                    onMessageFormatted();
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
