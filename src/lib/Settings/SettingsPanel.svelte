<script lang="ts">
  import { config } from "../config";
  import type {
    Setting,
    UploadImageConfig,
    CompletionConfig,
    Theme,
  } from "../config";

  let {
    onExit,
    onValidate,
  }: {
    onExit: () => void;
    onValidate: (arg0: Setting) => void;
  } = $props();

  function propertyOf<TObj>(name: keyof TObj) {
    return name;
  }

  let setting_clone: Setting = $state(structuredClone(config.config));
  let list_options = [
    {
      section: "Upload",
      id: "upload_image",
      options: [
        {
          title: "Url get",
          id: propertyOf<UploadImageConfig>("url_get"),
          type: "string",
          values: "",
        },
        {
          title: "Url post",
          id: propertyOf<UploadImageConfig>("url_post"),
          type: "string",
          values: "",
        },
      ],
    },
    {
      section: "Completion",
      id: "completion",
      options: [
        {
          title: "Url",
          id: propertyOf<CompletionConfig>("url"),
          type: "string",
          values: "",
        },
        {
          title: "Token",
          id: propertyOf<CompletionConfig>("token"),
          type: "string",
          values: "",
        },
        {
          title: "Triggers",
          id: propertyOf<CompletionConfig>("triggers"),
          type: "string",
          values: "",
        },
      ],
    },
    {
      section: "Theme",
      id: "theme",
      options: [
        {
          title: "mode",
          id: propertyOf<Theme>("mode"),
          type: "array",
          values: ["light", "dark"],
        },
        {
          title: "name",
          id: propertyOf<Theme>("name"),
          type: "array",
          values: ["modern", "cerberus"],
        },
      ],
    },
  ];
</script>

<div class="panel gap-4 pb-3">
  <form class="w-full max-w-sm pl-7 pt-7 pr-7 overflow-y-auto">
    {#if setting_clone}
      {#each list_options as section}
        <div class="font-bold">{section.section}</div>
        {#each section.options as option}
          <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
              <label
                class="label block md:text-left mb-1 md:mb-0 pr-4"
                for={`inline-${option.id}`}
              >
                {option.title}
              </label>
            </div>
            {#if Object.hasOwn(option, "type") && option.type == "string"}
              <div class="md:w-2/3">
                <input
                  class="input appearance-none border-2 rounded-sm w-full py-2 px-4 leading-tight focus:outline-hidden"
                  id={`inline-${option.id}`}
                  type="text"
                  bind:value={setting_clone[section.id][option.id]}
                />
              </div>
            {:else if Object.hasOwn(option, "type") && option.type == "array"}
              <select
                id={`inline-${option.id}`}
                class="select appearance-none border-2 rounded-sm w-full py-2 px-4 leading-tight focus:outline-hidden"
                bind:value={setting_clone[section.id][option.id]}
              >
                {#each option.values as v}
                  <option value={v}>{v}</option>
                {/each}
              </select>
            {:else}{/if}
          </div>
        {/each}
      {/each}
    {/if}
  </form>

  <div class="flex flex-row justify-center">
    <button
      class="btn preset-filled-primary-600-400 mt-1 justify-center p-4"
      onclick={() => {
        onValidate($state.snapshot(setting_clone));
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
    border-radius: 0.25rem;
    @apply border w-full;
  }

  .panel {
    display: flex;
    flex-direction: column;
    position: fixed; /* Stay in place */
    z-index: 1; /* Stay on top */
    left: 50%;
    transform: translateX(-50%);
    max-height: 90%;
    max-width: 80%;
    min-width: 50%;
    background-color: light-dark(var(--color-surface-50) /* oklch(98.5% 0 0deg) = #fafafa */, var(--color-surface-950) /* oklch(14.5% 0 0deg) = #0a0a0a */);
    @apply justify-center;
    -webkit-box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    border-radius: 5px;
    border-width: 3px;
    color: light-dark(var(--color-primary-950) , var(--color-primary-50) );
    border-color: var(--color-secondary-500);
  }
</style>
