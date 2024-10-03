<script lang="ts">
    import { objects, spritesheet } from "shared-lib/gd";
    import Image from "../../components/Image.svelte";
    import { Spritesheet } from "../../utils/spritesheet/spritesheet";

    export let id: number;

    let maxDimension = Math.max(
        ...(spritesheet.main_sprites[id]?.size ?? [2, 2]),
        ...(spritesheet.detail_sprites[id]?.size ?? [2, 2])
    );
    let scale = maxDimension >= 120 ? 1 : maxDimension / 120;

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
    style={`transform: scale(${scale})`}
/>
