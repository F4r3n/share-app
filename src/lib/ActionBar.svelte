<script>
    import LongArrow from "../assets/long-arrow.svelte";
    export let topic = "TEST";
    import {panelIsOpen} from "./discussStore"

    let showTopic = false;
</script>

<div
    class="flex flex-row bg-primary-500-400-token text-on-primary-token justify-between max-w[100%] leading-5 items-center align-middle p-1"
>
    <button class={showTopic ? "topic-show" : "topic-hide"} on:click={()=>{showTopic=!showTopic;}}>
        {topic}
    </button>
    <button
        on:click={() => {
            panelIsOpen.set(true)
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
