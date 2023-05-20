<script lang="ts">
  import Discuss from './lib/Discuss.svelte'
  import Connection from './lib/Connection.svelte'
  import { onMount, onDestroy } from 'svelte';
  import {config} from './lib/config'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri';
  import SettingsButton from "./lib/SettingsButton.svelte"
  import SettingsPanel from '$lib/SettingsPanel.svelte';
  import { clickOutside } from './lib/clickOutside';

  let nickName="pickles"
  let server="chat.freenode.net" 
  let channel="#rust-spam"
  let password=""

  let isSettingsOpened = false;
  let hasFailed = false;

  type Event = {

    payload : {
      kind : string
    }
  }

  onMount(async () => {

    await listen('irc-event', (event : Event)=> {
        if(event.payload.kind === "Quit")
        {
          isConnected = false;
        }
        else if(event.payload.kind === "ErrorConnection")
        {
          isConnected = false;
          hasFailed = true;
        }
    })

    await config.read();
    const c = config.getConnectionConfig();

    nickName = c.nick_name;
    server = c.server;
    channel = c.channel;
    password = c.password;

  });

  onDestroy(async () => {
    invoke("disconnect", {message:"bye", shallSendMessage:true, wrongIdentifer:false});
    config.write();
  });

  let isConnected = false;

  function handleClickOutside(event : CustomEvent) {
		isSettingsOpened = false;
	}
</script>


<main>
  {#if !isConnected}
  <div class="settings-button">
    <SettingsButton on:toggle={() => {isSettingsOpened = true; console.log(isSettingsOpened)}}></SettingsButton>
  </div>
  {#if isSettingsOpened}
  <div use:clickOutside on:click_outside={handleClickOutside}>
    <SettingsPanel></SettingsPanel>
  </div>
  {/if}
  <Connection
  bind:nickName={nickName} 
  bind:server={server} 
  bind:channel={channel}
  bind:password={password}
  {hasFailed}
  on:connected={() => {
    isConnected = true;
    config.setConnectionSettings(nickName, server, channel, password);
    config.write()
    }}>
  </Connection>


  {:else}
  <Discuss  on:connection_status={(event)=> {isConnected = event.detail;}}
  nickName={nickName} 
  channel={channel}></Discuss>
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100%;
  }

  .settings-button {
    color: var(--outline-color);
    display: flex;
    float: right;
    margin: 5px;
    cursor: pointer;
  }

  .settings-button:active {
      color:var(--press-color)
  }
</style>
