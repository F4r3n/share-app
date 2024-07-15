<script>
    import LongArrow from "../assets/long-arrow.svelte";
    export let topic = "TEST";
    import {panelIsOpen} from "./discussStore"
    import { get } from "svelte/store";
</script>

<div
    class="flex flex-row bg-primary-500-400-token text-on-primary-token justify-between text-ellipsis max-w[100%] leading-5 items-center align-middle p-1"
>
    <div>{topic}</div>
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
    .arrow {
        transform: rotate(180deg);
        stroke: theme("colors.primary.50");
        stroke-width: 1px;
    }
</style>
