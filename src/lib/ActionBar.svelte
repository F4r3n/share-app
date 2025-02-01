<script lang="ts">
    import LongArrow from "../assets/long-arrow.svelte";
    import { panelIsOpen } from "./discussStore";

    let { topic = "TEST" } = $props();

    let showTopic = $state(false);
</script>

<div
    class="flex flex-row bg-primary-500-400-token text-on-primary-token justify-between max-w[100%] leading-5 items-center align-middle p-1"
>
    <button
        class={showTopic ? "topic-show" : "topic-hide"}
        onclick={() => {
            showTopic = !showTopic;
        }}
    >
        {topic}
    </button>
    <button
        onclick={() => {
            panelIsOpen.set(true);
        }}
        class="arrow block mt-auto mr-3"
    >
        <LongArrow></LongArrow>
    </button>
</div>

<style>
    button {
        all: unset;
    }

    button:hover {
        cursor: pointer;
    }

    .topic-hide {
        @apply truncate block max-h-5;
    }

    .topic-show {
        @apply block;
    }

    .arrow {
        transform: rotate(180deg);
        stroke: theme("colors.primary.50");
        stroke-width: 1px;
    }
</style>
