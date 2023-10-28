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
    import { CATEGORY_MAP, OBJECT_SETTINGS } from "../gd/object";
    import SlidingSelector from "../components/SlidingSelector.svelte";

    enum Group {
        Build,
        Edit,
        Delete,
    }

    enum EditTab {
        Transform = "Transform",
        Layers = "Layers",
        Colors = "Colors",
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
</script>

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
        class="flex flex-col justify-end w-full h-full pb absolute pointer-events-none"
        use:motion
    >
        <div class="flex justify-end gap-2 pointer-events-all text-white">
            <Animate
                easing="easeInOut"
                duration={minimizeAnimDur}
                initial={{
                    gridTemplateRows: "48px 200px",
                }}
                definition={{
                    isMinimized: {
                        gridTemplateRows: "48px 0px",
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
                                    class={cx({
                                        "absolute w-full h-full p-2 flex justify-evenly overflow-y-hidden no-scrollbar": true,
                                        "overflow-hidden":
                                            menuSettings.isMinimized,
                                        "overflow-x-scroll":
                                            !menuSettings.isMinimized,
                                    })}
                                    use:motion
                                >
                                    <AnimateSharedLayout>
                                        {#if menuSettings.selectedGroup == Group.Build}
                                            {#each Object.entries(CATEGORY_MAP) as [key, path]}
                                                <li
                                                    class="relative h-full flex-center cursor-pointer flex-1"
                                                >
                                                    <button
                                                        class="z-20 w-full h-full flex-center py-1"
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
                                                    class="relative h-full flex-center cursor-pointer flex-1"
                                                >
                                                    <button
                                                        class="h-full flex-center px-4 cursor-pointer w-full"
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
                        class="w-full h-full flex-center overflow-hidden menu-panel side-menu flex-center"
                    >
                        <ul
                            class="absolute flex flex-col items-center w-full h-full justify-between px-3 py-4 gap-6"
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

                    <ul
                        class="buttons object-grid-container menu-panel w-full h-full overflow-x-hidden overflow-y-scroll no-scrollbar"
                    >
                        <AnimateSharedLayout>
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
                                        class={cx({
                                            "absolute w-full h-full p-2 z-20": true,
                                            hidden: false,
                                        })}
                                        on:click={() => {
                                            menuSettings.selectedObject =
                                                obj.id;
                                        }}
                                    >
                                        <div
                                            class="relative flex-center w-full h-full"
                                        >
                                            <Image
                                                class="absolute object-contain w-auto h-auto max-w-full max-h-full"
                                                src={`/textures/main/${obj.id}.png`}
                                                lazyLoad
                                                skeleton
                                            ></Image>
                                            <Image
                                                class="absolute object-contain w-auto h-auto max-w-full max-h-full"
                                                src={`/textures/detail/${obj.id}.png`}
                                                lazyLoad
                                            ></Image>
                                        </div>
                                    </button>
                                    {#if menuSettings.selectedObject == obj.id}
                                        <SlidingSelector
                                            layoutId="selected-object"
                                        ></SlidingSelector>
                                    {/if}
                                </li>
                            {/each}
                        </AnimateSharedLayout>
                    </ul>
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
                    <div class="w-full h-full overflow-hidden py-4">
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
        grid-gap: 12px;
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
