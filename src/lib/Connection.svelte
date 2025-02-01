<script lang="ts">
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import { ProgressRing } from "@skeletonlabs/skeleton-svelte";

  let {
    nickName = $bindable<string>(""),
    server = $bindable<string>(""),
    channel = $bindable<string>(""),
    password = $bindable<string>(""),
    hasFailed,
    onConnected,
  }: {
    nickName: string;
    server: string;
    channel: string;
    password: string;
    hasFailed: boolean;
    onConnected: (arg0: string) => void;
  } = $props();

  let isConnecting = $state(false);
  const connect = () => {
    if (isConnecting) return;
    console.log("Try to connect");
    isConnecting = true;
    invoke("loggin", {
      nickName: nickName,
      server: server,
      channel: channel,
      password: password,
    })
      .then(() => {
        onConnected("connected");
      })
      .catch((e) => console.error(e))
      .finally(() => {
        isConnecting = false;
      });
  };
</script>

<svelte:window
  on:keyup={(e) => {
    if (e.key == "Enter") connect();
  }}
/>

<main>
  <div in:fade={{ duration: 100 }} class="w-full flex justify-center">
    <form onsubmit={connect} class="flex flex-col justify-between">
      <div class="flex items-center mb-6">
        <div class="w-1/3">
          <label for="nickname" class="block text-left mb-1 md:mb-0 pr-4">
            NickName
          </label>
        </div>
        <div class="w-2/3">
          <input
            bind:value={nickName}
            type="text"
            id="nickname"
            autocomplete="off"
            class=" py-2 px-4 leading-tight"
          />
        </div>
      </div>

      <div class="flex items-center mb-6">
        <div class="w-1/3">
          <label for="server" class="block text-left mb-1 md:mb-0 pr-4">
            Server
          </label>
        </div>
        <div class="w-2/3">
          <input
            bind:value={server}
            type="text"
            id="server"
            autocomplete="off"
            class=" py-2 px-4 leading-tight"
          />
        </div>
      </div>

      <div class="flex items-center mb-6">
        <div class="w-1/3">
          <label for="channel" class="block text-left mb-1 md:mb-0 pr-4">
            Channel
          </label>
        </div>
        <div class="w-2/3">
          <input
            bind:value={channel}
            type="text"
            id="channel"
            autocomplete="off"
            class=" py-2 px-4 leading-tight"
          />
        </div>
      </div>

      <div class="flex items-center mb-6">
        <div class="w-1/3">
          <label for="password" class="block text-left mb-1 md:mb-0 pr-4">
            Password
          </label>
        </div>
        <div class="w-2/3">
          <input
            bind:value={password}
            type="password"
            id="password"
            autocomplete="off"
            class=" py-2 px-4 leading-tight"
          />
        </div>
      </div>

      <div class="flex flex-row">
        <button
          type="submit"
          class="preset-filled-primary-600-400 mt-5 btn-md w-[calc(100%)] flex items-center justify-center p-4"
          disabled={isConnecting}
        >
          Connexion
        </button>
        {#if isConnecting}
          <div class="pt-5 pl-3">
            <ProgressRing value={null} size="size-14" />
          </div>
        {/if}
      </div>
    </form>
    <div class="error" class:error-show={hasFailed}>Wrong identifiers!</div>
  </div>
</main>

<style lang="css">
  .error {
    background-color: rgb(255, 139, 139);
    color: rgb(216, 0, 0);
    border: 2px solid red;
    border-radius: 3px;
    margin-top: 10px;
    display: none;
  }

  button:disabled,
  button[disabled] {
    @apply bg-primary-100 text-primary-500;
  }

  .error-show {
    display: block;
  }
</style>
