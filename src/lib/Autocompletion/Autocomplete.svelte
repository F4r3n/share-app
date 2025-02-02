<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { AutocompletionItem } from "./type";
    import { config } from "../config";

    let {options, input = $bindable(""), triggers, onSelection}:
    {
        options : AutocompletionItem[],
        input : string,
        triggers : string[]
        onSelection : (arg0 : AutocompletionItem)=>void
    } = $props();

    let itemID: number = 0;
    let listedOptions = $derived(filterOptions(options, input));
    let triggerStart = "";

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
            if (optionFormatted.includes(inputFormatted)) return option;
        });
        if (listedOptions) itemID = listedOptions.length - 1;
        else itemID = 0;
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
            invoke("get_completion_help", {
                endpoint: config.getCompletionConfig()?.url,
                token: config.getCompletionConfig()?.token,
                word: listedOptions[inID].label,
            }).then((value) => {
                listedOptions[inID].help = value as string;
            });
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
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                <li
                    onclick={() => {
                        select(item);
                    }}
                    class="hover:bg-primary-300 pl-1 rounded content-between"
                    class:hoverItem={id == itemID}
                    use:scrollIntoView={id == itemID}
                    onmouseenter={() => {
                        fetchHelp(id);
                    }}
                >
                    <div class="font-bold">{item.label}</div>

                    {#if item.help}
                        <div class="italic">{item.help}</div>
                    {/if}
                </li>
            {/each}
        </ul>
    </nav>
</div>

<style>
    .hoverItem {
        @apply bg-primary-300;
    }
</style>
