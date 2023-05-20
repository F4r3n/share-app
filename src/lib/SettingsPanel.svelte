<script lang="ts">
    import { slide } from 'svelte/transition';
    import { linear } from 'svelte/easing';
    import {config} from './config'
    import { onDestroy } from 'svelte';

    let options = {duration: 100, easing: linear};
    export let upload_image_get : string = config.getUploadImageConfig().url_get    ;
    export let upload_image_post : string = config.getUploadImageConfig().url_post;

    onDestroy(async () => {
		config.setUploadImageConfig({url_get: upload_image_get, url_post:upload_image_post})
        await config.write()
	});
</script>

<div transition:slide={{...options, axis:"x"}} class="panel">
    <div class="section">
        <div class="subtitle">Upload image get</div>
        <input type="text" bind:value={upload_image_get}/>
    </div>
    <div class="section">
        <div class="subtitle">Upload image post</div>
        <input type="text" bind:value={upload_image_post}/>
    </div>
</div>

<style>



    .subtitle {
        color: var(--outline-color);
        font-size: larger;
        margin: 5px;
    }

    .section {
        margin-left: 5px;
        margin-top: 10px;
    }

    .panel input {
        font-family: inherit;
        border-radius: 3px;
        width: 90%;
    }

    .panel {
        display: flex;
        flex-direction: column;
        position: fixed; /* Stay in place */
        z-index: 1; /* Stay on top */
        top: 0;
        left: 0;
        height: 100%;
        width:50%;
        background-color: var(--background-color);
        -webkit-box-shadow: 5px 5px 15px 5px rgba(0,0,0,0.48); 
        box-shadow: 5px 5px 15px 5px rgba(0,0,0,0.48);
    }
    
</style>