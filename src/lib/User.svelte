<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Writable } from "svelte/store";
    const dispatch = createEventDispatcher();
    export let channelName: string;
    export let isSelected: boolean;
    export let isSelectable: boolean;
    export let unread: Writable<boolean>;
</script>

{#if isSelectable}
    <button
        class="px-0 mx-0 text-left text-nowrap text-ellipsis select-none"
        class:channel-missing-messages={$unread && !isSelected}
        class:channel-selected={isSelected}
        on:click={() => {
            unread.set(false)
            dispatch("channel_changed", channelName);
        }}
    >
        {channelName}
    </button>
{:else}
    <span class="px-0 mx-0 font-bold text-left text-nowrap text-ellipsis select-none">{channelName}</span>
{/if}

<style>
    .channel-selected {
        background-color: theme("colors.tertiary.300");
        color: theme("colors.tertiary.900");
        transition: border-color 200ms ease-out;
    }

    .channel-missing-messages {
        background-color: theme("colors.tertiary.300");
        transition: border-color 200ms ease-out;
        color: theme("colors.warning.600");
    }
</style>
