<script lang="ts">
    import { default as cx } from "classnames";

    import Image from "../../components/Image.svelte";

    import { CATEGORY_ICONS, getObjsInOrder } from "../../gd/object";

    import { DEBUG } from "../../utils/debug";

    import {
        chooseDefaultColor,
        chooseRandomTriggerColor,
        menuBuildTab,
        menuEditTab,
        menuMinimized,
        menuSelectedObject,
        menuTabGroup,
        resetPreviewColor,
        songPlaying,
        songPlayingIsPreview,
        TabGroup,
    } from "../../stores";
    import ObjectButtonImage from "./ObjectButtonImage.svelte";
    import { type ObjectInfo, objects as objectMap } from "shared-lib/gd";
    import { onMount } from "svelte";

    import fireMp3Url from "../assets/fire.mp3?url";
    import {
        BG_TRIGGER,
        COLOR_TRIGGERS,
        GROUND_2_TRIGGER,
        GROUND_TRIGGER,
        SFX_TRIGGER,
        SONG_TRIGGER,
    } from "shared-lib/nexusgen";
    import { playSound, stopSound } from "../../utils/audio";
    import sfxNoteIconUrl from "../assets/objects_tab/sfx_note.png";
    import songNoteIconUrl from "../assets/objects_tab/song_note.png";
    import FadedScroll from "../../components/FadedScroll.svelte";
    import Edit from "../../icons/Edit.svelte";
    import { EditTab } from "../edit/edit_tab";

    import * as wasm from "wasm-lib";
    export let state: wasm.State;

    let objects: [number, ObjectInfo][] = [];

    onMount(() => {
        objects = getObjsInOrder();
    });

    const SPECIAL_ICON_TAILWIND =
        "absolute left-[25%] top-[45%] right-[25%] bottom-[5%]";
</script>

<fieldset
    class={cx({
        "w-full h-full flex flex-col p-2": true,
        "!hidden": $menuTabGroup != TabGroup.Build,
    })}
    disabled={$menuMinimized}
>
    <FadedScroll update={$menuTabGroup} threshold={1}>
        {#each Object.entries(CATEGORY_ICONS) as [key, path] (key)}
            <ul
                class={cx({
                    "rounded-lg object-grid-container": true,
                    "!hidden": $menuBuildTab != key,
                })}
                tabindex="-1"
            >
                {#each objects.filter(([_, obj]) => obj.category == key) as [id, _] (id)}
                    <li
                        class="relative w-16 h-16 md:w-12 md:h-12 xs:w-10 xs:h-10"
                    >
                        <button
                            class={"absolute w-full h-full p-3 md:p-2 xs:p-1 z-20"}
                            tabindex={$menuMinimized ? -1 : 0}
                            on:click={() => {
                                // if (id == 3854) {
                                //     playSound({ url: fireMp3Url, volume: 0.04 });
                                // }

                                resetPreviewColor(state, $menuSelectedObject);
                                if (
                                    COLOR_TRIGGERS.includes($menuSelectedObject)
                                ) {
                                    chooseRandomTriggerColor(
                                        state,
                                        $menuSelectedObject
                                    );
                                } else {
                                    chooseDefaultColor();
                                }
                                if ($menuSelectedObject != SONG_TRIGGER) {
                                    stopSound("preview song");
                                    if ($songPlayingIsPreview)
                                        songPlaying.set(false);
                                }

                                $menuSelectedObject = id;
                            }}
                        >
                            {#if $DEBUG}
                                <span
                                    class="absolute z-50 flex flex-col items-start gap-0 font-bold -top-[2px] -left-[0px] text-stroke"
                                >
                                    <span class="text-red">{id}</span>
                                    <span class="text-xs text-orange"
                                        >{objectMap[id].hitboxType.slice(
                                            0,
                                            2
                                        )}</span
                                    >
                                </span>
                            {/if}
                            <div class="relative w-full h-full flex-center">
                                {#if COLOR_TRIGGERS.includes(id)}
                                    <div
                                        class={`${SPECIAL_ICON_TAILWIND} bg-white border border-black`}
                                    />
                                {/if}
                                {#if id == SFX_TRIGGER}
                                    <div class={SPECIAL_ICON_TAILWIND}>
                                        <Image
                                            src={sfxNoteIconUrl}
                                            class="object-contain w-full h-full"
                                            alt="note"
                                        />
                                    </div>
                                {/if}
                                {#if id == SONG_TRIGGER}
                                    <div class={SPECIAL_ICON_TAILWIND}>
                                        <Image
                                            src={songNoteIconUrl}
                                            class="object-contain w-full h-full"
                                            alt="note"
                                        />
                                    </div>
                                {/if}
                                <ObjectButtonImage {id} />
                            </div>
                        </button>
                        {#if $menuSelectedObject == id}
                            <span
                                class="absolute w-full h-full sliding-selector"
                            ></span>
                        {/if}
                    </li>
                {/each}
            </ul>
        {/each}
    </FadedScroll>
    {#if $menuBuildTab == "Triggers"}
        <button
            class="justify-end w-full p-4 text-sm text-center hover-text-transition"
            on:click={() => {
                $menuTabGroup = TabGroup.Edit;
                $menuEditTab = EditTab.Colors;
            }}
        >
            Go to <Edit class="stroke-[1.5] w-6 h-6 inline-block"></Edit> to change
            trigger settings. <b>All triggers are touch triggered.</b>
        </button>
    {/if}
</fieldset>

<style lang="postcss">
    .object-grid-container {
        @apply grid justify-between;
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
