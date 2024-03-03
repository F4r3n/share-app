<script lang="ts">
    import ATag from "./ATag.svelte";
    import ImgTag from "./IMGTag.svelte";
    import LinkPreview from "./LinkPreview.svelte";
    import { fetch } from "@tauri-apps/api/http";
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import { afterUpdate } from "svelte";
    import ColorTag from "./ColorTag.svelte";
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

    function isImage(url: string): Promise<boolean> {
        return fetch(url, { method: "HEAD" }).then((res) => {
            let value: boolean = false;
            if (res != null) {
                const content = res.headers["content-type"];
                if (content) {
                    value = content.startsWith("image");
                }
            }
            return value;
        });
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
                <ColorTag background={token.value.background} foreground={token.value.color}>{token.value.content}</ColorTag>
            {:else if token.type == "RAW" && token.value.content != ""}
                <span>{token.value.content}</span>
            {/if}
        {/each}
    </div>

    {#each checkLinks as link}
        {#await isImage(link) then value}
            {#if value}
                <ImgTag
                    on:message_formatted={() => {
                        dispatch("message_formatted");
                    }}
                    href={link}
                ></ImgTag>
            {:else}
                <LinkPreview
                    on:message_formatted={() => {
                        dispatch("message_formatted");
                    }}
                    href={link}
                ></LinkPreview>
            {/if}
        {/await}
    {/each}
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
    }
</style>
