<script lang="ts">
  import Discuss from './lib/Discuss.svelte'
  import Connection from './lib/Connection.svelte'
  import { onMount, onDestroy } from 'svelte';
  import {config} from './lib/config'

  let nickName="pickles"
  let server="chat.freenode.net" 
  let channel="#rust-spam"
  let password=""

  onMount(async () => {
    await config.read();
    const c = config.getConnectionConfig();
    console.log(c)
    if(c.hasOwnProperty("nickName"))
      nickName = c.nickName;
    if(c.hasOwnProperty("server"))
      server = c.server;
    if(c.hasOwnProperty("channel"))
      channel = c.channel;
    if(c.hasOwnProperty("password"))
      password = c.password;

  });

  onDestroy(async () => {
    config.write();
  });

  let isConnected = false;



</script>

<main>
  {#if !isConnected}

  <Connection 
  bind:nickName={nickName} 
  bind:server={server} 
  bind:channel={channel}
  bind:password={password}
  on:connected={() => {
    isConnected = true;
    config.setConnectionSettings(nickName, server, channel, password);
    config.write()
    }}>
  </Connection>

  {:else}
  <Discuss nickName={nickName} 
  server={server} 
  channel={channel} ></Discuss>
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100%;
  }
</style>
