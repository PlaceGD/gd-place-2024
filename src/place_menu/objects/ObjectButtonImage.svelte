<script lang="ts">
    import { objects, spritesheet } from "shared-lib/gd";
    import Image from "../../components/Image.svelte";
    import { Spritesheet } from "../../utils/spritesheet/spritesheet";
    import { getFixedSize } from "../../utils/spritesheet/util";

    export let id: number;

    let [fWidth, fHeight] = getFixedSize(
        spritesheet.main_sprites[id],
        spritesheet.detail_sprites[id]
    );

    let maxDimension = Math.max(
        fWidth * objects[id].builtinScaleX,
        fHeight * objects[id].builtinScaleY
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
    style={`transform: scale(${scale}); ${objects[id]?.sheet == "PixelSheet01" ? "image-rendering: pixelated;" : ""}`}
/>
