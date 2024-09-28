<script lang="ts">
    import PlusSign from "../assets/plus-sign-svg.svelte";
    import { createEventDispatcher } from "svelte";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { config } from "./config";
    import CloseButton from "../assets/circle-close.svelte";

    const dispatch = createEventDispatcher();
    type Navigation = "up" | "down";

    function clamp(num: number, min: number, max: number) {
        return num <= min ? min : num >= max ? max : num;
    }

    class MessageHistory {
        private messageHistory: string[] = [];
        private indexHistory = 0;
        private hasReachedStartHistory = true;
        append(inMessage: string) {
            this.messageHistory.push(inMessage);
            this.indexHistory = this.messageHistory.length - 1;
        }

        navigate(nav: Navigation): boolean {
            if (this.messageHistory.length == 0) return false;
            switch (nav) {
                case "up": {
                    this.indexHistory -= 1;
                    break;
                }
                case "down": {
                    this.indexHistory += 1;
                    break;
                }
            }
            let canContinue = true;
            if (this.indexHistory < 0 
            || this.indexHistory >= this.messageHistory.length)
            {
                canContinue = false;
            }
            if(this.indexHistory >= this.messageHistory.length)
            {
                this.hasReachedStartHistory = true;
            }
            else{
                this.hasReachedStartHistory = false;
            }
            this.indexHistory = clamp(
                this.indexHistory,
                0,
                this.messageHistory.length - 1,
            );
            return canContinue;
        }

        getMessage(): string {
            return this.messageHistory[this.indexHistory];
        }

        isStart(): boolean {
            return this.hasReachedStartHistory;
        }
    }
    let messageHistory = new MessageHistory();
    let currentMessage: string;
    let messageToSend: string;
    let input: HTMLInputElement;

    type Image = {
        base64: string;
        url: string;
        name: string;
    };

    let listImages: Image[] = [];

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

    async function uploadImage(image: Image): Promise<string> {
        let imageData: Uint8Array = await invoke("decode_base64", {
            message: image.base64,
        });
        let upload_id = await invoke("upload_image", {
            endpoint: `${config.getConfig().upload_image.url_post}`,
            imageBytes: imageData,
        });
        return (config.getConfig().upload_image.url_get + upload_id) as string;
    }

    async function manageKeyboardEvent(e: KeyboardEvent) {
        switch (e.key) {
            case "Enter": {
                try {
                    for (const image of listImages) {
                        messageToSend = messageToSend.replace(
                            image.name,
                            await uploadImage(image),
                        );
                    }
                    messageHistory.append(messageToSend);
                    dispatch("send_message", messageToSend);
                    listImages = [];
                    messageToSend = "";
                } catch (e) {
                    console.error(e);
                }
                break;
            }
            case "ArrowUp": {
                if (messageHistory.isStart()) {
                    currentMessage = messageToSend;
                }
                if (messageHistory.navigate("up")) {
                    messageToSend = messageHistory.getMessage();
                }
                break;
            }
            case "ArrowDown": {
                if (messageHistory.navigate("down")) {
                    messageToSend = messageHistory.getMessage();
                } 
                if (messageHistory.isStart()) {
                    messageToSend = currentMessage;
                }
                break;
            }
        }
    }
</script>

<main>
    <div class="flex flex-row">
        {#each listImages as image}
            <div class="pasted-image">
                <button
                    class="top close"
                    on:click={() => {
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

    <div class="main-input">
        <input
            class="flex-grow"
            type="text"
            bind:this={input}
            bind:value={messageToSend}
            on:keyup={async (e) => {
                manageKeyboardEvent(e);
            }}
        />
        <button
            class="ml-1 bg-primary-500-400-token text-on-primary-token btn-base"
            on:click={(event) => {
                dispatch("send_message", messageToSend);
                messageToSend = "";
            }}
        >
            <PlusSign width="15" height="15"></PlusSign>
        </button>
    </div>
</main>

<style>
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
