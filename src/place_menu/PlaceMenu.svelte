<script lang="ts">
    import * as wasm from "wasm-lib";

    import { default as cx } from "classnames";

    import { colors, objects } from "shared-lib/gd";
    import { CATEGORY_ICONS } from "../gd/object";

    import Image from "../components/Image.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";

    import { IconBox as Build } from "@tabler/icons-svelte";
    import { IconAdjustmentsHorizontal as Edit } from "@tabler/icons-svelte";
    import { IconTrash as Delete } from "@tabler/icons-svelte";
    import { IconCaretDownFilled as Minimize } from "@tabler/icons-svelte";

    import {
        TabGroup,
        editorSettings,
        selectedObject,
        colors as colorsStore,
        menuSelectedObject,
        menuMainColor,
        menuDetailColor,
        menuZLayer,
        menuZOrder,
        menuTabGroup,
        menuMinimized,
        menuBuildTab,
        menuEditTab,
    } from "../stores";
    import { addObject, removeObject } from "../firebase/object";
    import { useIsOverflowing } from "../utils/document";
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
    import { fade, type TransitionConfig } from "svelte/transition";
    import { COLOR_TRIGGERS } from "shared-lib/nexusgen";

    export let state: wasm.State;

    const minimizeAnimDur = 0.5;

    $: {
        if (COLOR_TRIGGERS.includes($menuSelectedObject)) {
            $menuMainColor.blending = false;
            $menuDetailColor.blending = false;
            $menuMainColor.opacity = 1;
            $menuDetailColor.opacity = 1;
        }
    }
    $: {
        let [mr, mg, mb] =
            colors.list[$menuMainColor.hue].palette[$menuMainColor.y][
                $menuMainColor.x
            ];

        let obj = state.get_preview_object();
        obj.main_color = new wasm.GDColor(
            mr,
            mg,
            mb,
            $menuMainColor.opacity * 255,
            $menuMainColor.blending
        );
        state.set_preview_object(obj);
    }
    $: {
        let [mr, mg, mb] =
            colors.list[$menuDetailColor.hue].palette[$menuDetailColor.y][
                $menuDetailColor.x
            ];

        let obj = state.get_preview_object();
        obj.detail_color = new wasm.GDColor(
            mr,
            mg,
            mb,
            $menuDetailColor.opacity * 255,
            $menuDetailColor.blending
        );
        state.set_preview_object(obj);
    }
    $: {
        let obj = state.get_preview_object();
        obj.z_layer = $menuZLayer;
        state.set_preview_object(obj);
    }
    $: {
        let obj = state.get_preview_object();
        obj.z_order = $menuZOrder;
        state.set_preview_object(obj);
    }
    $: {
        let obj = state.get_preview_object();

        let [oldPx, oldPy] = getTransformedPlaceOffset(obj);
        obj.id = $menuSelectedObject;
        let [newPx, newPy] = getTransformedPlaceOffset(obj);

        obj.x = obj.x + oldPx - newPx;
        obj.y = obj.y + oldPy - newPy;

        state.set_preview_object(obj);
    }
    $: {
        if ($menuTabGroup == TabGroup.Delete) {
            state.set_preview_visibility(false);
        } else {
            $selectedObject = null;
            state.deselect_object();
        }
    }

    $: canSelectByTab = $menuMinimized ? -1 : 0;

    const handleEditorSettings = (settings: typeof $editorSettings) => {
        state.set_show_collidable(settings.showCollidable);
        state.set_hide_triggers(settings.hideTriggers);
    };
    $: handleEditorSettings($editorSettings);

    const handleColorStore = (settings: typeof $colorsStore) => {
        state.set_bg_color(settings.bg.r, settings.bg.g, settings.bg.b);
        state.set_ground1_color(
            settings.ground1.r,
            settings.ground1.g,
            settings.ground1.b
        );
        state.set_ground2_color(
            settings.ground2.r,
            settings.ground2.g,
            settings.ground2.b
        );
    };
    $: handleColorStore($colorsStore);
</script>

<div
    class="absolute flex flex-col justify-end w-full pointer-events-none place-menu"
    data-minimised={+$menuMinimized}
>
    <div
        class="flex justify-end gap-2 text-white sm:flex-col pointer-events-all"
    >
        <div
            class="grid flex-1 gap-2 menu-grid-container"
            data-minimised={+$menuMinimized}
        >
            <div
                class="flex flex-col items-center minimize menu-panel justify-evenly focus:outline focus:outline-1 focus:outline-offset-1"
            >
                <button
                    class="absolute w-full p-3"
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

            <div class="relative overflow-hidden tabs menu-panel">
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
                                        // @ts-ignore
                                        $menuBuildTab = key;
                                    }}
                                    tabindex={canSelectByTab}
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
                            <li
                                class="relative flex-1 h-full cursor-pointer flex-center"
                            >
                                <button
                                    class="w-full h-full px-4 cursor-pointer xs:px-2 flex-center"
                                    on:click={() => {
                                        $menuEditTab = value;
                                    }}
                                    tabindex={canSelectByTab}
                                    aria-label={value}
                                >
                                    <h1
                                        class="z-20 text-2xl md:text-xl xs:text-sm font-pusab text-stroke"
                                    >
                                        {value}
                                    </h1>
                                </button>
                                {#if $menuEditTab == value}
                                    <div class="sliding-selector"></div>
                                {/if}
                            </li>
                        {/each}
                    {/if}
                </ul>

                <div
                    class="absolute flex justify-around w-24 h-full gap-3 p-2.5 tab-mini-icons"
                    data-minimised={+$menuMinimized}
                >
                    <Build class="stroke-1 w-full h-full"></Build>
                    <Delete class="stroke-1 w-full h-full"></Delete>
                </div>
            </div>

            <div
                class="w-full h-full overflow-hidden flex-center menu-panel side-menu"
            >
                <ul
                    class="absolute flex flex-col items-center w-full h-full gap-6 px-2 py-2 justify-evenly"
                >
                    <li class="w-full flex-center grow-0 shrink-0">
                        <button
                            class="w-full cursor-pointer"
                            on:click={() => {
                                $menuTabGroup = TabGroup.Build;
                            }}
                            tabindex={canSelectByTab}
                            aria-label="Build Tab"
                        >
                            <Build
                                class={cx({
                                    "stroke-1 w-full h-full": true,
                                    "opacity-30":
                                        $menuTabGroup != TabGroup.Build,
                                })}
                            ></Build>
                        </button>
                    </li>
                    <li class="w-full flex-center grow-0 shrink-0">
                        <button
                            class="w-full cursor-pointer"
                            on:click={() => {
                                $menuTabGroup = TabGroup.Edit;
                            }}
                            tabindex={canSelectByTab}
                            aria-label="Edit Tab"
                        >
                            <Edit
                                class={cx({
                                    "stroke-1 w-full h-full": true,
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
                            tabindex={canSelectByTab}
                            aria-label="Delete Tab"
                        >
                            <Delete
                                class={cx({
                                    "stroke-1 w-full h-full": true,
                                    "opacity-30":
                                        $menuTabGroup != TabGroup.Delete,
                                })}
                            ></Delete>
                        </button>
                    </li>
                </ul>
            </div>

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
                        <ColorsTab></ColorsTab>
                    {/if}
                {/if}

                {#if $menuTabGroup == TabGroup.Delete}
                    <div
                        class="w-full h-full p-4 text-4xl text-center md:text-3x sm:text-2x xs:text-xl flex-center font-pusab text-stroke"
                    >
                        Select an object to delete it!
                    </div>
                {/if}
            </div>
        </div>

        <button
            class={cx({
                "self-end overflow-hidden bounce-active pd-button": true,
                "place-bttn-place": $menuTabGroup != TabGroup.Delete,
                "place-bttn-delete": $menuTabGroup == TabGroup.Delete,
            })}
            tabindex={canSelectByTab}
            aria-label={`${$menuTabGroup != TabGroup.Delete ? "Place" : "Delete"} Button`}
            data-minimised={+$menuMinimized}
            on:click={() => {
                if ($menuTabGroup != TabGroup.Delete) {
                    addObject(state.get_preview_object());
                    state.set_preview_visibility(false);
                } else {
                    let k = state.get_selected_object_key();
                    let coord = state.get_selected_object_chunk();
                    if (k != null && coord != null) {
                        removeObject(k, [coord.x, coord.y]);
                    }
                }
            }}
        >
            <div class="w-full h-full py-4 overflow-hidden">
                <h1
                    class="w-full h-full overflow-hidden text-5xl md:text-4xl sm:text-4xl font-pusab text-stroke flex-center"
                >
                    {#if $menuTabGroup != TabGroup.Delete}
                        Place
                    {:else}
                        Delete
                    {/if}
                </h1>
            </div>
        </button>
    </div>
</div>

<style lang="postcss">
    /* https://www.reddit.com/r/nextjs/comments/11g3znz/comment/janib69/?utm_source=share&utm_medium=web2x&context=3 */
    .place-menu {
        height: 100vh;
        height: calc(var(--vh, 1vh) * 100);
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
    .place-bttn-place:active {
        border-radius: 16px;
        background: #61b91d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #3a6a16 inset,
            4px 4px 0px 8px #b2eb11 inset;
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
    .place-bttn-delete:active {
        border-radius: 16px;
        background: #b91d20;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #6a1617 inset,
            4px 4px 0px 8px #eb1158 inset;
    }

    @media screen(md) {
        .pd-button {
            width: 180px;
        }

        .menu-grid-container {
            grid-template-columns: 56px auto;
        }
    }

    @media screen(sm) {
        .pd-button {
            width: 100%;
        }
        .pd-button[data-minimised="0"] {
            height: 64px;
        }

        .menu-grid-container {
            grid-template-columns: 44px auto;
        }
    }
</style>
