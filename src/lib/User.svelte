<script>
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();
export let channelName;
export let isSelected;
export let unread;
</script>
<div class="channel"
class:channel-selected={isSelected}
class:channel-missing-messages={unread}
on:click={()=>{dispatch("channel_changed", channelName)}}>
{channelName}
</div>
<style>
    .channel {
        cursor: pointer;
        padding-left: 5px;
    }

    .channel-selected {
        border: 2px solid;
        border-color: var(--background-color);
        border-radius: 5px;
    }

    .channel-missing-messages {
        border: 2px solid;
        border-radius: 5px;
        border-color: orange !important;
    }
</style>