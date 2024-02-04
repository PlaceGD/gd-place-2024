<script lang="ts">
    import * as wasm from "wasm-lib";

    import { default as cx } from "classnames";
    import { AnimateSharedLayout } from "svelte-motion";

    import { colors } from "shared-lib/gd";
    import { CATEGORY_ICONS } from "../gd/object";

    import Image from "../components/Image.svelte";
    import Animate from "../components/Animate.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import SlidingSelector from "../components/SlidingSelector.svelte";

    import Build from "../icons/build.svg";
    import Edit from "../icons/edit.svg";
    import Delete from "../icons/delete.svg";
    import Minimize from "../icons/caret.svg";

    import { TabGroup, menuSettings } from "../stores";
    import { addObject, removeObject } from "../firebase/object";
    import { useIsOverflowing } from "../utils/document";
    import { DEBUG } from "../utils/debug";
    import SpriteSheet from "../utils/spritesheet";
    import LocalSettings from "../utils/local_settings";

    import { EditTab, TRANSFORM_BUTTONS, WidgetType } from "./edit/edit_tab";
    import ColorsTab from "./edit/ColorsTab.svelte";
    import LayersTab from "./edit/LayersTab.svelte";
    import TransformTab from "./edit/TransformTab.svelte";
    import ObjectsTab from "./objects/ObjectsTab.svelte";

    export let state: wasm.StateWrapper;

    const minimizeAnimDur = 0.5;

    const { isOverflowing: isTabsPanelOverflowing, element: tabsPanel } =
        useIsOverflowing();

    $: {
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

        // console.log($menuSettings.selectedMainColor == $menuSettings.selectedDetailColor);

        state.set_preview_object(obj);
    }

    import config from "../../tailwind.config";

    // some bs to fix tiny little responsive anim issue
    // that honestly no one would see but perfectionism :3
    let placeDeleteBttnAnim: Animate | null;
    let hasResetSm = false;
    let hasResetLg = false;
    window.addEventListener("resize", () => {
        let isSm = window.matchMedia(
            `(max-width: ${
                (config.theme?.screens as Record<string, any>).sm.max
            })`
        );
        let isNotSm = window.matchMedia(
            `(min-width: ${
                (config.theme?.screens as Record<string, any>).sm.max
            })`
        );

        if (placeDeleteBttnAnim == null) return;

        if (isSm.matches && !hasResetSm) {
            placeDeleteBttnAnim?.resetIntialStyles();
            hasResetSm = true;
            hasResetLg = false;
        } else if (isNotSm.matches && !hasResetLg) {
            placeDeleteBttnAnim?.resetIntialStyles();
            hasResetLg = true;
            hasResetSm = false;
        }
    });

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<Animate
    easing="easeInOut"
    duration={minimizeAnimDur}
    initial={{
        paddingBottom: "8px",
    }}
    to={{
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
        class="absolute flex flex-col justify-end w-full px-2 pt-2 pointer-events-none place-menu"
        use:motion
    >
        <div
            class="flex justify-end gap-2 text-white sm:flex-col pointer-events-all"
        >
            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                from={{
                    gridTemplateRows: `${
                        $isTabsPanelOverflowing ? "56px" : "48px"
                    } 200px`,
                }}
                to={{
                    isMinimized: {
                        gridTemplateRows: "48px 0px",
                    },
                }}
                conditions={{
                    isMinimized: $menuSettings.isMinimized,
                }}
                let:motion
            >
                <!-- 
                    for some reason on mobile devices getting the computed style for `grid-template-rows` returns `none` for... a while?
                    but if it's set with an inline style tag it will be immediately ready to query.
                    unfortunately this means all responsive styles now need !important :3
                -->
                <div class="grid flex-1 gap-2 menu-grid-container" use:motion>
                    <div
                        class="flex flex-col items-center minimize menu-panel justify-evenly focus:outline focus:outline-1 focus:outline-offset-1"
                    >
                        <button
                            class="absolute w-full p-3"
                            on:click={() => {
                                $menuSettings.isMinimized =
                                    !$menuSettings.isMinimized;
                            }}
                            aria-label="Minimize Menu"
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
                        to={{
                            isMinimized: {
                                width: 96,
                            },
                        }}
                        conditions={{
                            isMinimized: $menuSettings.isMinimized,
                        }}
                        let:motion
                    >
                        <div
                            class="relative overflow-hidden tabs menu-panel"
                            use:motion
                        >
                            <Animate
                                easing="easeInOut"
                                duration={minimizeAnimDur}
                                initial={{
                                    opacity: 0,
                                }}
                                to={{
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
                                    class="absolute w-full h-full p-2 xs:p-1.5 flex overflow-y-hidden overflow-x-auto thin-scrollbar"
                                    tabindex="-1"
                                    use:tabsPanel
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
                                                    class="relative h-full flex-center cursor-pointer flex-1 min-w-[64px] xs:min-w-[52px]"
                                                >
                                                    <button
                                                        class="z-20 w-full p-1 xs:p-1.5 h-full flex-center"
                                                        on:click={() => {
                                                            $menuSettings.selectedBuildTab =
                                                                key;
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
                                                        class="w-full h-full px-4 cursor-pointer xs:px-2 flex-center"
                                                        on:click={() => {
                                                            $menuSettings.selectedEditTab =
                                                                value;
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
                                to={{
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
                                    class="absolute flex justify-around w-24 h-full gap-3 p-2.5"
                                    use:motion
                                >
                                    <Build></Build>
                                    <Delete></Delete>
                                </div>
                            </Animate>
                        </div>
                    </Animate>

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
                                        $menuSettings.selectedGroup =
                                            TabGroup.Build;
                                    }}
                                    tabindex={canSelectByTab}
                                    aria-label="Build Tab"
                                >
                                    <Build
                                        class={cx({
                                            "opacity-30":
                                                $menuSettings.selectedGroup !=
                                                TabGroup.Build,
                                        })}
                                    ></Build>
                                </button>
                            </li>
                            <li class="w-full flex-center grow-0 shrink-0">
                                <button
                                    class="w-full cursor-pointer"
                                    on:click={() => {
                                        $menuSettings.selectedGroup =
                                            TabGroup.Edit;
                                    }}
                                    tabindex={canSelectByTab}
                                    aria-label="Edit Tab"
                                >
                                    <Edit
                                        class={cx({
                                            "opacity-30":
                                                $menuSettings.selectedGroup !=
                                                TabGroup.Edit,
                                        })}
                                    ></Edit>
                                </button>
                            </li>
                            <li class="w-full flex-center grow-0 shrink-0">
                                <button
                                    class="w-full cursor-pointer"
                                    on:click={() => {
                                        $menuSettings.selectedGroup =
                                            TabGroup.Delete;
                                    }}
                                    tabindex={canSelectByTab}
                                    aria-label="Delete Tab"
                                >
                                    <Delete
                                        class={cx({
                                            "opacity-30":
                                                $menuSettings.selectedGroup !=
                                                TabGroup.Delete,
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
                        {#if $menuSettings.selectedGroup == TabGroup.Edit}
                            {#if $menuSettings.selectedEditTab == EditTab.Transform}
                                <TransformTab bind:state></TransformTab>
                            {:else if $menuSettings.selectedEditTab == EditTab.Layers}
                                <LayersTab></LayersTab>
                            {:else if $menuSettings.selectedEditTab == EditTab.Colors}
                                <ColorsTab></ColorsTab>
                            {/if}
                        {/if}

                        {#if $menuSettings.selectedGroup == TabGroup.Delete}
                            <div
                                class="w-full h-full p-4 text-4xl text-center md:text-3x sm:text-2x xs:text-xl flex-center font-pusab text-stroke"
                            >
                                Select an object to delete it!
                            </div>
                        {/if}
                    </div>
                </div>
            </Animate>

            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={["minHeight", "height"]}
                to={{
                    isMinimized: {
                        minHeight: 0,
                        height: 0,
                    },
                }}
                conditions={{
                    isMinimized: $menuSettings.isMinimized,
                }}
                let:motion
                bind:this={placeDeleteBttnAnim}
            >
                <button
                    class={cx({
                        "self-end overflow-hidden bounce-active": true,
                        "place-bttn-place":
                            $menuSettings.selectedGroup != TabGroup.Delete,
                        "place-bttn-delete":
                            $menuSettings.selectedGroup == TabGroup.Delete,
                    })}
                    tabindex={canSelectByTab}
                    aria-label={`${$menuSettings.selectedGroup != TabGroup.Delete ? "Place" : "Delete"} Button`}
                    use:motion
                    on:click={() => {
                        if ($menuSettings.selectedGroup != TabGroup.Delete) {
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
    /* https://www.reddit.com/r/nextjs/comments/11g3znz/comment/janib69/?utm_source=share&utm_medium=web2x&context=3 */
    .place-menu {
        height: 100vh;
        height: calc(var(--vh, 1vh) * 100);
    }

    /* this element DOESNT include the place/delete button, thats part of the flex parent */
    .menu-grid-container {
        grid-template-columns: 48px auto;
        grid-template-areas:
            "minimize tabs"
            "side-menu buttons";
        grid-template-rows: 48px 200px;
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

    .place-bttn-place,
    .place-bttn-delete {
        height: 256px;
        min-height: 256px;
        width: 256px;
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
        .place-bttn-place,
        .place-bttn-delete {
            height: 256px;
            min-height: 256px;
            width: 180px;
        }

        .menu-grid-container {
            grid-template-columns: 56px auto;
        }
    }

    @media screen(sm) {
        .place-bttn-place,
        .place-bttn-delete {
            height: 64px;
            min-height: 64px;
            width: 100%;
        }

        .menu-grid-container {
            grid-template-columns: 44px auto;
        }
    }
</style>
