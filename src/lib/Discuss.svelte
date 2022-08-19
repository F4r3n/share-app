<script lang="ts">
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'
import { onMount, onDestroy } from 'svelte';
import { afterUpdate } from 'svelte';
import { Jumper } from 'svelte-loading-spinners'
import PlusSign from './plusSign.svelte';

type Response = {
    kind : number,
    content : string
}

type MessageFromIRC = {
    nick_name: string;
    content: string;
    command : string,
    response?: Response
}

type Message = {
    nick_name: string;
    content: string;
    link:string;
    date: Date;
}

type User = {
    nick_name : string,
    user_mode : number
}
let listMessages : Message[] = [];
let messageToSend = ""

export let nickName : string;


let discussSection = null;
let users : User[] = []
let updateScroll = true;

var urlRegex = /(((https?:\/\/)|(www\.))[^\s]+)/g;
function parseMessage(inMessage : string)
{
    console.log(inMessage)
    getLinkPreview(
        inMessage
).then((data) => console.debug(data));

    return "";
}

let isLoaded = false;

function isScrollAtTheEnd() {
    if(discussSection == null) return;

    const modifier = 30;
    //console.log(currentScroll + modifier, documentHeight)
    return discussSection.scrollTop + discussSection.offsetHeight + modifier > discussSection.scrollHeight;
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

    discussSection.scroll({ top: discussSection.scrollHeight, behavior: 'smooth' });
}

onMount(async () => {
    isLoaded = true;
    await listen('irc-recieved', (event) => {
    let data : MessageFromIRC = event.payload as MessageFromIRC

        
        let message : Message = {} as Message;
        message.content = data.content;
        message.link = ""
        message.nick_name = data.nick_name;
        if(data.command === "PRIVMSG") {
            message.date = new Date();
            message.link = parseMessage(message.content)
            updateScroll = isScrollAtTheEnd();

            listMessages.push(message)
            listMessages = listMessages;
        }
        else if(data.command === "JOIN") {
            message.date = new Date();
            updateScroll = isScrollAtTheEnd();
            listMessages.push(message)
            listMessages = listMessages;
            updateUsers();
        }
        else if(data.command === "QUIT") {
            message.date = new Date();
            updateScroll = isScrollAtTheEnd();
            listMessages.push(message)
            listMessages = listMessages;
            updateUsers();
        }
        else if(data.command === "TOPIC") {
            message.date = new Date();
            updateScroll = isScrollAtTheEnd();
            listMessages.push(message)
            listMessages = listMessages;
            updateUsers();
        }
        else if(data.command === "RESPONSE"){
            if(data.response.kind === 97) { //users
                users = []
                updateUsers();
            }
        }

})

    invoke('connect').then(()=> {
        getUsers().then((data)=> {
            users = data as User[];
        })
    })

});

onDestroy(async ()=> {
    invoke('disconnect', {message:"Bye"});
})

function updateUsers() {
    getUsers().then((data)=> {
        users = data as User[];
        isLoaded = true;
    })
}

function sendCurrentMessage() {
    updateScroll = isScrollAtTheEnd();
    let message : Message;
    message = {nick_name:nickName, content:messageToSend, date: new Date(), link:parseMessage(messageToSend)}
    listMessages.push(message)
    invoke('send_message', {message:messageToSend}).then(()=> {

    })
    listMessages = listMessages;
    messageToSend = "";
}

function getUsers() {
    return new Promise(resolve=> {
        invoke('get_users').then((data)=> {
        resolve(data)
    })
    })

}

$: isSameMessage = (id, message) : boolean => {return (id === 0 || (id > 0 && listMessages[id - 1].nick_name !== message.nick_name))}
</script>


<main>
    {#if !isLoaded}
    <div class="loading" class:loading-hide={isLoaded}>
        <Jumper size="60" color="#845EC2" unit="px" duration="1s"></Jumper>
    </div>
    {:else}
    <div class="discuss-section">
        <div class="wrapper-messages" >
            <div class="messages" bind:this={discussSection}>
                {#each listMessages as message, id}
                <div class="message" style="--space:{isSameMessage(id, message)? "10px" : "0px"}">
                    {#if isSameMessage(id, message)}
                    <div class="title">
                        <div class="username" class:username-me={message.nick_name === nickName}>
                            {message.nick_name}
                        </div>
        
                        <div class="date">
                            {message.date.toLocaleTimeString()}
                        </div>
                    </div>
                    {/if}
                    <div class="message-content" class:message-content-system={message.nick_name === ""}>
                        <div>{message.content}</div>
                    </div>
                </div>
                {/each}
            </div>
        </div>
    
    
        <div class="wrapper-writter">
            <div class="write-section">
                <input type="text" bind:value={messageToSend} 
                on:keyup={e=>e.key==='Enter' && sendCurrentMessage()}>
                <button on:click={()=> {
                   sendCurrentMessage()
    
                }}><PlusSign width=15 height=15></PlusSign></button>
            </div>
        </div>
    
    </div>

    <div class="list-users">
        {#each users as user }
            <div class="user-item">
                {user.nick_name}
            </div>
        {/each}
    </div>
    {/if}


</main>

<style>
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
        padding-left: 15px;
        padding-right: 15px;
        min-width: 50px;
        background-color: var(--primary-accent-color);
        color: var(--background-color);
    }

    main {
        height: 100%;
        position: relative;
        display: flex;
        flex-direction: row;
    }

    .discuss-section {
        flex: 1;
        height: 100%;
        position: relative;
        display: flex;
        flex-direction: column;
    }

    input {
        width: 100%;
    }

    button {
        padding: 0px;
        border-radius: 0px;
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

       height: calc(100vh - 35px);
       margin-left: 8px;

    }

    .wrapper-writter {
        align-items: center;
        justify-content: center;
        margin: auto;
        width: 95%;
        flex: 0 1 auto;
        height: 30px;
        margin-bottom: 7px;
    }

    .message {
        margin-top: var(--space);
    }

    .message-content {
        margin-left: 20px;
        display: flex;
        flex-direction: column;
    }

    .message-content-system {
        margin-left: 20px;
        color:#B0A8B9;
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