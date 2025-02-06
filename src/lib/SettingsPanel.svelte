<script lang="ts">
  import { config } from "./config";
  import { onMount } from "svelte";
  import type { Setting } from "./config";

  let {
    onExit,
    onValidate,
  }: {
    onExit: () => void;
    onValidate: (arg0: Setting) => void;
  } = $props();

  let setting_clone: Setting = $state(structuredClone(config.config));

  let list_options = [
    {
      section: "Upload",
      id: "upload_image",
      options: [
        {
          title: "Url get",
          id: "url_get",
        },
        {
          title: "Url post",
          id: "url_post",
        },
      ],
    },
    {
      section: "Completion",
      id: "completion",
      options: [
        {
          title: "Url",
          id: "url",
        },
        {
          title: "Token",
          id: "token",
        },
      ],
    },
  ];
  onMount(async () => {

  });
</script>

<div class="panel gap-4">
  <form class="w-full max-w-sm">
    {#if setting_clone}
      {#each list_options as section}
      <div class="font-bold">{section.section}</div>
        {#each section.options as option}
          <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
              <label
                class="block md:text-right mb-1 md:mb-0 pr-4"
                for="inline-full-name"
              >
                {option.title}
              </label>
            </div>
            <div class="md:w-2/3">
              <input
                class="appearance-none border-2 rounded w-full py-2 px-4 leading-tight focus:outline-none"
                id="inline-full-name"
                type="text"
                bind:value={setting_clone[section.id][option.id]}
              />
            </div>
          </div>
        {/each}
      {/each}
    {/if}
  </form>

  <div class="flex flex-row justify-center">
    <button
      class="btn preset-filled-primary-600-400 mt-1 justify-center p-4"
      onclick={() => {
        onValidate(setting_clone);
      }}>OK</button
    >
    <button
      class="btn preset-filled-primary-600-400 mt-1 justify-center p-4 ml-1"
      onclick={() => {
        onExit();
      }}>Close</button
    >
  </div>
</div>

<style>
  .panel input {
    font-family: inherit;
    border-radius: 3px;
    line-height: 10px;
    @apply border rounded w-full;
  }

  .panel {
    display: flex;
    flex-direction: column;
    position: fixed; /* Stay in place */
    z-index: 1; /* Stay on top */
    left: 50%;
    transform: translateX(-50%);
    height: 80%;
    width: 80%;
    @apply bg-surface-50-950;
    @apply justify-center;
    -webkit-box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    border-radius: 5px;
    border-width: 3px;
    @apply border-tertiary-50;
  }
</style>
