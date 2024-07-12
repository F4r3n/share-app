<script lang="ts">
    import PlusSign from "../assets/plus-sign-svg.svelte";
    import { createEventDispatcher } from "svelte";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { config } from "./config";
    import CloseButton from "../assets/circle-close.svelte";

    const dispatch = createEventDispatcher();
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
                console.log(image);
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
        console.log(upload_id);
        return (config.getConfig().upload_image.url_get + upload_id) as string;
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
                if (e.key === "Enter") {
                    try {
                        for (let image of listImages) {
                            let url = await uploadImage(image);
                            messageToSend = messageToSend.replace(
                                image.name,
                                url,
                            );
                        }
                        dispatch("send_message", messageToSend);
                        listImages = [];
                        messageToSend = "";
                    } catch (e) {
                        console.log(e);
                    }
                }
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

    .row img {
        padding-right: 5px;
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
