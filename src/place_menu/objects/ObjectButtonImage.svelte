<!-- <script lang="ts">
    import { objects, spritesheet, type SpriteData } from "shared-lib/gd";
    import { round } from "shared-lib/util";
    import { spritesheetProgress } from "../../load_wasm";

    export let id: number;
    export let objButtonSize: number;

    const getImgStyle = (id: number, objButtonSize: number) => {
        let mainSprite: SpriteData | null = spritesheet.mainSprites[id];
        let detailSprite: SpriteData | null = spritesheet.detailSprites[id];

        let builtinScale = objects[id].builtinScale;
        let maxDisplaySize = 120 / builtinScale;

        let largest = [
            Math.max(mainSprite?.size[0] ?? 1, detailSprite?.size[0] ?? 1),
            Math.max(mainSprite?.size[1] ?? 1, detailSprite?.size[1] ?? 1),
        ];

        // console.log(mainSprite, detailSprite);
        let detailOffset = [0, 0];
        if (mainSprite != null && detailSprite != null) {
            detailOffset = [
                detailSprite.offset[0] - mainSprite.offset[0],
                detailSprite.offset[1] - mainSprite.offset[1],
            ];
        }

        let scale =
            objButtonSize /
            1.45 /
            Math.max(largest[0], largest[1], maxDisplaySize);

        return [
            mainSprite == null
                ? null
                : `
                width: ${mainSprite.size[0]}px;
                height: ${mainSprite.size[1]}px;
                max-height: none;
                max-width: none;
                object-position: ${-mainSprite.pos[0]}px ${-mainSprite.pos[1]}px;
                transform: scale(${round(scale, 2)});
            `,
            detailSprite == null
                ? null
                : `
                width: ${detailSprite.size[0]}px;
                height: ${detailSprite.size[1]}px;
                max-height: none;
                max-width: none;
                object-position: ${-detailSprite.pos[0]}px ${-detailSprite.pos[1]}px;
                transform: scale(${round(scale, 2)}) translate(${detailOffset[0]}px, ${detailOffset[1]}px);
                
                background-color: #B2B2FF;
                mask-image: url(${$spritesheetProgress.blobURL});
                mask-mode: alpha;
                mask-position: ${-detailSprite.pos[0]}px ${-detailSprite.pos[1]}px;
            `,
            detailSprite == null
                ? null
                : `
                width: ${detailSprite.size[0]}px;
                height: ${detailSprite.size[1]}px;
                max-height: none;
                max-width: none;
                object-position: ${-detailSprite.pos[0]}px ${-detailSprite.pos[1]}px;
                transform: scale(${round(scale, 2)}) translate(${detailOffset[0]}px, ${detailOffset[1]}px);
                mix-blend-mode: multiply;
                // filter: grayscale(100%);
            `,
        ];
    };

    $: [mainStyle, detailColoredStyle, detailMultStyle] = getImgStyle(
        id,
        objButtonSize
    );
</script>

<div class="relative w-0 h-0 flex-center">
    {#if mainStyle != null}
        <img
            draggable="false"
            class="absolute object-none"
            src={$spritesheetProgress.blobURL}
            style={mainStyle}
            alt=""
        />
    {/if}
    {#if detailColoredStyle != null}
        <div
            class="absolute object-none"
            style={`
            ${detailColoredStyle}
        `}
        />
        <img
            draggable="false"
            class="absolute object-none"
            src={$spritesheetProgress.blobURL}
            style={detailMultStyle}
            alt=""
        />
    {/if}
</div> -->

<script lang="ts">
    import Image from "../../components/Image.svelte";
    import { Spritesheet } from "../../utils/spritesheet/spritesheet";

    export let id: number;

    let src: string | null = null;

    const onImageVisible = () => {
        // console.log(objButtonSize);

        Spritesheet.spriteImageStringFromId(id).then(bSrc => {
            // console.log("gle", bSrc);
            src = bSrc;
        });
    };
</script>

<!-- <div class="relative flex-center"> -->
<Image
    bind:src
    lazyLoad
    on:visible={onImageVisible}
    skeleton
    class="object-contain w-full h-full"
/>
<!-- </div> -->
