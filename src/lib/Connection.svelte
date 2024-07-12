<script lang="ts">
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";

  export let hasFailed = false;
  const dispatch = createEventDispatcher();
  export let nickName: string = "pickles";
  export let server: string = "chat.freenode.net";
  export let channel: string = "#share-chan";
  export let password: string = "";
  const connect = () => {
    console.log("Try to connect");
    invoke("loggin", {
      nickName: nickName,
      server: server,
      channel: channel,
      password: password,
    })
      .then(() => {
        dispatch("connected");
      })
      .catch((e) => console.log("cannot connect"));
  };
</script>

<svelte:window
  on:keyup={(e) => {
    if (e.key == "Enter") connect();
  }}
/>

<main>
  <div in:fade={{ duration: 100 }} class="grid place-items-center">
    <form
      on:submit|preventDefault={connect}
      class="flex flex-col justify-between"
    >
      <div class="mb-1">
        <label for="nickname" class=""> NickName </label>
        <input
          bind:value={nickName}
          type="text"
          id="nickname"
          autocomplete="off"
          class="group"
        />
      </div>

      <div class="mb-1">
        <label for="server"> Server </label>
        <input
          bind:value={server}
          type="text"
          id="server"
          autocomplete="off"
          class="group"
        />
      </div>

      <div class="mb-1">
        <label for="channel"> Channel </label>
        <input
          bind:value={channel}
          type="text"
          id="channel"
          autocomplete="off"
          class="group"
        />
      </div>

      <div class="mb-1">
        <label for="password"> Password </label>
        <input
          bind:value={password}
          type="password"
          id="password"
          autocomplete="off"
          class="group"
        />
      </div>

      <button
        type="submit"
        class="bg-primary-500-400-token text-on-primary-token mt-5 btn-md"
      >
        Connexion
      </button>
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
    padding: 10px 100px 10px 100px;
    margin-top: 10px;
    visibility: hidden;
  }

  .error-show {
    visibility: visible;
  }
</style>
