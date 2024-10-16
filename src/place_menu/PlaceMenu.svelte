<script lang="ts">
    import * as wasm from "wasm-lib";

    import { default as cx } from "classnames";

    import { colors, objects } from "shared-lib/gd";
    import {
        BG_TRIGGER,
        GROUND_TRIGGER,
        GROUND_2_TRIGGER,
        ARROW_TRIGGER,
        SFX_TRIGGER,
        SONG_TRIGGER,
    } from "shared-lib/nexusgen";
    import { CATEGORY_ICONS } from "../gd/object";

    import Image from "../components/Image.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";

    import Build from "../icons/Build.svelte";
    import Edit from "../icons/Edit.svelte";
    import Delete from "../icons/Delete.svelte";
    import Minimize from "../icons/Caret.svelte";

    import {
        TabGroup,
        editorSettings,
        selectedObject,
        menuSelectedObject,
        menuMainColor,
        menuDetailColor,
        menuZLayer,
        menuZOrder,
        menuTabGroup,
        menuMinimized,
        menuBuildTab,
        menuEditTab,
        bgColor,
        ground1Color,
        ground2Color,
        loginData,
        mainColorRGB,
        detailColorRGB,
        menuSelectedSFX,
        menuSpeed,
        menuSelectedSong,
        canPlacePreview,
    } from "../stores";
    import { addObject, removeObject } from "../firebase/object";
    import { DEBUG } from "../utils/debug";
    // import SpriteSheet from "../utils/spritesheet";

    import {
        EditTab,
        getTransformedPlaceOffset,
        TRANSFORM_BUTTONS,
        WidgetType,
    } from "./edit/edit_tab";
    import ColorsTab from "./edit/ColorsTab.svelte";
    import LayersTab from "./edit/LayersTab.svelte";
    import TransformTab from "./edit/TransformTab.svelte";
    import ObjectsTab from "./objects/ObjectsTab.svelte";
    import {
        blur,
        fade,
        scale,
        type TransitionConfig,
    } from "svelte/transition";
    import { COLOR_TRIGGERS } from "shared-lib/nexusgen";
    import { timerDisplay } from "shared-lib/util";
    import { SyncedCooldown } from "../utils/cooldown";
    import { db } from "../firebase/firebase";
    import { onDestroy, onMount } from "svelte";
    import RadialCooldown from "../components/RadialCooldown.svelte";
    import { setCheckedPreviewObject } from "../utils/misc";
    import DeleteTab from "./delete/DeleteTab.svelte";
    import SFXSongTab from "./edit/SFXSongTab.svelte";
    import { playSound } from "../utils/audio";
    import deleteTimerFinishedSoundUrl from "./assets/sounds/delete_timer_finished.ogg?url";
    import placeTimerFinishedSoundUrl from "./assets/sounds/place_timer_finished.ogg?url";

    export let state: wasm.State;

    const minimizeAnimDur = 0.5;

    $: showColorsTab = $menuSelectedObject != ARROW_TRIGGER;
    const editTabName = (tab: EditTab) => {
        if (tab == EditTab.Colors) {
            if ($menuSelectedObject == SFX_TRIGGER) {
                return "SFX";
            } else if ($menuSelectedObject == SONG_TRIGGER) {
                return "Song";
            }
            return EditTab.Colors;
        }
        return tab;
    };
    $: {
        if (COLOR_TRIGGERS.includes($menuSelectedObject)) {
            $menuMainColor.blending = false;
            $menuDetailColor.blending = false;
            $menuMainColor.opacity = 1;
            $menuDetailColor.opacity = 1;
        }
    }
    $: {
        if ($menuSelectedObject == ARROW_TRIGGER) {
            if ($menuEditTab == EditTab.Colors) {
                $menuEditTab = EditTab.Transform;
            }
        }
    }
    $: [mainR, mainG, mainB] = $mainColorRGB;
    $: [detailR, detailG, detailB] = $detailColorRGB;

    $: {
        let obj = state.get_preview_object();
        let old_id = obj.id;

        let [oldPx, oldPy] = getTransformedPlaceOffset(obj);
        obj.id = $menuSelectedObject;
        let [newPx, newPy] = getTransformedPlaceOffset(obj);

        obj.x = obj.x + oldPx - newPx;
        obj.y = obj.y + oldPy - newPy;

        if (obj.id == SFX_TRIGGER) {
            obj.main_color = new wasm.GDColor(
                $menuSelectedSFX,
                $menuSpeed + 12,
                0,
                255,
                false
            );
        } else if (obj.id == SONG_TRIGGER) {
            obj.main_color = new wasm.GDColor(
                $menuSelectedSong,
                $menuSpeed + 12,
                0,
                255,
                false
            );
        } else if (old_id == SFX_TRIGGER || old_id == SONG_TRIGGER) {
            obj.main_color = wasm.GDColor.white();
        }

        setCheckedPreviewObject(state, obj);
    }

    $: {
        let obj = state.get_preview_object();
        if (
            $menuSelectedObject != SFX_TRIGGER &&
            $menuSelectedObject != SONG_TRIGGER
        ) {
            obj.main_color = new wasm.GDColor(
                mainR,
                mainG,
                mainB,
                $menuMainColor.opacity * 255,
                $menuMainColor.blending
            );
        }
        setCheckedPreviewObject(state, obj);
    }
    $: {
        let obj = state.get_preview_object();
        obj.detail_color = new wasm.GDColor(
            detailR,
            detailG,
            detailB,
            $menuDetailColor.opacity * 255,
            $menuDetailColor.blending
        );
        setCheckedPreviewObject(state, obj);
    }

    $: {
        let obj = state.get_preview_object();
        obj.z_layer = $menuZLayer;
        setCheckedPreviewObject(state, obj);
    }
    $: {
        let obj = state.get_preview_object();
        obj.z_order = $menuZOrder;
        setCheckedPreviewObject(state, obj);
    }

    $: {
        if ($menuTabGroup == TabGroup.Delete && $canPlacePreview) {
            state.set_preview_visibility(false);
        } else {
            $selectedObject = null;
            state.deselect_object();
        }
    }

    let totalPlaceCooldown = 0;
    let totalDeleteCooldown = 0;
    let totalPlaceListener = db
        .ref("metaVariables/placeCooldown")
        .on("value", v => (totalPlaceCooldown = v.val()));
    let totalDeleteListener = db
        .ref("metaVariables/deleteCooldown")
        .on("value", v => (totalDeleteCooldown = v.val()));
    onDestroy(() => {
        totalPlaceListener();
        totalDeleteListener();
        placeCooldown.unsub();
        deleteCooldown.unsub();
    });

    const placeCooldown = SyncedCooldown.new(
        `userDetails/${$loginData.currentUserData!.user.uid}/epochNextPlace`,
        1.5
    );
    let {
        display: placeCooldownDisplay,
        finished: placeCooldownFinished,
        remaining: placeCooldownRemaining,
    } = placeCooldown;

    const deleteCooldown = SyncedCooldown.new(
        `userDetails/${$loginData.currentUserData!.user.uid}/epochNextDelete`,
        1.5
    );
    let {
        display: deleteCooldownDisplay,
        finished: deleteCooldownFinished,
        remaining: deleteCooldownRemaining,
    } = deleteCooldown;

    let pdButtonDisabled = false;
    $: pdButtonDisabled =
        $menuTabGroup != TabGroup.Delete
            ? !$placeCooldownFinished
            : !$deleteCooldownFinished;

    let playPlaceCooldownSound = !$placeCooldownFinished;
    $: {
        if ($placeCooldownFinished) {
            if (playPlaceCooldownSound) {
                playSound({ url: placeTimerFinishedSoundUrl });
            } else {
                playPlaceCooldownSound = true;
            }
        }
    }
    let playDeleteCooldownSound = !$deleteCooldownFinished;
    $: {
        if ($deleteCooldownFinished) {
            if (playDeleteCooldownSound) {
                playSound({ url: deleteTimerFinishedSoundUrl });
            } else {
                playDeleteCooldownSound = true;
            }
        }
    }
</script>

<div
    class="absolute flex flex-col justify-end w-full pointer-events-none place-menu"
    data-minimised={+$menuMinimized}
>
    <div
        class={cx({
            "flex justify-end text-white sm:flex-col pointer-events-all": true,
            "gap-2": !$menuMinimized,
        })}
    >
        <div
            class="grid flex-1 gap-2 menu-grid-container"
            data-minimised={+$menuMinimized}
        >
            <div
                class="flex flex-col items-center minimize menu-panel justify-evenly focus:outline focus:outline-1 focus:outline-offset-1"
            >
                <button
                    class="absolute flex w-full p-3 md:p-2 xs:p-1 flex-center"
                    on:click={() => {
                        $menuMinimized = !$menuMinimized;
                    }}
                    aria-label="Minimize Menu"
                >
                    <Minimize
                        class={cx({
                            "cursor-pointer": true,
                            "rotate-180": $menuMinimized,
                        })}
                    ></Minimize>
                </button>
            </div>

            <fieldset
                class="relative overflow-hidden tabs menu-panel"
                disabled={$menuMinimized}
            >
                <ul
                    class="absolute w-full h-full p-2 xs:p-1.5 flex overflow-y-hidden overflow-x-auto thin-scrollbar tab-options"
                    tabindex="-1"
                    on:wheel|passive={e => {
                        if (!e || !e.target) return;
                        e.currentTarget.scrollLeft += e.deltaY / 10;
                    }}
                    data-minimised={+$menuMinimized}
                >
                    {#if $menuTabGroup == TabGroup.Build}
                        {#each Object.entries(CATEGORY_ICONS) as [key, path]}
                            <li
                                class="relative h-full flex-center cursor-pointer flex-1 min-w-[64px] xs:min-w-[52px]"
                            >
                                <button
                                    class="z-20 w-full p-1 xs:p-1.5 h-full flex-center"
                                    on:click={() => {
                                        // @ts-expect-error its fine
                                        $menuBuildTab = key;
                                    }}
                                    aria-label={key}
                                >
                                    <Image
                                        src={path}
                                        alt={key}
                                        class="object-contain w-auto h-auto max-w-full max-h-full"
                                    ></Image>
                                </button>
                                {#if $menuBuildTab == key}
                                    <div class="sliding-selector"></div>
                                {/if}
                            </li>
                        {/each}
                    {:else if $menuTabGroup == TabGroup.Edit}
                        {#each Object.values(EditTab) as value}
                            {#if value != EditTab.Colors || (value == EditTab.Colors && showColorsTab)}
                                <li
                                    class="relative flex-1 h-full cursor-pointer flex-center"
                                >
                                    <button
                                        class="w-full h-full px-4 cursor-pointer xs:px-2 flex-center"
                                        on:click={() => {
                                            $menuEditTab = value;
                                        }}
                                        aria-label={value}
                                    >
                                        <h1
                                            class="z-20 text-2xl md:text-xl xs:text-sm font-pusab text-stroke"
                                        >
                                            {editTabName(value)}
                                        </h1>
                                    </button>
                                    {#if $menuEditTab == value}
                                        <div class="sliding-selector"></div>
                                    {/if}
                                </li>
                            {/if}
                        {/each}
                    {/if}
                </ul>

                <div
                    class="absolute flex justify-around w-24 h-full gap-3 p-2.5 md:p-2 tab-mini-icons"
                    data-minimised={+$menuMinimized}
                >
                    <RadialCooldown
                        max={totalPlaceCooldown}
                        remaining={$placeCooldownRemaining}
                    >
                        <Build class="w-full h-full stroke-[1.5]" />
                    </RadialCooldown>
                    <RadialCooldown
                        max={totalDeleteCooldown}
                        remaining={$deleteCooldownRemaining}
                    >
                        <Delete class="w-full h-full stroke-[1.5]" />
                    </RadialCooldown>
                </div>
            </fieldset>

            <fieldset
                class="w-full h-full overflow-hidden flex-center menu-panel side-menu"
                disabled={$menuMinimized}
            >
                <ul
                    class="absolute flex flex-col items-center w-full h-full gap-6 px-2 md:px-1.5 py-2 justify-evenly"
                >
                    <li class="w-full flex-center grow-0 shrink-0">
                        <button
                            class="w-full cursor-pointer"
                            on:click={() => {
                                $menuTabGroup = TabGroup.Build;
                            }}
                            aria-label="Build Tab"
                        >
                            <RadialCooldown
                                max={totalPlaceCooldown}
                                remaining={$placeCooldownRemaining}
                            >
                                <Build
                                    class={cx({
                                        "stroke-[1.5] w-full h-full": true,
                                        "opacity-30":
                                            $menuTabGroup != TabGroup.Build,
                                    })}
                                ></Build>
                            </RadialCooldown>
                        </button>
                    </li>
                    <li class="w-full flex-center grow-0 shrink-0">
                        <button
                            class="w-full cursor-pointer"
                            on:click={() => {
                                $menuTabGroup = TabGroup.Edit;
                            }}
                            aria-label="Edit Tab"
                        >
                            <Edit
                                class={cx({
                                    "stroke-[1.5] w-full h-full": true,
                                    "opacity-30":
                                        $menuTabGroup != TabGroup.Edit,
                                })}
                            ></Edit>
                        </button>
                    </li>
                    <li class="w-full flex-center grow-0 shrink-0">
                        <button
                            class="w-full cursor-pointer"
                            on:click={() => {
                                $menuTabGroup = TabGroup.Delete;
                            }}
                            aria-label="Delete Tab"
                        >
                            <RadialCooldown
                                max={totalDeleteCooldown}
                                remaining={$deleteCooldownRemaining}
                            >
                                <Delete
                                    class={cx({
                                        "stroke-[1.5] w-full h-full": true,
                                        "opacity-30":
                                            $menuTabGroup != TabGroup.Delete,
                                    })}
                                ></Delete>
                            </RadialCooldown>
                        </button>
                    </li>
                </ul>
            </fieldset>

            <div
                class="w-full h-full overflow-hidden rounded-lg buttons menu-panel"
            >
                <!-- 
                    the reason we dont use ifs statements to toggle the tabs is that it causes lag when switching back to the 
                    object tab as it has to add all the elements back to the dom
                    its more efficient to just set them to not be visible
                -->
                <ObjectsTab></ObjectsTab>
                <!-- EDIT TAB TRANSFORM + LAYERS -->
                {#if $menuTabGroup == TabGroup.Edit}
                    {#if $menuEditTab == EditTab.Transform}
                        <TransformTab bind:state></TransformTab>
                    {:else if $menuEditTab == EditTab.Layers}
                        <LayersTab></LayersTab>
                    {:else if $menuEditTab == EditTab.Colors}
                        {#if $menuSelectedObject == SFX_TRIGGER}
                            <SFXSongTab tabType="sfx"></SFXSongTab>
                        {:else if $menuSelectedObject == SONG_TRIGGER}
                            <SFXSongTab tabType="song"></SFXSongTab>
                        {:else}
                            <ColorsTab bind:state />
                        {/if}
                    {/if}
                {/if}

                {#if $menuTabGroup == TabGroup.Delete}
                    <DeleteTab {state} />
                {/if}
            </div>
        </div>

        <button
            class={cx({
                "self-end overflow-hidden pd-button cursor-pointer": true,
                "place-bttn-place": $menuTabGroup != TabGroup.Delete,
                "place-bttn-delete": $menuTabGroup == TabGroup.Delete,
                "bounce-active": !pdButtonDisabled,
            })}
            aria-label={`${$menuTabGroup != TabGroup.Delete ? "Place" : "Delete"} Button`}
            data-minimised={+$menuMinimized}
            on:click={() => {
                if ($menuTabGroup != TabGroup.Delete) {
                    addObject(state.get_preview_object(), k => {
                        canPlacePreview.set(true);
                        state.set_preview_visibility(false);
                        // state.add_object(k, state.get_preview_object());
                    });
                    // state.set_preview_visibility(false);
                    pdButtonDisabled = true;
                    $canPlacePreview = false;
                } else {
                    let k = state.get_selected_object_key();
                    let coord = state.get_selected_object_chunk();
                    if (k != null && coord != null) {
                        pdButtonDisabled = true;
                        removeObject(k, [coord.x, coord.y]);
                    }
                }
            }}
            disabled={pdButtonDisabled || $menuMinimized}
        >
            <div
                class="flex flex-col w-full h-full gap-1 py-4 text-5xl sm:flex-row sm:gap-2 flex-center md:text-4xl sm:text-4xl text-stroke"
            >
                <h1 class="font-pusab tab-text">
                    {#if $menuTabGroup != TabGroup.Delete}
                        Place
                    {:else}
                        Delete
                    {/if}
                </h1>
                {#if pdButtonDisabled}
                    <span
                        class="proportional-nums font-pusab"
                        style={$placeCooldownDisplay == "--:--" ||
                        $placeCooldownDisplay == "--:--"
                            ? "opacity: 0.5"
                            : ""}
                        >{$menuTabGroup != TabGroup.Delete
                            ? $placeCooldownDisplay
                            : $deleteCooldownDisplay}</span
                    >
                {/if}
            </div>
        </button>
    </div>
</div>

<style lang="postcss">
    .place-menu {
        height: 100svh;
        padding: 8px;
    }
    .place-menu[data-minimised="0"] {
        padding-bottom: 8px;
    }
    .place-menu[data-minimised="1"] {
        padding-bottom: 0px;
    }

    /* this element DOESNT include the place/delete button, thats part of the flex parent */
    .menu-grid-container {
        grid-template-areas:
            "minimize tabs"
            "side-menu buttons";
    }

    .menu-grid-container[data-minimised="0"] {
        grid-template-columns: 48px 1fr;
        grid-template-rows: 48px 200px;
    }
    .menu-grid-container[data-minimised="1"] {
        grid-template-columns: 48px 96px;
        grid-template-rows: 48px 0px;
    }

    @media screen(md) {
        .menu-grid-container[data-minimised="0"] {
            grid-template-columns: 44px 1fr;
            grid-template-rows: 44px 200px;
        }
        .menu-grid-container[data-minimised="1"] {
            grid-template-columns: 44px 96px;
            grid-template-rows: 44px 0px;
        }
    }

    @media screen(xs) {
        .menu-grid-container[data-minimised="0"] {
            grid-template-columns: 42px 1fr;
            grid-template-rows: 42px 200px;
        }
        .menu-grid-container[data-minimised="1"] {
            grid-template-columns: 42px 96px;
            grid-template-rows: 42px 0px;
        }
    }

    .tab-mini-icons .minimize {
        grid-area: minimize;
    }

    .tabs {
        grid-area: tabs;
    }

    .tab-options[data-minimised="0"] {
        opacity: 1;
    }
    .tab-options[data-minimised="1"] {
        opacity: 0;
    }

    .tab-mini-icons[data-minimised="0"] {
        opacity: 0;
    }
    .tab-mini-icons[data-minimised="1"] {
        opacity: 1;
    }

    .side-menu {
        grid-area: side-menu;
    }

    .buttons {
        grid-area: buttons;
    }

    .pd-button {
        width: 256px;
    }
    .pd-button[data-minimised="0"] {
        height: 256px;
    }
    .pd-button[data-minimised="1"] {
        height: 0px;
    }

    .place-bttn-place {
        border-radius: 16px;
        background: #7ade2d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #49851b inset,
            4px 4px 0px 8px #c6f249 inset;
    }
    .place-bttn-place:not(:disabled):active {
        border-radius: 16px;
        background: #61b91d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #3a6a16 inset,
            4px 4px 0px 8px #b2eb11 inset;
    }
    .place-bttn-place:disabled {
        cursor: not-allowed;
    }
    .place-bttn-place:disabled .tab-text {
        @apply text-xl;
        opacity: 0.5;
    }

    .place-bttn-delete {
        border-radius: 16px;
        background: #de2d30;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #851b1d inset,
            4px 4px 0px 8px #f24980 inset;
    }
    .place-bttn-delete:not(:disabled):active {
        border-radius: 16px;
        background: #b91d20;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #6a1617 inset,
            4px 4px 0px 8px #eb1158 inset;
    }
    .place-bttn-delete:disabled {
        cursor: not-allowed;
    }
    .place-bttn-delete:disabled .tab-text {
        @apply text-xl;
        opacity: 0.5;
    }

    @media screen(md) {
        .pd-button {
            width: 180px;
        }
    }

    @media screen(sm) {
        .pd-button {
            width: 100%;
        }
        .pd-button[data-minimised="0"] {
            height: 64px;
        }
    }
</style>
