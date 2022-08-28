<script lang="ts">
import ATag from "./ATag.svelte";

    type Token = {
        type: string,
        href: string,
        content: string
    }
export let tokens : Token[] = [];
</script>

{#each tokens as token}
    {#if token.type == 'ATag'}
    <ATag href={token.href}>{token.content}</ATag>
    {:else}
    <span>{token.content}</span>
    {/if}
{/each}
