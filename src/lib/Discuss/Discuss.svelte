<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";
    import { Jumper } from "svelte-loading-spinners";
    import MessageContent from "../MessageRenderer/MessageContent.svelte";
    import MessageInput from "../MessageRenderer/MessageInput.svelte";
    import type { Message } from "./channel";
    import User from "./User.svelte";
    import { Window, UserAttentionType } from "@tauri-apps/api/window";
    import { messagesManager } from "./MessagesManager";
    import type { MessageFromIRC } from "../MessageRenderer/MessageType";
    import Arrow from "../../assets/arrow.svelte";
    import ActionBar from "./ActionBar.svelte";
    import { panelIsOpen } from "../discussStore";
    import { get } from "svelte/store";
    import { SvelteMap } from "svelte/reactivity";
    import Settings from "../Settings/Settings.svelte";
    type UserType = {
        nick_name: string;
        user_mode: number;
    };

    let {
        nickName,
        channel,
        onConnectionStatus,
    }: {
        nickName: string;
        channel: string;
        onConnectionStatus: ({}) => void;
    } = $props();

    class ScrollBehaviorManager {
        public followEnd = true;
        private _hasReachedEnd: boolean = true;
        public scroll_behaviour: ScrollBehavior = "smooth";

        constructor() {}
        updateScrollPosition(discussSection: HTMLDivElement) {
            this._hasReachedEnd =
                discussSection.offsetHeight + discussSection.scrollTop >=
                discussSection.scrollHeight - 2;
        }

        isAtTheEnd(): boolean {
            return this._hasReachedEnd;
        }

        public updateScroll(inHTML: HTMLDivElement | null) {
            if (inHTML && this.followEnd) {
                this.refreshScroll(inHTML);
            }
        }

        public refreshScroll(inHTML: HTMLDivElement | null) {
            if (!inHTML) return;

            inHTML.scroll({
                top: inHTML.scrollHeight + 300,
                behavior: this.scroll_behaviour,
            });
        }
    }
    let scrollBehaviourManager = new ScrollBehaviorManager();

    class ChattManager {
        public isConnected = $state(true);
        public isUnread: boolean = $state(false);
        constructor(name: string) {}

        public pushMessage() {
            this.isUnread = true;
        }

        public setAsSelected(isSelected: boolean) {
            if (isSelected) {
                this.isUnread = false;
                scrollBehaviourManager.scroll_behaviour = "smooth";
            } else {
                this.isUnread = false;
                scrollBehaviourManager.scroll_behaviour = "instant";
            }
        }
    }

    function pushMessage(inChannel: string) {
        _chatts.get(inChannel)?.pushMessage();
    }

    function getChat(inChannel: string): ChattManager {
        if (!_chatts.has(inChannel)) {
            _chatts.set(inChannel, new ChattManager(inChannel));
        }

        let chatt = _chatts.get(inChannel);
        return chatt ? chatt : new ChattManager(inChannel);
    }

    let topic: string = $state("");
    let channelNameSelected: string = $state(channel ?? "");

    let discussSection: HTMLDivElement | null = $state(null);

    let _chatts: SvelteMap<string, ChattManager> = new SvelteMap<
        string,
        ChattManager
    >([[channel, new ChattManager(channel)]]);

    let isLoaded = $state(true);
    let irc_received_unsubscribe = () => {};

    function isMessageHighlight(inMessageContent: string): boolean {
        return inMessageContent.search(nickName) !== -1;
    }

    async function read_messages() {
        try {
            console.log("Read messages");
            await invoke("read_messages");
        } catch (e) {
            console.error(e);
        }
        updateUsers();
    }

    async function irc_received() {
        irc_received_unsubscribe = await listen(
            "irc-recieved",
            async (event) => {
                let data: MessageFromIRC = event.payload as MessageFromIRC;
                let message: Message = {} as Message;
                message.content = data.content;
                message.nick_name = data.nick_name;
                message.highlight = false;
                let channelOrigin = data.channel;
                if (channelOrigin === "") {
                    channelOrigin = channel;
                }
                //Get the origin as source
                if (channelOrigin === nickName) {
                    channelOrigin = message.nick_name;
                }
                if (data.command !== "PRIVMSG") updateUsers();
                isLoaded = true;
                if (data.command === "PRIVMSG") {
                    updateUsers();
                    message.date = new Date();
                    message.highlight = isMessageHighlight(message.content);
                    messagesManager.putMessageInList(message, channelOrigin);
                    pushMessage(channelOrigin);

                    if (message.highlight) {
                        await Window.getCurrent().requestUserAttention(
                            UserAttentionType.Critical,
                        );
                    } else {
                        await Window.getCurrent().requestUserAttention(
                            UserAttentionType.Informational,
                        );
                    }
                } else if (data.command === "PING") {
                    isLoaded = true;
                } else if (data.command === "PONG") {
                    isLoaded = true;
                } else if (data.command === "NOTICE") {
                    message.date = new Date();
                    message.highlight = isMessageHighlight(message.content);
                    messagesManager.putMessageInList(message, channelOrigin);

                    pushMessage(channelOrigin);
                } else if (data.command === "JOIN") {
                    messagesManager.putMessageInList(
                        {
                            nick_name: "",
                            content:
                                message.nick_name === nickName
                                    ? `you joined`
                                    : `${message.nick_name} has joined`,
                            date: new Date(),
                        } as Message,
                        channelOrigin,
                    );
                } else if (data.command === "QUIT") {
                    let quitMessage = message.content.replace("Quit:", "");
                    messagesManager.putMessageInList(
                        {
                            nick_name: "",
                            content: `${message.nick_name} has quit (${quitMessage})`,
                            date: new Date(),
                        } as Message,
                        channelOrigin,
                    );
                } else if (data.command === "TOPIC") {
                    messagesManager.putMessageInList(
                        {
                            nick_name: "",
                            content: `${message.nick_name} has changed the topic to: '${data.content}' `,
                            date: new Date(),
                        } as Message,
                        channelOrigin,
                    );
                    topic = data.content;
                } else if (data.command === "RESPONSE") {
                    if (data.response?.kind === 353) {
                        //users
                    } else if (data.response?.kind === 332) {
                        topic = data.response?.content.at(-1) ?? "";
                    } else if (data.response?.kind === 1) {
                        isLoaded = true;
                    }
                } else if (data.command === "NICK") {
                    if (nickName == data.nick_name) {
                        nickName = data.content;
                    } 
                    updateUsers();
                    
                } else if (data.command === "ERROR") {
                    message.date = new Date();
                    message.highlight = isMessageHighlight(message.content);
                    messagesManager.putMessageInList(message, channelOrigin);
                    pushMessage(channelOrigin);
                } else if (data.command === "INTERNAL_ERROR") {
                    if (isLoaded) {
                        console.log("disconnect");
                        invoke("disconnect", {
                            message: data.content,
                            shallSendMessage: false,
                        }).then(() => {
                            onConnectionStatus({
                                result: false,
                                message: data.content,
                            });
                        });
                    }
                }
            },
        );
    }

    async function irc_event() {
        type Event = {
            payload: {
                kind: string;
            };
        };
        await listen("irc-event", (event: Event) => {
            if (event.payload.kind === "Quit") {
            } else if (event.payload.kind === "ErrorReadingMessages") {
                read_messages();
            }
        });
    }

    onMount(async () => {
        irc_received();
        read_messages();
        irc_event();
        discussSection.addEventListener("wheel", function (e) {
            if (e.deltaY < 0) {
                scrollBehaviourManager.followEnd = false;
            } else {
                scrollBehaviourManager.followEnd =
                    scrollBehaviourManager.isAtTheEnd();
            }
        });
        panelIsOpen.set(currentModeSize == Width_Mode.DESKTOP);
    });

    let screen_height = window.innerHeight;
    let screen_width = $state(window.innerWidth);
    const mobile_width = 500;
    let panelOpeningPercentage = $state(0);
    let panelOpeningPercentageToDisplay = $state(0);
    const maxOpeningUserDistance = 80;

    enum Width_Mode {
        PHONE,
        DESKTOP,
    }

    let currentModeSize =
        screen_width < mobile_width ? Width_Mode.PHONE : Width_Mode.DESKTOP;
    let panel_mode = $derived(
        screen_width < mobile_width ? Width_Mode.PHONE : Width_Mode.DESKTOP,
    );
    function onResize() {
        if (currentModeSize != panel_mode) {
            currentModeSize = panel_mode;
            if (get(panelIsOpen) && currentModeSize == Width_Mode.PHONE) {
                panelIsOpen.set(false);
            }
        }
        if (screen_height != window.innerHeight) {
            screen_height = window.innerHeight;
            scrollBehaviourManager.refreshScroll(discussSection);
        }
    }

    onDestroy(async () => {
        irc_received_unsubscribe();
    });

    async function updateUsers() {
        try {
            for (let [channel, info] of _chatts.entries()) {
                info.isConnected = false;
            }
            getChat(channel).isConnected = true;

            for (let user of (await invoke("get_users")) as UserType[]) {
                getChat(user.nick_name).isConnected = true;
            }
        } catch (e) {
            console.error(e);
        }
    }

    function sendCurrentMessage(inMessageContent: string) {
        if (inMessageContent === "") return;
        const isCommand: boolean = inMessageContent.at(0) == "/";
        let message: Message = {
            nick_name: nickName,
            content: inMessageContent,
            date: new Date(),
            highlight: false,
        };
        messagesManager.putMessageInList(message, channelNameSelected);

        if (!isCommand) {
            invoke("send_message", {
                message: inMessageContent,
                channel: channelNameSelected,
            })
                .then(() => {})
                .catch((e) => console.error(e));
        } else {
            invoke("send_irc_command", {
                message: inMessageContent.slice(1),
            }).then(() => {});
        }
    }

    function changeChannel(inChannel: string) {
        getChat(channelNameSelected).setAsSelected(false);
        getChat(inChannel).setAsSelected(true);

        channelNameSelected = inChannel;
    }

    let listMessages = $derived(
        messagesManager.getChannel(channelNameSelected).getListMessages(),
    );

    const unsubscribe = panelIsOpen.subscribe((value) => {
        if (value == true) panelOpeningPercentageToDisplay = 100;
        else panelOpeningPercentageToDisplay = 0;
    });
    let open = $derived(
        panel_mode == Width_Mode.DESKTOP ||
            $panelIsOpen ||
            panelOpeningPercentageToDisplay > 0,
    );

    let isSettingsOpened = $state(false);
</script>

<svelte:window on:resize={onResize} />
<main class="flex flex-row" bind:clientWidth={screen_width}>
    {#if !isLoaded}
        <div class="loading" class:loading-hide={isLoaded}>
            <Jumper size="60" color="#845EC2" unit="px" duration="1s"></Jumper>
        </div>
    {:else}
        <div class="discuss-section flex-grow-1 min-w-0">
            <ActionBar {topic}></ActionBar>
            <div
                class="flex-grow overflow-y-auto"
                bind:this={discussSection}
                onscrollend={() => {
                    if (discussSection) {
                        scrollBehaviourManager.updateScrollPosition(
                            discussSection,
                        );
                    }
                }}
            >
                <div class="messages">
                    {#each $listMessages as message, id}
                        <div class="message">
                            <div class="title">
                                <div
                                    class="username"
                                    class:username-me={message.nick_name ===
                                        nickName}
                                >
                                    {message.nick_name}
                                </div>
                                <div
                                    class="text-primary-300-400-token text-xs ml-10"
                                >
                                    {message.date.toLocaleTimeString()}
                                </div>
                            </div>

                            <div
                                class="flex flex-col"
                                class:message-content-highlight={message.highlight}
                                class:message-content-system={message.nick_name ===
                                    ""}
                            >
                                <MessageContent
                                    onMessageFormatted={() => {
                                        scrollBehaviourManager.updateScroll(
                                            discussSection,
                                        );
                                    }}
                                    content={message.content}
                                ></MessageContent>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>

            <div class="wrapper-writter">
                <div class="write-section">
                    <MessageInput
                        onSendMessage={(event) => {
                            sendCurrentMessage(event);
                        }}
                    ></MessageInput>
                </div>
            </div>
        </div>
        <div
            class:panel-open-mobile-transition={panelOpeningPercentageToDisplay !=
                panelOpeningPercentage}
            class:panel-opening-mobile={open}
            class={panel_mode == Width_Mode.PHONE
                ? "panel-open-mobile"
                : "list-users-desktop"}
            style="--opening_width:{panelOpeningPercentageToDisplay *
                (maxOpeningUserDistance / 100)}%;"
        >
            <div class="list-users-desktop">
                {#each _chatts.entries() as [channel_name, info]}
                    {#if info.isConnected}
                        <User
                            onChannelChanged={() => {
                                if (nickName !== channel_name) {
                                    changeChannel(channel_name);
                                }
                            }}
                            isSelectable={nickName !== channel_name}
                            unread={info.isUnread}
                            channelName={channel_name}
                            isSelected={channelNameSelected === channel_name}
                        ></User>
                    {/if}
                {/each}
                {#if panel_mode == Width_Mode.PHONE}
                    <button
                        type="button"
                        class="btn"
                        onclick={() => {
                            panelIsOpen.set(false);

                            panelOpeningPercentageToDisplay = 0;
                            panelOpeningPercentage = 1;
                        }}
                    >
                        <Arrow></Arrow>
                    </button>
                {/if}
            </div>

            <Settings {isSettingsOpened}></Settings>
        </div>
    {/if}
</main>

<style>
    .list-users-desktop {
        @apply flex flex-col preset-filled-secondary-300-700 p-1 justify-between;
    }

    .panel-open-mobile {
        display: flex;
        flex-direction: column;
        position: fixed; /* Stay in place */
        z-index: 1; /* Stay on top */
        top: 0;
        right: 0;
        height: 100%;
        width: var(--opening_width);
        @apply preset-filled-secondary-300-700 justify-between;
    }

    .panel-opening-mobile {
        -webkit-box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
        box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
        @apply p-1;
    }

    .panel-open-mobile-transition {
        transition: width 0.1s;
    }

    .loading {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        visibility: visible;
    }

    .loading-hide {
        visibility: hidden;
    }

    .discuss-section {
        flex-grow: 1;
        height: 100vh;
        position: relative;
        display: flex;
        flex-direction: column;
    }

    .write-section {
        display: flex;
        flex-direction: row;
    }

    .wrapper-writter {
        width: 100%;
        align-items: center;
        justify-content: center;
        margin-top: 7px;
        padding-left: 5px;
        padding-right: 5px;
        margin-bottom: calc(7px + env(keyboard-inset-height));
    }

    .messages {
        display: flex;
        flex-direction: column;
        justify-content: left;
        justify-items: left;
    }

    .message {
        margin-top: var(--space);

        @apply preset-filled-secondary-100-900;

        @apply ml-2 mr-2 mt-1;
        @apply rounded-xl;
        @apply p-1;
    }

    .message-content-system {
        color: theme("colors.primary.400");
    }

    .message-content-highlight {
        background-color: theme("accentColor.primary.700");
        color: theme("accentColor.primary.300");
    }

    .username {
        font-weight: bold;
        color: theme("colors.secondary.700");
    }

    .username-me {
        color: theme("colors.primary.500");
    }

    .title {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    }
</style>
