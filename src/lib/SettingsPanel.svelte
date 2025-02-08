<script lang="ts">
  import { config } from "./config";
  import { onMount } from "svelte";
  import type { Setting, UploadImageConfig, CompletionConfig } from "./config";

  let {
    onExit,
    onValidate,
  }: {
    onExit: () => void;
    onValidate: (arg0: Setting) => void;
  } = $props();

  let setting_clone: Setting = $state(structuredClone(config.config));
  // Utility type to map TypeScript types to string names
  type PropertyType<T, K extends keyof T> = T[K] extends string
    ? "string"
    : T[K] extends number
      ? "number"
      : T[K] extends boolean
        ? "boolean"
        : "unknown";

  // Define the Option type
  type Option<T, K extends keyof T> = {
    title: string;
    id: K;
    type: PropertyType<T, K>;
  };

  // Generate options dynamically based on object keys
  function generateOptions<T>(config: T): Option<T, keyof T>[] {
    return (Object.keys(config) as (keyof T)[]).map((key) => ({
      title: String(key).replace(/_/g, " ").toUpperCase(),
      id: key,
      type: typeof config[key] as PropertyType<T, typeof key>,
    }));
  }

  // Section types to enforce correct structure
  type Section<T, ID extends string> = {
    section: string;
    id: ID;
    options: Option<T, keyof T>[];
  };

  // List of sections with mixed types
  const list_options: (
    | Section<UploadImageConfig, "upload_image">
    | Section<CompletionConfig, "completion">
  )[] = [
    {
      section: "Upload",
      id: "upload_image",
      options: generateOptions(setting_clone.upload_image),
    },
    {
      section: "Completion",
      id: "completion",
      options: generateOptions(setting_clone.completion),
    },
  ];
  onMount(async () => {});
</script>

<div class="panel gap-4 p-7">
  <form class="w-full max-w-sm overflow-y-auto">
    {#if setting_clone}
      {#each list_options as section}
        <div class="font-bold">{section.section}</div>
        {#each section.options as option}
          <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
              <label
                class="block md:text-right mb-1 md:mb-0 pr-4"
                for={`inline-${option.id}`}
              >
                {option.title}
              </label>
            </div>
            <div class="md:w-2/3">
              <input
                class="appearance-none border-2 rounded w-full py-2 px-4 leading-tight focus:outline-none"
                id={`inline-${option.id}`}
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
    max-height: 90%;
    max-width: 80%;
    min-width: 50%;
    @apply bg-surface-50-950;
    @apply justify-center;
    -webkit-box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    box-shadow: 5px 5px 15px 5px rgba(0, 0, 0, 0.48);
    border-radius: 5px;
    border-width: 3px;
    @apply text-primary-950-50;
    @apply border-secondary-500;
  }
</style>
