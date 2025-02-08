<script lang="ts">
    import PlusSign from "../../assets/plus-sign-svg.svelte";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { config, type UploadImageConfig } from "../config";
    import CloseButton from "../../assets/circle-close.svelte";
    import { MessageHistory } from "./MessageHistory";
    import type { AutocompletionItem } from ".././Autocompletion/type";
    import Autocomplete from "../Autocompletion/Autocomplete.svelte";


    let { onSendMessage }: { onSendMessage: (arg0: string) => void } = $props();

    let messageHistory = new MessageHistory();
    let currentMessage: string;
    let messageToSend: string = $state("");
    let input: HTMLInputElement;
    let displayAutoComplete: boolean = $state(false);
    let completionList: object[] = [];
    let autocomplete: Autocomplete = $state(null);
    type Image = {
        base64: string;
        url: string;
        name: string;
    };

    function getListTriggers(): string[] {
        let completionConfig = config.getCompletionConfig();
        if (completionConfig) {
            return completionConfig.triggers.split(" ");
        }
        return [];
    }

    let listImages: Image[] = $state([]);
    let listWords: AutocompletionItem[] = $state([]);

    let autocompleteMessage = $state(messageToSend?.split(" ").at(-1));
    $effect(() => {
        autocompleteMessage = messageToSend?.split(" ").at(-1);
    });

    async function removeImage(event: Event) {
        invoke("get_image_clipboard")
            .then((base64) => {
                let image: Image = {
                    base64: base64 as string,
                    url: "",
                    name: "#_image_" + listImages.length,
                };
                listImages.push(image);
                listImages = listImages;

                messageToSend += image.name;
            })
            .catch((e) => {
                console.error(e);
            });
    }

    onMount(async () => {
        input.focus();
        addEventListener("paste", removeImage);
    });

    onDestroy(() => {
        removeEventListener("paste", removeImage);
    });

    async function uploadImage(
        image: Image,
        uploadConfig: UploadImageConfig,
    ): Promise<string> {
        let imageData: Uint8Array = await invoke("decode_base64", {
            message: image.base64,
        });
        let upload_id = await invoke("upload_image", {
            endpoint: `${uploadConfig.url_post}`,
            imageBytes: imageData,
        });
        return (uploadConfig.url_get + upload_id) as string;
    }

    async function manageKeyboardEventDown(e: KeyboardEvent) {
        switch (e.key) {
            case "ArrowUp": {
                if (!displayAutoComplete) {
                    if (messageHistory.isStart()) {
                        currentMessage = messageToSend;
                    }
                    if (messageHistory.navigate("up")) {
                        messageToSend = messageHistory.getMessage();
                    }
                } else {
                    autocomplete.navigate(e);
                    e.preventDefault();
                }
                break;
            }
            case "ArrowDown": {
                if (!displayAutoComplete) {
                    if (messageHistory.navigate("down")) {
                        messageToSend = messageHistory.getMessage();
                    }
                    if (messageHistory.isStart()) {
                        messageToSend = currentMessage;
                    }
                } else {
                    autocomplete.navigate(e);
                    e.preventDefault();
                }

                break;
            }
            case "Tab": {
                e.preventDefault();

                if (displayAutoComplete && autocomplete) {
                    autocomplete.navigate(e);
                }

                if (completionList.length == 0) {
                    try {
                        let result = await invoke("get_completion_list", {
                            endpoint: config.getCompletionConfig()?.url,
                            token: config.getCompletionConfig()?.token,
                            word: "",
                        });
                        completionList = result as object[];
                    } catch (e) {}
                }

                if (autocompleteMessage) {
                    if (
                        getListTriggers().some((trigger) =>
                            autocompleteMessage.startsWith(trigger),
                        )
                    )
                        listWords = completionList as AutocompletionItem[];
                    else if (autocompleteMessage.length > 0) {
                        listWords = ((await invoke("get_users")) as any[]).map(
                            (value) => {
                                return {
                                    label: value.nick_name,
                                } as AutocompletionItem;
                            },
                        );
                    } else {
                        listWords = [];
                    }
                    displayAutoComplete = true;
                }
                break;
            }
        }
    }

    async function manageKeyboardEventUp(e: KeyboardEvent) {
        switch (e.key) {
            case "Escape": {
                displayAutoComplete = false;
                break;
            }
            case "Enter": {
                if (!displayAutoComplete) {
                    try {
                        const upload_config = config.getUploadImageConfig();
                        if (upload_config) {
                            for (const image of listImages) {
                                messageToSend = messageToSend.replace(
                                    image.name,
                                    await uploadImage(image, upload_config),
                                );
                            }
                        }

                        messageHistory.append(messageToSend);
                        onSendMessage(messageToSend);
                        listImages = [];
                        messageToSend = "";
                    } catch (e) {
                        console.error(e);
                    }
                } else {
                    autocomplete.navigate(e);
                }

                displayAutoComplete = false;

                break;
            }
            case "Backspace": {
                if (messageToSend.length == 0) displayAutoComplete = false;
                break;
            }
        }
    }

    function onCompletionSelection(event: AutocompletionItem): void {
        let arr = messageToSend?.split(" ");
        arr[arr.length - 1] = event.label;
        messageToSend = arr.join(" ");

        displayAutoComplete = false;
    }
</script>

<main class="relative">
    <div class="flex flex-row">
        {#each listImages as image}
            <div class="pasted-image">
                <button
                    class="top close"
                    onclick={() => {
                        messageToSend = messageToSend.replace(image.name, "");
                        listImages = listImages.filter((i) => {
                            i.name !== image.name;
                        });
                    }}
                >
                    <CloseButton width="20" height="20"></CloseButton>
                </button>
                <img
                    class="pasted-image"
                    src={"data:image/png;base64," + image.base64}
                    alt="current-paster {image.name}"
                />
            </div>
        {/each}
    </div>
    {#if displayAutoComplete}
        <div
            class="absolute card w-full max-w-sm max-h-48 p-3 overflow-y-auto z-40 -translate-y-full"
            tabindex="-1"
        >
            <Autocomplete
                bind:this={autocomplete}
                input={autocompleteMessage}
                options={listWords}
                triggers={getListTriggers()}
                onSelection={(item) => {
                    onCompletionSelection(item);
                }}
            />
        </div>
    {/if}

    <div class="main-input">
        <input
            class="flex-grow"
            type="text"
            bind:this={input}
            bind:value={messageToSend}
            onkeydown={async (e) => {
                manageKeyboardEventDown(e);
            }}
            onkeyup={async (e) => {
                manageKeyboardEventUp(e);
            }}
        />
        <button
            class="ml-1"
            onclick={(event) => {
                onSendMessage(messageToSend);
                messageToSend = "";
            }}
        >
            <PlusSign width="15" height="15"></PlusSign>
        </button>
    </div>
</main>

<style>
    button {
        @apply btn;
        @apply preset-filled-primary-800-200;
        @apply px-4;
    }
    .pasted-image {
        width: 150px;
        max-height: 150px;
        position: relative;
    }

    .top {
        position: absolute;
        left: calc(100% - 25px);
        top: 0px;
        z-index: 10;
    }

    .close {
        color: var(--text-color-control);
    }

    .close:hover {
        color: rgb(255, 62, 62);
    }

    main {
        display: flex;
        flex-direction: column;

        width: 100%;
        margin: auto;
    }

    .main-input {
        width: 100%;
        display: flex;
        flex-direction: row;
    }
</style>
