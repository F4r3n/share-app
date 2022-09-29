<script lang="ts">
import ATag from "./ATag.svelte";
export let href : string;
</script>

<div class="image-container">
    <ATag {href}>{href}</ATag>
    <div class="image">
        <img src={href} alt={href}>
    </div>
</div>

<style>

.image-container {
    display: flex;
    flex-direction: column;

    border-style: solid;
    border-radius: 5px;
    border-width: 2px;
    border-color: var(--secondary-accent-color);
    padding: 10px;
}

.image {
    margin: auto;
    width: 30%;
    height: 30%;
}

</style>