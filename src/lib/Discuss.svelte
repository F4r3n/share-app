<script lang="ts">
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'
import { onMount, onDestroy } from 'svelte';
import { afterUpdate } from 'svelte';
import { Jumper } from 'svelte-loading-spinners'
import PlusSign from './plus-sign-svg.svelte';
import MessageContent from "./MessageContent.svelte"
import type {Message} from "./channel";
import {Channel} from "./channel";
import User from './User.svelte';

type Response = {
    kind : number,
    content : string[];
}

type MessageFromIRC = {
    nick_name: string;
    content: string;
    command : string;
    channel : string;
    response?: Response
}


type User = {
    nick_name : string,
    user_mode : number
}

export let nickName : string;
export let channel : string;

let listMessages : Map<string, Channel> = new Map<string, Channel>([[
    channel, new Channel]]);
let messageToSend :string = ""
let topic :string = ""
let channelNameSelected :string = channel ?? "";

let discussSection: HTMLDivElement|null|undefined = null;
let users : User[] = []
let updateScroll = true;
let messagesUnreadChannel : Set<string> = new Set<string>();
let isLoaded = false;

function isScrollAtTheEnd() : boolean {
    if(discussSection == undefined) return true;

    const modifier = 100;
    return (discussSection.scrollTop + discussSection.offsetHeight + modifier) > discussSection.scrollHeight;
}

function isMessageHighlight(inMessageContent : string) : boolean{
    return inMessageContent.search(nickName) !== -1;
}

afterUpdate(() => {
    if(updateScroll)
    {
        refreshScroll();
    }

    updateScroll = false;
  });

function refreshScroll() {
    if(discussSection == null) return;
    discussSection.scroll({ top: discussSection.scrollHeight + 300, behavior: 'smooth' });
}

onMount(async () => {
    await listen('irc-recieved', (event) => {
    let data : MessageFromIRC = event.payload as MessageFromIRC

    let message : Message = {} as Message;
    message.content = data.content;
    message.nick_name = data.nick_name;
    message.highlight = false;
    let channelOrigin = data.channel;
    if(channelOrigin === "") {
        channelOrigin = channel;
    }
    if(channelOrigin === nickName) { //Get the origin as source
        channelOrigin = message.nick_name;
    }

    if(!listMessages.has(channelOrigin)) {
        listMessages.set(channelOrigin, new Channel)
    }
    let currentChannel : Channel|undefined = listMessages.get(channelOrigin);

    
    if(data.command === "PRIVMSG") {
        message.date = new Date();
        message.highlight = isMessageHighlight(message.content);
        currentChannel?.pushMessage(message)
        listMessages = listMessages;

        if(channelNameSelected !== channelOrigin) {
        messagesUnreadChannel.add(channelOrigin);
        messagesUnreadChannel = messagesUnreadChannel;
    }
    }
    else if(data.command === "JOIN") {
        if(message.nick_name === nickName)
        {
            currentChannel?.pushMessage({nick_name:"", content:`you joined`, date:new Date() } as Message)
        }
        else {
            currentChannel?.pushMessage({nick_name:"", content:`${message.nick_name} has joined`, date:new Date() } as Message)
        }
        listMessages = listMessages;
        updateUsers();
    }
    else if(data.command === "QUIT") {
        currentChannel?.pushMessage({nick_name:"", content:`${message.nick_name} has quit`, date:new Date() } as Message)
        listMessages = listMessages;
        updateUsers();
    }
    else if(data.command === "TOPIC") {
        message.date = new Date();
        currentChannel?.pushMessage(message)
        listMessages = listMessages;
        updateUsers();
    }
    else if(data.command === "RESPONSE"){
        if(data.response?.kind === 353) { //users
            updateUsers();
        }
        else if(data.response?.kind === 332) {
            topic = data.response?.content.at(-1) ?? ""
        }
    }
    })

    invoke('read_messages').finally(()=> {
        getUsers().then((data)=> {
            users = data as User[];
        })
        isLoaded = true;
    })

});

onDestroy(async ()=> {
    //invoke('disconnect', {message:"Bye"});
})

function updateUsers() {
    getUsers().then((data)=> {
        users = data as User[];
    })
}

function sendCurrentMessage(inMessageContent : string) {
    let message : Message;
    message = {nick_name:nickName, content:inMessageContent, date: new Date(), highlight:false}

    if(!listMessages.has(channelNameSelected)) {
        listMessages.set(channelNameSelected, new Channel(message))
    }
    else {
        messagesSelected.push(message);
    }
    invoke('send_message', {message:messageToSend, channel:channelNameSelected}).then(()=> {

    })
    listMessages = listMessages;
}

function getUsers() {
    return new Promise(resolve=> {
        invoke('get_users').then((data)=> {
        resolve(data)
    })
    })
}

$: isSameMessage = (id : number, nick_name : string) : boolean =>
 {return (id === 0 || (id > 0 && messagesSelected[id - 1].nick_name !== nick_name))}

 function getListMessages(inMessagesList : Map<string, Channel>, inChannel : string) : Message[]{
    return inMessagesList.has(inChannel) ? inMessagesList!.get(inChannel)!.messages 
: [] as Message[];
}

$: messagesSelected = getListMessages(listMessages, channelNameSelected)


function changeChannel(inChannel : string) {
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
        <div class="wrapper-messages" >
            <div class="messages" bind:this={discussSection}>
                {#each getListMessages(listMessages, channelNameSelected) as message, id}
                <div class="message" style="--space:{isSameMessage(id, message.nick_name) && id !== 0 ? "10px" : "0px"}">
                    {#if isSameMessage(id, message.nick_name)}
                    <div class="title">
                        <div class="username" class:username-me={message.nick_name === nickName}>
                            {message.nick_name}
                        </div>
        
                        <div class="date">
                            {message.date.toLocaleTimeString()}
                        </div>
                    </div>
                    {/if}
                    {#key channelNameSelected}
                    <div class="message-content"
                    class:message-content-highlight={message.highlight}
                    class:message-content-system={message.nick_name === ""}>
                        <MessageContent on:message-formatted={()=>{
                           updateScroll = isScrollAtTheEnd(); refreshScroll();}}
                           content={message.content}></MessageContent>
                    </div>
                    {/key}
                </div>
                {/each}
            </div>
        </div>
    
    
        <div class="wrapper-writter">
            <div class="write-section">
                <input type="text" bind:value={messageToSend} 
                on:keyup={(e)=>{
                    if(e.key==='Enter') {
                        sendCurrentMessage(messageToSend)
                        messageToSend = "";
                    }
                    }}>
                <button on:click={(event)=> {
                   sendCurrentMessage(messageToSend)
                   messageToSend = ""
    
                }}><PlusSign width=15 height=15></PlusSign></button>
            </div>
        </div>
    
    </div>

    <div class="list-users">
        <User on:channel_changed={()=>{changeChannel(channel)}}
        channelName={channel}
        isSelectable={true}
        unread={messagesUnreadChannel.has(channel)}
        isSelected={channelNameSelected === channel}></User>

        {#each users as user }
        <div class="user-item">
            <User on:channel_changed={()=>{
                if(nickName !== user.nick_name)
                {
                    changeChannel(user.nick_name)
                    messagesSelected = messagesSelected;
                }
            }}
            isSelectable={nickName !== user.nick_name}
            unread={messagesUnreadChannel.has(user.nick_name)}
            channelName={user.nick_name} 
            isSelected={channelNameSelected === user.nick_name}>
            </User>
        </div>

        {/each}
    </div>
    {/if}


</main>

<style>
    .topic {
        margin-left: 10px;
        display:inline-block;
        text-overflow: ellipsis;
        overflow: hidden;
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

    .list-users {
        display: flex;
        flex-direction: column;
        padding-top: 10px;
        padding-left: 5px;
        padding-right: 20px;
        min-width: 50px;
        background-color: var(--primary-accent-color);
        color: var(--background-color);
        width:120px;
        max-width: 120px;
    }

    .user-item {
        padding-left: 5px;
        margin-top: 5px;
        padding-right: 20px;
    }


    main {
        height: 100%;
        position: relative;
        display: flex;
        flex-direction: row;
        max-width: 100vw;
    }

    .discuss-section {
        flex: 1;
        height: 100%;
        position: relative;
        display: flex;
        flex-direction: column;
        max-width: calc(100vw - 120px);
    }

    input {
        width: 100%;
    }

    button {
        padding: 0px;
        border-radius: 4px;
        padding-left: 4px;
        padding-right: 4px;
        margin-left: 2px;
    }

    .write-section {
        display: flex;
        flex-direction: row;
        width: 100%;
    }

    .messages {
        display: flex;
        flex-direction: column;
        justify-content: left;
        justify-items: left;
        overflow-y: scroll;
        height: 100%;
        width: 100%;
    }

    .wrapper-messages {
        flex: 1;
        height: calc(100vh - 62px);
        margin-left: 8px;
    }

    .wrapper-writter {
        align-items: center;
        justify-content: center;
        margin: auto;
        width:95%;
        flex: 0 1 auto;
        height: 30px;
        margin-top: 7px;
        margin-bottom: 7px;
    }

    .message {
        margin-top: var(--space);
    }

    .message-content {
        display: flex;
        flex-direction: column;
    }

    .message-content-system {
        color:#B0A8B9;
    }

    .message-content-highlight {
        background-color: var(--secondary-accent-color-light);
    }

    .date {
        font-size: x-small;
        color: #B0A8B9;
        margin-left: 10px;
    }

    .username {
        font-weight: bold;
        color: var(--primary-accent-color)
    }

    .username-me {
        color: red;
    }

    .title {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    }
</style>