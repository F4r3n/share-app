<script lang="ts">
    import { config, type Setting } from "../config";
    import SettingsButton from "./SettingsButton.svelte";
    import SettingsPanel from "./SettingsPanel.svelte";

    let {isSettingsOpened = $bindable(false)}: {
        isSettingsOpened : boolean
    } = $props()
</script>

<div class="m-1 text-primary-600">
    <SettingsButton
      onToggle={() => {
        isSettingsOpened = true;
      }}
    ></SettingsButton>
  </div>
  {#if isSettingsOpened}
      <SettingsPanel
        onExit={() => {
          isSettingsOpened = false;
        }}
        onValidate={(setting : Setting
        ) => {
          config.setConfig(setting);
          config.write();
          isSettingsOpened = false;
        }}
      ></SettingsPanel>
  {/if}