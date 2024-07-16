<script lang="ts">
    import ATag from "./ATag.svelte";
    import { createEventDispatcher } from "svelte";
    import type { MetaData } from "./metaData";
    import ImgTag from "./IMGTag.svelte";

    const dispatch = createEventDispatcher();

    export let preview: MetaData;
    export let link: string;
</script>

{#if preview.image_only}
    <ImgTag href={link}></ImgTag>
{:else if preview.image_url}
    <div class="image-container">
        {#if preview.site}
            <div class="subtitle">{preview.site}</div>
        {/if}
        <ATag href={link}><span>{preview.title}</span></ATag>
        {#if preview.description}
            <div class="description">{preview.description}</div>
        {/if}
        <img
            class="image"
            on:load={() => {
                dispatch("message_formatted");
            }}
            src={preview.image_url}
            alt={preview.title}
        />
    </div>
{/if}

<style>
    .description {
        font-size: small;
    }

    .subtitle {
        font-size: small;
    }

    .image-container {
        display: flex;
        flex-direction: column;

        border-style: solid;
        border-radius: 5px;
        border-width: 2px;
        border-color: var(--secondary-accent-color);
        padding: 10px;

        width: 95%;
        margin-top: 10px;
    }

    .image {
        margin-top: 5px;
        width: 100%;
    }
</style>
