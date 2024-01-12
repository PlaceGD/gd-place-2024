<script lang="ts">
    import * as wasm from "wasm-lib";

    import { default as cx } from "classnames";
    import {
        AnimateSharedLayout,
        Motion,
        useAnimation,
        useReducedMotion,
    } from "svelte-motion";
    import colors from "../gd/colors.json";

    import LocalSettings from "../utils/LocalSettings";

    import Image from "../components/Image.svelte";
    import Animate from "../components/Animate.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";

    import Build from "./icons/build.svg";
    import Edit from "./icons/edit.svg";
    import Delete from "./icons/delete.svg";
    import Minimize from "./icons/caret.svg";
    import {
        CATEGORY_ICONS,
        OBJECT_SETTINGS,
        MAIN_DETAIL_TEX_RATIOS,
    } from "../gd/object";
    import SlidingSelector from "../components/SlidingSelector.svelte";
    import { isOverflow } from "../util";
    import { EditTab, TRANSFORM_BUTTONS } from "./edit_tab";
    import { DEBUG } from "../main";
    import SpriteSheet from "../utils/SpriteSheet";

    import ColorTab from "./ColorTab.svelte";
    import { TabGroup, menuSettings } from "../stores";
    import { addObject, deleteObject } from "../firebase";
    import LayersTab from "./LayersTab.svelte";
    import { tweened } from "svelte/motion";
    import { bounceOut } from "svelte/easing";

    export let state: wasm.StateWrapper | null;

    const minimizeAnimDur = 0.5;
    const shouldReducedMotion = useReducedMotion();

    let tabsPanel: HTMLUListElement;
    let isTabsPanelOverflow: boolean = false;

    // let selectedMainColor = { hue: 0, x: 0, y: 0, blending: false };
    // let selectedDetailColor = { hue: 0, x: 0, y: 0, blending: false };

    $: {
        if (state != null) {
            let [mr, mg, mb] =
                colors.list[$menuSettings.selectedMainColor.hue].palette[
                    $menuSettings.selectedMainColor.y
                ][$menuSettings.selectedMainColor.x];
            let m_opacity = $menuSettings.selectedMainColor.opacity;
            let m_blending = $menuSettings.selectedMainColor.blending;
            let [dr, dg, db] =
                colors.list[$menuSettings.selectedDetailColor.hue].palette[
                    $menuSettings.selectedDetailColor.y
                ][$menuSettings.selectedDetailColor.x];
            let d_opacity = $menuSettings.selectedDetailColor.opacity;
            let d_blending = $menuSettings.selectedDetailColor.blending;

            let obj = state.get_preview_object();

            obj.main_color = new wasm.GDColor(
                mr,
                mg,
                mb,
                m_opacity * 255,
                m_blending
            );
            obj.detail_color = new wasm.GDColor(
                dr,
                dg,
                db,
                d_opacity * 255,
                d_blending
            );
            obj.id = $menuSettings.selectedObject;
            obj.z_layer = $menuSettings.zLayer;
            obj.z_order = $menuSettings.zOrder;

            state.set_preview_object(obj);
        }
    }
</script>

<svelte:window
    on:resize={() => {
        if (tabsPanel && isOverflow(tabsPanel)) {
            isTabsPanelOverflow = true;
        } else {
            isTabsPanelOverflow = false;
        }
    }}
/>

<Animate
    easing="easeInOut"
    duration={minimizeAnimDur}
    initial={{
        padding: "8px",
        paddingBottom: "8px",
    }}
    definition={{
        isMinimized: {
            paddingBottom: 0,
        },
    }}
    conditions={{
        isMinimized: $menuSettings.isMinimized,
    }}
    let:motion
>
    <div
        class="absolute flex flex-col justify-end w-full h-full pointer-events-none pb"
        use:motion
    >
        <div
            class="flex justify-end gap-2 text-white sm:flex-col pointer-events-all"
        >
            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={// {
                //     // adds 8px to account for 8px scroll bar if overflowing
                //     gridTemplateRows: `${
                //         isTabsPanelOverflow ? "56px" : "48px"
                //     } 200px`,
                // }
                ["gridTemplateRows"]}
                definition={{
                    isMinimized: {
                        gridTemplateRows: `${
                            isTabsPanelOverflow ? "56px" : "48px"
                        } 0px`,
                    },
                }}
                conditions={{
                    isMinimized: $menuSettings.isMinimized,
                }}
                let:motion
            >
                <div
                    class={cx({
                        "flex-1 menu-grid-container": true,
                        tabOverflow: isTabsPanelOverflow,
                    })}
                    use:motion
                >
                    <div
                        class="flex flex-col items-center minimize menu-panel justify-evenly"
                    >
                        <button
                            class="absolute w-full h-full p-4 flex-center"
                            on:click={() => {
                                $menuSettings.isMinimized =
                                    !$menuSettings.isMinimized;
                            }}
                        >
                            <Minimize
                                class={cx({
                                    "cursor-pointer": true,
                                    "rotate-180": $menuSettings.isMinimized,
                                })}
                            ></Minimize>
                        </button>
                    </div>

                    <Animate
                        easing="easeInOut"
                        duration={minimizeAnimDur}
                        initial={{
                            width: "auto",
                        }}
                        definition={{
                            isMinimized: {
                                width: 96,
                            },
                        }}
                        conditions={{
                            isMinimized: $menuSettings.isMinimized,
                        }}
                        let:motion
                    >
                        <div class="relative tabs menu-panel" use:motion>
                            <Animate
                                easing="easeInOut"
                                duration={minimizeAnimDur}
                                initial={{
                                    opacity: 0,
                                }}
                                definition={{
                                    isMinimized: {
                                        opacity: 1,
                                    },
                                }}
                                conditions={{
                                    isMinimized: !$menuSettings.isMinimized,
                                }}
                                let:motion
                            >
                                <ul
                                    bind:this={tabsPanel}
                                    class={"absolute w-full h-full p-2 flex justify-evenly overflow-y-hidden overflow-x-auto thin-scrollbar"}
                                    use:motion
                                    on:wheel={e => {
                                        if (!e || !e.target) return;
                                        e.currentTarget.scrollLeft +=
                                            e.deltaY / 10;
                                    }}
                                >
                                    <AnimateSharedLayout>
                                        {#if $menuSettings.selectedGroup == TabGroup.Build}
                                            {#each Object.entries(CATEGORY_ICONS) as [key, path]}
                                                <li
                                                    class="relative h-full flex-center cursor-pointer flex-1 min-w-[64px]"
                                                >
                                                    <button
                                                        class="z-20 w-full h-full py-1 flex-center"
                                                        on:click={() => {
                                                            $menuSettings.selectedBuildTab =
                                                                key;
                                                        }}
                                                    >
                                                        <Image
                                                            src={path}
                                                            alt={key}
                                                            class="object-contain w-auto h-auto max-w-full max-h-full"
                                                        ></Image>
                                                    </button>
                                                    {#if $menuSettings.selectedBuildTab == key}
                                                        <SlidingSelector
                                                            layoutId="selected-build-tab"
                                                        ></SlidingSelector>
                                                    {/if}
                                                </li>
                                            {/each}
                                        {:else if $menuSettings.selectedGroup == TabGroup.Edit}
                                            {#each Object.values(EditTab) as value}
                                                <li
                                                    class="relative flex-1 h-full cursor-pointer flex-center"
                                                >
                                                    <button
                                                        class="w-full h-full px-4 cursor-pointer flex-center"
                                                        on:click={() => {
                                                            $menuSettings.selectedEditTab =
                                                                value;
                                                        }}
                                                    >
                                                        <h1
                                                            class="z-20 text-2xl font-pusab text-stroke"
                                                        >
                                                            {value}
                                                        </h1>
                                                    </button>
                                                    {#if $menuSettings.selectedEditTab == value}
                                                        <SlidingSelector
                                                            layoutId="selected-edit-tab"
                                                        ></SlidingSelector>
                                                    {/if}
                                                </li>
                                            {/each}
                                        {/if}
                                    </AnimateSharedLayout>
                                </ul>
                            </Animate>

                            <Animate
                                easing="easeInOut"
                                duration={minimizeAnimDur}
                                initial={{
                                    opacity: 0,
                                }}
                                definition={{
                                    isMinimized: {
                                        opacity: 1,
                                    },
                                }}
                                conditions={{
                                    isMinimized: $menuSettings.isMinimized,
                                }}
                                let:motion
                            >
                                <div
                                    class="absolute w-24 h-full gap-3 p-3 flex-center"
                                    use:motion
                                >
                                    <Build></Build>
                                    <Delete></Delete>
                                </div>
                            </Animate>
                        </div>
                    </Animate>

                    <!-- class="flex flex-col items-center w-full h-full justify-evenly menu-panel g-8" -->
                    <div
                        class="w-full h-full overflow-hidden flex-center menu-panel side-menu"
                    >
                        <ul
                            class="absolute flex flex-col items-center justify-between w-full h-full gap-6 px-3 py-4"
                        >
                            <li class="w-full flex-center grow-0 shrink-0">
                                <button
                                    class={cx({
                                        "w-full cursor-pointer ": true,
                                        "opacity-30":
                                            $menuSettings.selectedGroup !=
                                            TabGroup.Build,
                                    })}
                                    on:click={() => {
                                        $menuSettings.selectedGroup =
                                            TabGroup.Build;
                                    }}
                                >
                                    <Build></Build>
                                </button>
                            </li>
                            <li class="w-full flex-center grow-0 shrink-0">
                                <button
                                    class={cx({
                                        "w-full cursor-pointer": true,
                                        "opacity-30":
                                            $menuSettings.selectedGroup !=
                                            TabGroup.Edit,
                                    })}
                                    on:click={() => {
                                        $menuSettings.selectedGroup =
                                            TabGroup.Edit;
                                    }}
                                >
                                    <Edit></Edit>
                                </button>
                            </li>
                            <li class="w-full flex-center grow-0 shrink-0">
                                <button
                                    class={cx({
                                        "w-full cursor-pointer": true,
                                        "opacity-30":
                                            $menuSettings.selectedGroup !=
                                            TabGroup.Delete,
                                    })}
                                    on:click={() => {
                                        $menuSettings.selectedGroup =
                                            TabGroup.Delete;
                                    }}
                                >
                                    <Delete></Delete>
                                </button>
                            </li>
                        </ul>
                    </div>

                    <div class="w-full h-full rounded-lg buttons menu-panel">
                        <!-- 
                            the reason we dont use ifs statements to toggle the tabs is that it causes lag when switching back to the 
                            object tab as it has to add all the elements back to the dom
                            its more efficient to just set them to not be visible, but that means we need conditional grids for this element
                        -->
                        <ul
                            class={cx({
                                "w-full h-full overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar object-grid-container": true,
                                "!hidden":
                                    $menuSettings.selectedGroup !=
                                    TabGroup.Build,
                            })}
                        >
                            <!-- BUILD TAB -->
                            {#each Object.entries(OBJECT_SETTINGS) as [id, obj], i}
                                <li
                                    class={cx({
                                        "relative w-16 h-16": true,
                                        hidden:
                                            $menuSettings.selectedBuildTab !=
                                            obj.category,
                                    })}
                                >
                                    <button
                                        class={"absolute w-full h-full p-3 z-20"}
                                        on:click={() => {
                                            $menuSettings.selectedObject = id;
                                        }}
                                    >
                                        {#if DEBUG}
                                            <span
                                                class="absolute text-red font-lg bottom-3/4 right-1/2"
                                            >
                                                {id}
                                            </span>
                                        {/if}
                                        <div
                                            class="relative w-full h-full flex-center"
                                        >
                                            <Image
                                                class="absolute object-contain max-w-full max-h-full"
                                                src={`/textures/main/${id}.png`}
                                                lazyLoad
                                                skeleton
                                            ></Image>
                                            <!-- <Image
                                                class="absolute object-contain"
                                                src={`/textures/detail/${id}.png`}
                                                lazyLoad
                                            ></Image> -->

                                            <!-- <Image
                                                class="absolute object-contain"
                                                src={buttons_spr_sheet.mainSpriteFromId(
                                                    obj.id
                                                )}
                                                lazyLoad
                                                skeleton
                                            ></Image> -->
                                        </div>
                                    </button>
                                    {#if $menuSettings.selectedObject == id}
                                        <span
                                            class={"absolute w-full h-full sliding-selector"}
                                        ></span>
                                    {/if}
                                </li>
                            {/each}
                        </ul>
                        <!-- EDIT TAB TRANSFORM + LAYERS -->
                        {#if $menuSettings.selectedGroup == TabGroup.Edit}
                            {#if $menuSettings.selectedEditTab == EditTab.Transform}
                                <ul
                                    class="w-full h-full overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar object-grid-container gap-4"
                                >
                                    {#each TRANSFORM_BUTTONS as button, i (i)}
                                        <li class="w-16 h-16">
                                            <button
                                                class={"flex-center w-full h-full p-3 z-20 rounded-md bg-button-green bounce-active"}
                                                on:click={() => {
                                                    if (state == null) return;
                                                    let obj =
                                                        state.get_preview_object();
                                                    button.cb(obj);
                                                    state.set_preview_object(
                                                        obj
                                                    );
                                                }}
                                            >
                                                <Image
                                                    class={"object-cover w-12 h-12"}
                                                    src={`/assets/ui/edit/${button.image}.svg`}
                                                    style={`transform: rotate(${
                                                        button.angle
                                                    }deg)${
                                                        button.flipped
                                                            ? " scaleX(-1)"
                                                            : ""
                                                    }`}
                                                    lazyLoad
                                                    skeleton
                                                />
                                            </button>
                                        </li>
                                    {/each}
                                </ul>
                            {:else if $menuSettings.selectedEditTab == EditTab.Layers}
                                <LayersTab
                                    bind:selectedLayer={$menuSettings.zLayer}
                                    bind:zOrder={$menuSettings.zOrder}
                                ></LayersTab>
                            {:else if $menuSettings.selectedEditTab == EditTab.Colors}
                                <ColorTab
                                    bind:currentMainColor={$menuSettings.selectedMainColor}
                                    bind:currentDetailColor={$menuSettings.selectedDetailColor}
                                ></ColorTab>
                            {/if}
                        {/if}

                        {#if $menuSettings.selectedGroup == TabGroup.Delete}
                            <div
                                class="w-full h-full text-4xl flex-center font-pusab text-stroke"
                            >
                                Select an object to delete it 😍
                            </div>
                        {/if}
                    </div>
                </div>
            </Animate>

            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={["minHeight", "height"]}
                definition={{
                    isMinimized: {
                        minHeight: 0,
                        height: 0,
                    },
                }}
                conditions={{
                    isMinimized: $menuSettings.isMinimized,
                }}
                let:motion
            >
                <button
                    class={cx({
                        "self-end overflow-hidden bounce-active": true,
                        "place-bttn-place":
                            $menuSettings.selectedGroup != TabGroup.Delete,
                        "place-bttn-delete":
                            $menuSettings.selectedGroup == TabGroup.Delete,
                    })}
                    use:motion
                    on:click={() => {
                        if (state != null) {
                            if (
                                $menuSettings.selectedGroup != TabGroup.Delete
                            ) {
                                addObject(state.get_preview_object());
                                state.set_preview_visibility(false);
                            } else {
                                let k = state.get_selected_object_key();
                                let coord = state.get_selected_object_chunk();
                                if (k != null && coord != null) {
                                    deleteObject(k, [coord.x, coord.y]);
                                }
                            }
                        }
                    }}
                >
                    <div class="w-full h-full py-4 overflow-hidden">
                        <h1
                            class="w-full h-full overflow-hidden text-5xl md:text-4xl sm:text-4xl font-pusab text-stroke flex-center"
                        >
                            {#if $menuSettings.selectedGroup != TabGroup.Delete}
                                Place
                            {:else}
                                Delete
                            {/if}
                        </h1>
                    </div>
                </button>
            </Animate>
        </div>
    </div>
</Animate>

<style lang="postcss">
    .object-grid-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, 64px);
        padding: 16px;
        justify-content: space-between;
    }

    .menu-grid-container {
        display: grid;
        grid-template-columns: 64px auto;
        grid-template-rows: 48px 200px;
        gap: 8px 8px;
        grid-template-areas:
            "minimize tabs"
            "side-menu buttons";
    }

    .menu-grid-container.tabOverflow {
        grid-template-rows: 56px 200px !important;
    }

    .minimize {
        grid-area: minimize;
    }

    .tabs {
        grid-area: tabs;
    }

    .side-menu {
        grid-area: side-menu;
    }

    .buttons {
        grid-area: buttons;
    }

    .place-bttn-place {
        height: 256px;
        min-height: 256px;
        width: 256px;
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
        height: 256px;
        min-height: 256px;
        width: 256px;
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
        .place-bttn-place,
        .place-bttn-delete {
            width: 180px;
        }
    }

    @media screen(sm) {
        .place-bttn-place,
        .place-bttn-delete {
            height: 64px !important;
            min-height: 64px !important;
            width: 100%;
        }
    }
</style>
