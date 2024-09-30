<script lang="ts">
    import Image from "../../components/Image.svelte";
    import { Spritesheet } from "../../utils/spritesheet/spritesheet";

    export let id: number;

    let src: string | null = null;

    const onImageVisible = () => {
        Spritesheet.spriteImageStringFromId(id).then(bSrc => {
            src = bSrc;
        });
    };
</script>

<Image
    bind:src
    lazyLoad
    on:visible={onImageVisible}
    skeleton
    class="object-contain w-full h-full"
/>
