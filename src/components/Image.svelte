<script lang="ts">
    export let src: string;
    export let alt: string = "";
    export let lazyLoad: boolean = false;
    export let loadAsync: boolean = false;
    export let skeleton: boolean = false;

    const preload = async (src: string): Promise<string> => {
        const resp = await fetch(src);
        const blob = await resp.blob();

        return new Promise(function (resolve, reject) {
            let reader = new FileReader();
            reader.onload = () => resolve(reader.result! as string);
            reader.onerror = error => reject(error);
            reader.readAsDataURL(blob);
        });
    };
</script>

{#if loadAsync}
    {#await preload(src)}
        {#if skeleton}{:else}

        {/if}
    {:then base64}
        <img src={base64} {alt} {...$$restProps} draggable="false" />
    {/await}
{:else}
    <img {src} {alt} {...$$restProps} draggable="false" />
{/if}
