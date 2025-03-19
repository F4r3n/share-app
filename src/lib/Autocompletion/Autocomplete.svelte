<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { AutocompletionItem } from "./type";
    import { config } from "../config";
    import { onMount } from "svelte";

    let {
        options,
        input,
        triggers,
        onSelection,
    }: {
        options: AutocompletionItem[];
        input: string;
        triggers: string[];
        onSelection: (arg0: AutocompletionItem) => void;
    } = $props();

    let itemID: number = $state(0);
    let listedOptions = $derived(filterOptions(options, input));
    let triggerStart = "";

    onMount(() => {
    });

    $effect(() => {
    });

    function filterOptions(
        inOptions: AutocompletionItem[],
        word: string,
    ): AutocompletionItem[] {
        let w = word;
        for (const trigger of triggers) {
            if (w.startsWith(trigger)) {
                w = w.replace(trigger, "");
                triggerStart = trigger;
            }
        }

        // Create a local copy of options
        let _options = [...inOptions];

        // Filter options
        _options = _options.filter((option) => {
            // Format the input search value
            const inputFormatted = String(w).toLowerCase().trim();
            // Format the option
            let optionFormatted = JSON.stringify([option.label]).toLowerCase();

            // Check Match
            if (optionFormatted.includes(inputFormatted)) return true;

            return false;
        });

        return _options;
    }

    export function navigate(event: KeyboardEvent) {
        switch (event.key) {
            case "Tab":
                select(listedOptions[itemID]);
                break;
            case "Enter":
                select(listedOptions[itemID]);
                break;
            case "ArrowUp":
                itemID = Math.max(0, itemID - 1);

                break;
            case "ArrowDown":
                itemID = Math.min(listedOptions.length - 1, itemID + 1);
                break;
        }
        fetchHelp(itemID);
    }

    function fetchHelp(inID: number) {
        if (listedOptions[inID] && !listedOptions[inID].help) {
            try {
                invoke("get_completion_help", {
                    endpoint: config.getCompletionConfig()?.url,
                    token: config.getCompletionConfig()?.token,
                    word: listedOptions[inID].label,
                }).then((value) => {
                    listedOptions[inID].help = value as string;
                });
            } catch (error) {}
        }
    }

    function select(label: AutocompletionItem) {
        if (label) {
            const autocompleteWord: AutocompletionItem = {
                label: triggerStart + label.label,
                help: label.help,
            };
            onSelection(autocompleteWord);
        }
    }

    function scrollIntoView(node: any, scroll: any) {
        function update(scroll: any) {
            if (scroll) node.scrollIntoView({ behavior: "smooth" });
        }

        update(scroll);
        return { update };
    }
</script>

<div>
    <nav>
        <ul>
            {#each listedOptions as item, id}
                <li
                    onmouseenter={() => {
                        itemID = id;
                        fetchHelp(id);
                    }}
                    class="hover:bg-primary-300 pl-1 rounded-sm content-between"
                    class:hoverItem={id == itemID}
                >
                    <button
                        onclick={() => {
                            select(item);
                        }}
                        use:scrollIntoView={id == itemID}
                        onfocus={() => {}}
                    >
                        <div class="font-bold">{item.label}</div>

                        {#if item.help}
                            <div class="italic">{item.help}</div>
                        {/if}
                    </button>
                </li>
            {/each}
        </ul>
    </nav>
</div>

<style>
    button {
        all: unset;
    }

    nav {
        @apply bg-tertiary-50;
        border-radius: 9px;
    }

    .hoverItem {
        @apply bg-primary-300;
    }
</style>
