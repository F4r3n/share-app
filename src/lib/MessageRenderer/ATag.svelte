<script lang="ts">
    import { open } from "@tauri-apps/plugin-shell";
    import { onMount, onDestroy } from "svelte";

    export let href: string;

    onMount(async () => {
        link.addEventListener(
            "click",
            function (event) {
                open(link.href);
                event.preventDefault();
            },
            false,
        );
    });

    onDestroy(async () => {});

    let link: HTMLAnchorElement;
</script>

<a target="" bind:this={link} {href}><slot></slot></a>
