<script lang="ts">
    import { fetch, ResponseType } from '@tauri-apps/api/http';
    import ATag from './ATag.svelte';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

type Preview = {
    imageURL: string | null,
    href: string,
    title: string | null,
    description: string | null,
    siteName: string | null,
    width? : number,
    height? : number
}

export let href : string;

function getOgAttribute(inMeta : HTMLMetaElement) : string | null {
    return inMeta.getAttribute("content")
}

function hasOGAttribute(inMeta : HTMLMetaElement, inName : string) : boolean {
    const property = inMeta.getAttribute("property");
    return property == inName;
}

function getPreview(inURL : string) : Promise<Preview> {
    return fetch(inURL, {method: 'GET', responseType : ResponseType.Text})
        .then(res => {
            let data = res.data as string
            let value : Preview = {
                imageURL : null,
                href : inURL,
                description : "",
                siteName: "",
                title : inURL,
                width : 0,
                height : 0,
            }
            if(res)
            {
                const parser = new DOMParser();
                const doc : Document = parser.parseFromString(data, 'text/html');
                const head = doc.getElementsByTagName("head")[0] as HTMLHeadElement
                const metas : HTMLCollectionOf<HTMLMetaElement> = head.getElementsByTagName("meta");
                if(metas.length > 0) {
                    for(const meta of metas) {
                        if(hasOGAttribute(meta, "og:image"))
                        {
                            value.imageURL = getOgAttribute(meta);
                        }
                        else if(hasOGAttribute(meta, "og:title")) {
                            value.title = getOgAttribute(meta);
                        }
                        else if(hasOGAttribute(meta, "og:description")) {
                            value.description = getOgAttribute(meta);
                        }
                        else if(hasOGAttribute(meta, "og:site_name")) {
                            value.siteName = getOgAttribute(meta);
                        }
                    }
                }
            }
            return value;
    })

}

</script>

{#await getPreview(href)}
    
{:then preview}
    {#if preview.imageURL != null}
    <div class="image-container">
        {#if preview.siteName}
            <div class="subtitle">{preview.siteName}</div>
        {/if}
        <ATag href={preview.href}><span>{preview.title}</span></ATag>
        {#if preview.description }
            <div class="description">{preview.description}</div>
        {/if}
        <img class="image" on:load={()=> {dispatch("message_formatted")}} src={preview.imageURL} alt={preview.title}>
    </div>

    {/if}
{/await}


<style>
    .description {
        font-size: small;
    }

    .subtitle {
        font-size: small;
    }

    .image-container {
        display: flex;
        flex-direction: column;
        
        border-style: solid;
        border-radius: 5px;
        border-width: 2px;
        border-color: var(--secondary-accent-color);
        padding: 10px;
        
        width: 400px;
        margin-top: 10px;
    }

    .image {
        margin-top: 5px;
    }
</style>