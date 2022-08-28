<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
	import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
    export let nickName :string ="pickles"
    export let server: string = "chat.freenode.net" 
    export let channel:string = "#share-chan"
    export let password : string =""
    const connect = () => {
        
        invoke('loggin', {nickName:nickName, server:server, channel:channel, password:password})
        .then(() => {
            dispatch('connected')
        })
    }

</script>

<main>
    <form on:submit|preventDefault={connect} class="form">
          <label for="nickname" class="">
            NickName
          </label>
        <input bind:value={nickName} type="text" id="nickname" autocomplete="off" class="group"/>
        <label for="server" class="">
            Server
          </label>
        <input bind:value={server} type="text" id="server" autocomplete="off" class="group"/>
        <label for="channel" class="">
            Channel
          </label>
        <input bind:value={channel} type="text" id="channel" autocomplete="off" class="group"/>
        <label for="password" class="">
            Password
          </label>
        <input bind:value={password} type="password" id="password" autocomplete="off" class="group"/>
        <button type="submit">
            Connexion
        </button>
      </form>
</main>

<style>

    .group {
        margin-bottom: 10px;
    }

    input {
        font-family: inherit;
        border-radius: 5px;
    }

    .form {
       
        position: absolute;
        top: 50%;
        left:50%;
        transform: translate(-50%, -100%);
        display: flex;
        flex-direction: column;
        padding: 10px;

        justify-content: space-between;
    }

</style>