<script lang="ts">
  import Discuss from "$lib/Discuss/Discuss.svelte";
  import Connection from "./lib/Connection.svelte";
  import { onMount, onDestroy } from "svelte";
  import { config } from "./lib/config";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import Settings from "$lib/Settings/Settings.svelte";

  let nickName: string = "pickles";
  let server: string = "chat.freenode.net";
  let channel: string = "#rust-spam";
  let password: string = "";

  let isSettingsOpened = false;
  let hasFailed: boolean = false;
  let errorMessage = "";
  type Event = {
    payload: {
      kind: string;
    };
  };
  let unlisten: () => void;
  onMount(async () => {
    try {
      unlisten = await listen("irc-event", (event: Event) => {
        if (event.payload.kind === "Quit") {
          isConnected = false;
        } else if (event.payload.kind === "ErrorConnection") {
          isConnected = false;
          hasFailed = true;
        }
      });
    } catch (e) {
      console.error(e);
    }

    try {
      await config.read();
      const c = config.getConnectionConfig();
      console.log(c);

      nickName = c.nick_name;
      server = c.server;
      channel = c.channel;
      password = c.password;
    } catch (e) {
      console.error(e);
    }
  });

  onDestroy(async () => {
    unlisten();
    invoke("disconnect", {
      message: "bye",
      shallSendMessage: true,
    });
    config.write();
  });

  let isConnected = false;
</script>

<main class="flex flex-col max-w-full max-h-full min-w-0">
  {#if !isConnected}
    <Settings {isSettingsOpened}></Settings>
    <Connection
      bind:nickName
      bind:server
      bind:channel
      bind:password
      {hasFailed}
      onConnected={() => {
        console.log("Is Connected");
        isConnected = true;
        config.setConnectionSettings(nickName, server, channel, password);
        config.write();
      }}
    ></Connection>
    {#if errorMessage !== ""}
      <aside class="alert variant-filled-error min-w-[60%] ml-auto mr-auto">
        <!-- Message -->
        <div class="alert-message">
          <h3 class="h3">Connection error</h3>
          <p>{errorMessage}</p>
        </div>
      </aside>
    {/if}
  {:else}
    <Discuss
      onConnectionStatus={(event: any) => {
        isConnected = event.detail.result;
        errorMessage = event.detail.message;
      }}
      {nickName}
      {channel}
    ></Discuss>
  {/if}
</main>

<style>
  main {
    font-family: 'Roboto';
  }
</style>
