<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api";
    import { onMount, onDestroy } from "svelte";
    import { afterUpdate } from "svelte";
    import { Jumper } from "svelte-loading-spinners";
    import MessageContent from "./MessageContent.svelte";
    import MessageInput from "./MessageInput.svelte";
    import type { Message } from "./channel";
    import { Channel } from "./channel";
    import User from "./User.svelte";
    import { createEventDispatcher } from "svelte";
    import { appWindow, UserAttentionType } from "@tauri-apps/api/window";

    const dispatch = createEventDispatcher();

    import type { MessageFromIRC } from "./MessageType";

    type User = {
        nick_name: string;
        user_mode: number;
    };

    export let nickName: string;
    export let channel: string;

    let listMessages: Map<string, Channel> = new Map<string, Channel>([
        [channel, new Channel()],
    ]);
    let topic: string = "";
    let channelNameSelected: string = channel ?? "";

    let discussSection: HTMLDivElement | null | undefined = null;
    let users: User[] = [];
    let updateScroll = true;
    let messagesUnreadChannel: Set<string> = new Set<string>();
    let isLoaded = false;

    function isScrollAtTheEnd(): boolean {
        if (discussSection == undefined) return true;

        const modifier = 100;
        return (
            discussSection.scrollTop + discussSection.offsetHeight + modifier >
            discussSection.scrollHeight
        );
    }

    function isMessageHighlight(inMessageContent: string): boolean {
        return inMessageContent.search(nickName) !== -1;
    }

    afterUpdate(() => {
        if (updateScroll) {
            refreshScroll();
        }

        updateScroll = false;
    });

    function refreshScroll() {
        if (discussSection == null) return;
        discussSection.scroll({
            top: discussSection.scrollHeight + 300,
            behavior: "smooth",
        });
    }

    async function read_messages() {
        await invoke("read_messages")
        updateUsers();
    }

    async function irc_received() {
        await listen("irc-recieved", async (event) => {
            let data: MessageFromIRC = event.payload as MessageFromIRC;
            let message: Message = {} as Message;
            message.content = data.content;
            message.nick_name = data.nick_name;
            message.highlight = false;
            let channelOrigin = data.channel;
            if (channelOrigin === "") {
                channelOrigin = channel;
            }
            if (channelOrigin === nickName) {
                //Get the origin as source
                channelOrigin = message.nick_name;
            }

            if (!listMessages.has(channelOrigin)) {
                listMessages.set(channelOrigin, new Channel());
            }
            let currentChannel: Channel | undefined =
                listMessages.get(channelOrigin);

            if (data.command === "PRIVMSG") {
                updateUsers();
                message.date = new Date();
                message.highlight = isMessageHighlight(message.content);
                currentChannel?.pushMessage(message);
                listMessages = listMessages;

                if (channelNameSelected !== channelOrigin) {
                    messagesUnreadChannel.add(channelOrigin);
                    messagesUnreadChannel = messagesUnreadChannel;
                }

                if (message.highlight) {
                    await appWindow.requestUserAttention(
                        UserAttentionType.Critical,
                    );
                } else {
                    await appWindow.requestUserAttention(
                        UserAttentionType.Informational,
                    );
                }
            } else if (data.command === "NOTICE") {
                message.date = new Date();
                message.highlight = isMessageHighlight(message.content);
                currentChannel?.pushMessage(message);
                listMessages = listMessages;

                if (channelNameSelected !== channelOrigin) {
                    messagesUnreadChannel.add(channelOrigin);
                    messagesUnreadChannel = messagesUnreadChannel;
                }
            } else if (data.command === "JOIN") {
                if (message.nick_name === nickName) {
                    currentChannel?.pushMessage({
                        nick_name: "",
                        content: `you joined`,
                        date: new Date(),
                    } as Message);
                } else {
                    currentChannel?.pushMessage({
                        nick_name: "",
                        content: `${message.nick_name} has joined`,
                        date: new Date(),
                    } as Message);
                }
                listMessages = listMessages;
                updateUsers();
            } else if (data.command === "QUIT") {
                let quitMessage = message.content.replace("Quit:", "");
                currentChannel?.pushMessage({
                    nick_name: "",
                    content: `${message.nick_name} has quit (${quitMessage})`,
                    date: new Date(),
                } as Message);
                listMessages = listMessages;
                updateUsers();
            } else if (data.command === "TOPIC") {
                currentChannel?.pushMessage({
                    nick_name: "",
                    content: `${message.nick_name} has changed the topic to: '${data.content}' `,
                    date: new Date(),
                } as Message);
                listMessages = listMessages;
                topic = data.content;
            } else if (data.command === "RESPONSE") {
                if (data.response?.kind === 353) {
                    //users
                    updateUsers();
                } else if (data.response?.kind === 332) {
                    topic = data.response?.content.at(-1) ?? "";
                } else if (data.response?.kind === 1) {
                    isLoaded = true;
                }
            } else if (data.command === "NICK") {
                nickName = data.content;
                updateUsers();
            } else if (data.command === "ERROR") {
                if (!isLoaded) {
                    setTimeout(() => {
                        console.log("disconnect");
                        invoke("disconnect", {
                            message: "",
                            shallSendMessage: false,
                            wrongIdentifier: true,
                        }).then(() => {
                            dispatch("connection_status", false);
                        });
                    }, 1000);
                }
            }
        });
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
    });

    onDestroy(async () => {
        // invoke('disconnect', {message:"Bye"});
    });

    async function updateUsers() {
        try
        {
            users = await getUsers() as User[];
        }
        catch(e) {}
    }

    function sendCurrentMessage(inMessageContent: string) {
        let message: Message;

        const isCommand: boolean = inMessageContent.at(0) == "/";
        message = {
            nick_name: nickName,
            content: inMessageContent,
            date: new Date(),
            highlight: false,
        };

        if (!isCommand) {
            if (!listMessages.has(channelNameSelected)) {
                listMessages.set(channelNameSelected, new Channel(message));
            } else {
                messagesSelected.push(message);
            }
            invoke("send_message", {
                message: inMessageContent,
                channel: channelNameSelected,
            })
                .then(() => {})
                .catch((e) => console.error(e));
            listMessages = listMessages;
        } else {
            let command = inMessageContent.split(" ");
            const commandName = command.at(0)?.substring(1);
            invoke("send_irc_command", {
                command: commandName,
                args: command.slice(1),
            }).then(() => {});
        }
    }

    function getUsers() {
        return new Promise((resolve, reject) => {
            invoke("get_users")
                .then((data) => {
                    resolve(data);
                })
                .catch((e) => {
                    reject(e);
                });
        });
    }

    $: isSameMessage = (id: number, nick_name: string): boolean => {
        return (
            id === 0 ||
            (id > 0 && messagesSelected[id - 1].nick_name !== nick_name)
        );
    };

    function getListMessages(
        inMessagesList: Map<string, Channel>,
        inChannel: string,
    ): Message[] {
        return inMessagesList.has(inChannel)
            ? inMessagesList!.get(inChannel)!.messages
            : ([] as Message[]);
    }

    $: messagesSelected = getListMessages(listMessages, channelNameSelected);

    function changeChannel(inChannel: string) {
        channelNameSelected = inChannel;
        messagesUnreadChannel.delete(inChannel);
        messagesUnreadChannel = messagesUnreadChannel;
    }
</script>

<main>
    {#if !isLoaded}
        <div class="loading" class:loading-hide={isLoaded}>
            <Jumper size="60" color="#845EC2" unit="px" duration="1s"></Jumper>
        </div>
    {:else}
        <div class="discuss-section">
            <div class="topic">{topic}</div>
            <div class="wrapper-messages" bind:this={discussSection}>
                <div class="messages">
                    {#each getListMessages(listMessages, channelNameSelected) as message, id}
                        <div
                            class="message"
                            style="--space:{isSameMessage(
                                id,
                                message.nick_name,
                            ) && id !== 0
                                ? '10px'
                                : '0px'}"
                        >
                            {#if isSameMessage(id, message.nick_name)}
                                <div class="title">
                                    <div
                                        class="username"
                                        class:username-me={message.nick_name ===
                                            nickName}
                                    >
                                        {message.nick_name}
                                    </div>

                                    <div class="date">
                                        {message.date.toLocaleTimeString()}
                                    </div>
                                </div>
                            {/if}
                            {#key channelNameSelected}
                                <div
                                    class="message-content"
                                    class:message-content-highlight={message.highlight}
                                    class:message-content-system={message.nick_name ===
                                        ""}
                                >
                                    <MessageContent
                                        on:message_formatted={() => {
                                            updateScroll = isScrollAtTheEnd();
                                            refreshScroll();
                                        }}
                                        content={message.content}
                                    ></MessageContent>
                                </div>
                            {/key}
                        </div>
                    {/each}
                </div>
            </div>

            <div class="wrapper-writter">
                <div class="write-section">
                    <MessageInput
                        on:send_message={(event) => {
                            sendCurrentMessage(event.detail);
                        }}
                    ></MessageInput>
                </div>
            </div>
        </div>

        <div class="list-users">
            <User
                on:channel_changed={() => {
                    changeChannel(channel);
                }}
                channelName={channel}
                isSelectable={true}
                unread={messagesUnreadChannel.has(channel)}
                isSelected={channelNameSelected === channel}
            ></User>

            {#each users as user}
                <div class="user-item">
                    <User
                        on:channel_changed={() => {
                            if (nickName !== user.nick_name) {
                                changeChannel(user.nick_name);
                                messagesSelected = messagesSelected;
                            }
                        }}
                        isSelectable={nickName !== user.nick_name}
                        unread={messagesUnreadChannel.has(user.nick_name)}
                        channelName={user.nick_name}
                        isSelected={channelNameSelected === user.nick_name}
                    ></User>
                </div>
            {/each}
        </div>

        <div class="widgets">
            
        </div>
    {/if}
</main>

<style>
    .topic {
        margin-left: 10px;
        display: inline-block;
        text-overflow: ellipsis;
        overflow: hidden;
        min-height: 20px;
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

    .user-item {
        padding-left: 5px;
        margin-top: 5px;
        padding-right: 20px;
    }

    main {
        max-height: 100%;
        max-width: 100%;
        position: relative;
        display: flex;
        flex-direction: row;
    }

    .list-users {
        display: flex;
        flex-direction: column;
        padding-top: 10px;
        padding-left: 5px;
        padding-right: 20px;
        min-width: 50px;
        background-color: var(--primary-accent-color);
        color: var(--background-color);
        width: 120px;
        max-width: 120px;
    }

    .discuss-section {
        flex-grow: 1;
        max-width: calc(100vw - 130px);

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
        max-width: calc(100vw - 180px);
        align-items: center;
        justify-content: center;
        margin: auto;
        margin-top: 7px;
        margin-bottom: 7px;
        margin-left: 10px;
    }

    .messages {
        display: flex;
        flex-direction: column;
        justify-content: left;
        justify-items: left;
    }

    .wrapper-messages {
        flex-grow: 1;
        min-height: 0;
        margin-left: 8px;
        overflow-y: scroll;
    }

    .message {
        margin-top: var(--space);
    }

    .message-content {
        display: flex;
        flex-direction: column;
    }

    .message-content-system {
        color: #b0a8b9;
    }

    .message-content-highlight {
        background-color: var(--highlight-color);
    }

    .date {
        font-size: x-small;
        color: #b0a8b9;
        margin-left: 10px;
    }

    .username {
        font-weight: bold;
        color: var(--secondary-accent-color);
    }

    .username-me {
        color: red;
    }

    .title {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    }

    .widgets
    {
        display: flex;
        flex-direction: row;
    }
</style>
