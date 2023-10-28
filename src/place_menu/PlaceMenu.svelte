<script lang="ts">
    import { default as cx } from "classnames";
    import {
        AnimateSharedLayout,
        Motion,
        useAnimation,
        useReducedMotion,
    } from "svelte-motion";

    import LocalSettings from "../utils/LocalSettings";

    import Image from "../components/Image.svelte";
    import Animate from "../components/Animate.svelte";

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
    import { EDIT_BUTTONS, EditTab } from "./edit_tab";
    import { __DEBUG } from "../main";

    enum Group {
        Build,
        Edit,
        Delete,
    }

    let menuSettings = new LocalSettings("menuSettings", {
        isMinimized: false,
        selectedGroup: Group.Build,
        selectedEditTab: EditTab.Transform,
        selectedBuildTab: "Blocks",
        selectedObject: 1,
    });

    const minimizeAnimDur = 0.5;
    const shouldReducedMotion = useReducedMotion();

    let tabsPanel: HTMLUListElement;
    let isTabsPanelOverflow: boolean = false;
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
        isMinimized: menuSettings.isMinimized,
    }}
    let:motion
>
    <div
        class="absolute flex flex-col justify-end w-full h-full pointer-events-none pb"
        use:motion
    >
        <div class="flex justify-end gap-2 text-white pointer-events-all">
            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={{
                    // adds 8px to account for 8px scroll bar if overflowing
                    gridTemplateRows: `${
                        isTabsPanelOverflow ? "56px" : "48px"
                    } 200px`,
                }}
                definition={{
                    isMinimized: {
                        gridTemplateRows: `${
                            isTabsPanelOverflow ? "56px" : "48px"
                        } 0px`,
                    },
                }}
                conditions={{
                    isMinimized: menuSettings.isMinimized,
                }}
                let:motion
            >
                <div class="flex-1 menu-grid-container" use:motion>
                    <div
                        class="flex flex-col items-center minimize menu-panel justify-evenly"
                    >
                        <button
                            class="absolute w-full h-full p-4 flex-center"
                            on:click={() => {
                                menuSettings.isMinimized =
                                    !menuSettings.isMinimized;
                            }}
                        >
                            <Minimize
                                class={cx({
                                    "cursor-pointer": true,
                                    "rotate-180": menuSettings.isMinimized,
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
                            isMinimized: menuSettings.isMinimized,
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
                                    isMinimized: !menuSettings.isMinimized,
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
                                        {#if menuSettings.selectedGroup == Group.Build}
                                            {#each Object.entries(CATEGORY_ICONS) as [key, path]}
                                                <li
                                                    class="relative h-full flex-center cursor-pointer flex-1 min-w-[64px]"
                                                >
                                                    <button
                                                        class="z-20 w-full h-full py-1 flex-center"
                                                        on:click={() => {
                                                            menuSettings.selectedBuildTab =
                                                                key;
                                                        }}
                                                    >
                                                        <Image
                                                            src={path}
                                                            alt={key}
                                                            class="object-contain w-auto h-auto max-w-full max-h-full"
                                                        ></Image>
                                                    </button>
                                                    {#if menuSettings.selectedBuildTab == key}
                                                        <SlidingSelector
                                                            layoutId="selected-build-tab"
                                                        ></SlidingSelector>
                                                    {/if}
                                                </li>
                                            {/each}
                                        {:else if menuSettings.selectedGroup == Group.Edit}
                                            {#each Object.keys(EditTab) as key}
                                                <li
                                                    class="relative flex-1 h-full cursor-pointer flex-center"
                                                >
                                                    <button
                                                        class="w-full h-full px-4 cursor-pointer flex-center"
                                                        on:click={() => {
                                                            menuSettings.selectedEditTab =
                                                                key;
                                                        }}
                                                    >
                                                        <h1
                                                            class="z-20 text-2xl font-pusab text-stroke"
                                                        >
                                                            {key}
                                                        </h1>
                                                    </button>
                                                    {#if menuSettings.selectedEditTab == key}
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
                                    isMinimized: menuSettings.isMinimized,
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
                                            menuSettings.selectedGroup !=
                                            Group.Build,
                                    })}
                                    on:click={() => {
                                        menuSettings.selectedGroup =
                                            Group.Build;
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
                                            menuSettings.selectedGroup !=
                                            Group.Edit,
                                    })}
                                    on:click={() => {
                                        menuSettings.selectedGroup = Group.Edit;
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
                                            menuSettings.selectedGroup !=
                                            Group.Delete,
                                    })}
                                    on:click={() => {
                                        menuSettings.selectedGroup =
                                            Group.Delete;
                                    }}
                                >
                                    <Delete></Delete>
                                </button>
                            </li>
                        </ul>
                    </div>

                    <div class="w-full h-full rounded-lg buttons menu-panel">
                        <ul
                            class={cx({
                                "w-full h-full overflow-x-hidden overflow-y-auto rounded-lg object-grid-container thin-scrollbar": true,
                                "gap-4":
                                    menuSettings.selectedGroup == Group.Edit,
                            })}
                        >
                            <!-- BUILD TAB -->
                            {#each OBJECT_SETTINGS as obj, i}
                                <li
                                    class={cx({
                                        "relative w-16 h-16": true,
                                        hidden:
                                            menuSettings.selectedGroup !=
                                                Group.Build ||
                                            menuSettings.selectedBuildTab !=
                                                obj.category,
                                    })}
                                >
                                    <button
                                        class={"absolute w-full h-full p-3 z-20"}
                                        on:click={() => {
                                            menuSettings.selectedObject =
                                                obj.id;
                                        }}
                                    >
                                        {#if __DEBUG}
                                            <span
                                                class="absolute text-red font-lg bottom-3/4 right-1/2"
                                            >
                                                {obj.id}
                                            </span>
                                        {/if}
                                        <div
                                            class="relative w-full h-full flex-center"
                                        >
                                            <Image
                                                class="absolute object-contain"
                                                src={`/textures/main/${obj.id}.png`}
                                                lazyLoad
                                                skeleton
                                                style={`
                                                        max-width: ${
                                                            MAIN_DETAIL_TEX_RATIOS[
                                                                obj.id
                                                            ].main * 100
                                                        }%;
                                                        max-height: ${
                                                            MAIN_DETAIL_TEX_RATIOS[
                                                                obj.id
                                                            ].main * 100
                                                        }%;
                                                    `}
                                            ></Image>
                                            <Image
                                                class="absolute object-contain"
                                                src={`/textures/detail/${obj.id}.png`}
                                                lazyLoad
                                                style={`
                                                        max-width: ${
                                                            MAIN_DETAIL_TEX_RATIOS[
                                                                obj.id
                                                            ].detail * 100
                                                        }%;
                                                        max-height: ${
                                                            MAIN_DETAIL_TEX_RATIOS[
                                                                obj.id
                                                            ].detail * 100
                                                        }%;
                                                        filter: sepia(50%) saturate(5000%) hue-rotate(175deg);
                                                    `}
                                            ></Image>
                                        </div>
                                    </button>
                                    {#if menuSettings.selectedObject == obj.id}
                                        <span
                                            class={"absolute w-full h-full sliding-selector"}
                                        ></span>
                                    {/if}
                                </li>
                            {/each}
                            <!-- EDIT TAB -->
                            {#each EDIT_BUTTONS[menuSettings.selectedEditTab].buttons as button, i (Object.keys(EditTab).indexOf(menuSettings.selectedEditTab) * 100 + i)}
                                <li
                                    class={cx({
                                        " w-16 h-16": true,
                                        hidden:
                                            menuSettings.selectedGroup !=
                                            Group.Edit,
                                    })}
                                >
                                    <button
                                        class={"flex-center w-full h-full p-3 z-20 rounded-md bg-gradient-to-b from-button-light-green to-button-dark-green"}
                                    >
                                        <Image
                                            class={"object-cover max-w-full max-h-full h-auto w-auto"}
                                            src={`/assets/ui/edit/${button.image}.png`}
                                            style="transform: scale({button.scale})"
                                            lazyLoad
                                        ></Image>
                                    </button>
                                </li>
                            {/each}
                        </ul>
                    </div>
                </div>
            </Animate>

            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={{
                    minHeight: "256px",
                }}
                definition={{
                    isMinimized: {
                        minHeight: 0,
                        height: 0,
                    },
                }}
                conditions={{
                    isMinimized: menuSettings.isMinimized,
                }}
                let:motion
            >
                <button
                    class={cx({
                        "self-end overflow-hidden": true,
                        "place-bttn-place":
                            menuSettings.selectedGroup != Group.Delete,
                        "place-bttn-delete":
                            menuSettings.selectedGroup == Group.Delete,
                    })}
                    use:motion
                >
                    <div class="w-full h-full py-4 overflow-hidden">
                        <h1
                            class="w-full h-full overflow-hidden text-5xl font-pusab text-stroke flex-center"
                        >
                            {#if menuSettings.selectedGroup != Group.Delete}
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

<style>
    .object-grid-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, 64px);
        padding: 16px;
        justify-content: space-between;
    }

    .menu-grid-container {
        display: grid;
        grid-template-columns: 64px auto;
        /* grid-template-rows: 48px 200px; */
        gap: 8px 8px;
        grid-template-areas:
            "minimize tabs"
            "side-menu buttons";
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
        width: 256px;
        border-radius: 16px;
        background: #61b91d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #3a6a16 inset,
            4px 4px 0px 8px #b2eb11 inset;
    }
    .place-bttn-delete {
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
        width: 256px;
        border-radius: 16px;
        background: #b91d20;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #6a1617 inset,
            4px 4px 0px 8px #eb1158 inset;
    }

    /* .side-menu-container {
        display: grid;
        grid-template-rows: auto auto auto;
        grid-template-areas:
            "build"
            "edit"
            "delete";
    } */
</style>
