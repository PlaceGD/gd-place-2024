<script lang="ts">
    import { default as cx } from "classnames";

    import Image from "../../components/Image.svelte";

    import { getObjsInOrder } from "../../gd/object";

    import { DEBUG } from "../../utils/debug";

    import { TabGroup, menuSettings } from "../../stores";
    import ObjectButtonImage from "./ObjectButtonImage.svelte";

    let objButtonSize = 0;
</script>

<div
    class="absolute opacity-0 w-16 h-16 md:w-12 md:h-12 xs:w-10 xs:h-10"
    style="position: absolute;"
    bind:offsetWidth={objButtonSize}
/>
<ul
    class={cx({
        "w-full h-full overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar object-grid-container": true,
        "!hidden": $menuSettings.selectedGroup != TabGroup.Build,
    })}
    tabindex="-1"
>
    {#each getObjsInOrder() as [id, obj]}
        <li
            class={cx({
                "relative w-16 h-16 md:w-12 md:h-12 xs:w-10 xs:h-10": true,
                hidden: $menuSettings.selectedBuildTab != obj.category,
            })}
        >
            <button
                class={"absolute w-full h-full p-3 md:p-2 xs:p-1 z-20"}
                tabindex={$menuSettings.isMinimized ? -1 : 0}
                on:click={() => {
                    $menuSettings.selectedObject = id;
                }}
            >
                {#if $DEBUG}
                    <!-- <span
                        class="absolute opacity-50 text-red font-lg bottom-3/4 right-1/2"
                    >
                        {id}
                    </span> -->
                {/if}
                <div class="relative w-full h-full flex-center">
                    <ObjectButtonImage {id} {objButtonSize} />
                    <!-- <img
                        draggable="false"
                        class="absolute object-none"
                        src={$spritesheetProgress.blobURL}
                        style={getImgStyle(id, objButtonSize)}
                        alt=""
                    /> -->
                    <!-- <Image
                        class="absolute object-contain max-w-full max-h-full"
                        src={`/textures/main/${id}.png`}
                        lazyLoad
                        skeleton
                    ></Image> -->
                </div>
            </button>
            {#if $menuSettings.selectedObject == id}
                <span class="absolute w-full h-full sliding-selector"></span>
            {/if}
        </li>
    {/each}
</ul>

<style lang="postcss">
    .object-grid-container {
        @apply grid justify-between p-4 md:p-3 xs:p-2;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    @media screen(sm) {
        .object-grid-container {
            grid-template-columns: repeat(auto-fill, 56px);
        }
    }

    @media screen(xs) {
        .object-grid-container {
            grid-template-columns: repeat(auto-fill, 48px);
        }
    }
</style>
