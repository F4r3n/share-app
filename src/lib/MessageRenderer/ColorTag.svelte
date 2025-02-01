<script lang="ts">
    import { onMount, onDestroy } from "svelte";

    let { background, foreground }: { background: number; foreground: number } =
        $props();

    let cb: any = $state({});
    let cf: any = $state({});
    const ircColor = [
        { name: "White", value: "FFFFFF" },
        { name: "Black", value: "000000" },
        { name: "Navy", value: "00007F" },
        { name: "Green", value: "009300" },
        { name: "Red", value: "FF0000" },
        { name: "Maroon", value: "7F0000" },
        { name: "Purple", value: "9C009C" },
        { name: "Orange", value: "FC7F00" },
        { name: "Yellow", value: "FFFF00" },
        { name: "Light Green", value: "00FC00" },
        { name: "Teal", value: "009393" },
        { name: "Cyan", value: "00FFFF" },
        { name: "Royal blue", value: "0000FC" },
        { name: "Magenta", value: "FF00FF" },
        { name: "Gray", value: "7F7F7F" },
        { name: "Light Gray", value: "D2D2D2" },
    ];
    onMount(async () => {
        cb = ircColor[background];
        cf = ircColor[foreground];
    });

    onDestroy(async () => {});
</script>

{#if cf && cb}
    <span style="background-color:#{cb.value};color:#{cf.value}"><slot /></span>
{:else}
    <span><slot /></span>
{/if}
