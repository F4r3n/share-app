<script lang="ts">
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();
export let channelName : string;
export let isSelected : boolean;
export let isSelectable : boolean;
export let unread : boolean;
</script>
<div class="channel"
class:channel-selected={isSelected}
class:channel-missing-messages={unread}
class:channel-selectable={isSelectable}
on:click={()=>{dispatch("channel_changed", channelName)}}>
{channelName}
</div>
<style>
    .channel {
        padding-left: 5px;
        border: 2px solid;
        border-color: transparent;
        border-radius: 5px;
        color: var(--text-color-control)
    }

    .channel-selected {
        border-color: var(--text-color-control);
        transition: border-color 200ms ease-out;
    }

    .channel-selectable {
        cursor: pointer;
    }

    .channel-missing-messages {
        border-color: orange;
        transition: border-color 200ms ease-out;
    }
</style>