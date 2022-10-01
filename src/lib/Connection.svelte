<script lang="ts">
  import {fade} from "svelte/transition"
  import { invoke } from '@tauri-apps/api/tauri';
	import { createEventDispatcher } from 'svelte';

  export let hasFailed = false;
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
        .catch(e => console.log("cannot connect"))
    }

</script>

<main>
  <div in:fade={{duration:100}} class="form_position">
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
  <div class="error" class:error-show={hasFailed}>
    Wrong identifiers!
  </div>
  </div>


</main>

<style>

  .error {
    background-color: rgb(255, 139, 139);
    color:rgb(216, 0, 0);
    border: 2px solid red;
    border-radius: 3px;
    padding: 10px 100px 10px 100px;
    margin-top: 10px;
    visibility: hidden;
  }

  .error-show  {
    visibility: visible;
  }

    .group {
        margin-bottom: 10px;
    }

    input {
        font-family: inherit;
        border-radius: 5px;
    }

    .form {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      width: fit-content;
      margin: auto;
    }

    .form_position {
       
        position: absolute;
        top: 40%;
        left:50%;
        transform: translate(-50%, -50%);

        display: flex;
        flex-direction: column;
        justify-content: space-between;
        
    }

</style>