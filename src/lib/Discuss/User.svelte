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
        class="btn m-0 p-0 px-2 text-nowrap text-ellipsis select-none rounded-md h-fit w-fit"
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
        class="px-2 mx-0 font-bold text-left text-nowrap text-ellipsis select-none"
        >{channelName}</span
    >
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
