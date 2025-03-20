<script lang="ts">
    let {
        channelName,
        isSelected,
        isSelectable,
        unread,
        onChannelChanged,
    }: {
        channelName: string;
        isSelected: boolean;
        isSelectable: boolean;
        unread: boolean;
        onChannelChanged: (arg0: string) => void;
    } = $props();
</script>

{#if isSelectable}
    <button
        class="btn my-0.5 p-0 px-1 text-nowrap text-ellipsis select-none rounded-md h-fit w-fit channel"
        class:channel-missing-messages={unread && !isSelected}
        class:channel-selected={isSelected}
        onclick={() => {
            unread = false;
            onChannelChanged(channelName);
        }}
    >
        {channelName}
    </button>
{:else}
    <span
        class="px-2 mx-0 text-left text-nowrap text-ellipsis select-none"
        >{channelName}</span
    >
{/if}

<style>
    .channel {
        border-width: 2px;
    }

    .channel-selected {
        background-color: var(--color-primary-300-700);
        color: var(--color-primary-contrast-300-700);
        transition: border-color 200ms ease-out;
    }

    .channel-missing-messages {
        transition: border-color 200ms ease-out;
        background-color: var(--color-warning-300-700);
        color: var(--color-warning-contrast-300-700);
    }
</style>
