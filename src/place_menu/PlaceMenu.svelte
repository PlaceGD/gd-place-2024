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

    enum BuildTab {
        Blocks = "build_tab_icons/blocks.png",
        Outlines = "build_tab_icons/outlines.png",
        Slopes = "build_tab_icons/slopes.png",
        Spikes = "build_tab_icons/spikes.png",
        Utilities = "build_tab_icons/util.png",
        GroundDeco = "build_tab_icons/ground_deco.png",
        Deco = "build_tab_icons/deco.png",
        Pulsing = "build_tab_icons/pulsing.png",
        Saws = "build_tab_icons/saws.png",
    }

    let menuSettings = new LocalSettings("menuSettings", {
        isMinimized: false,
        selectedGroup: Group.Build,
        selectedEditTab: EditTab.Transform,
        selectedBuildTab: BuildTab.Blocks,
    });

    const minimizeAnimDur = 0.5;
    const shouldReducedMotion = useReducedMotion();
</script>

<div
    class="flex flex-col justify-end w-full h-full p-2 absolute pointer-events-none"
>
    <div class="flex justify-end gap-2 pointer-events-all">
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
            <div class="flex-1 grid-container overflow-hidden" use:motion>
                <div
                    class="flex flex-col items-center minimize menu-panel justify-evenly"
                >
                    <button
                        class="w-full h-full p-4 flex-center"
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
                                    "absolute w-full h-full p-2 flex justify-evenly text-white overflow-y-hidden no-scrollbar": true,
                                    "overflow-hidden": menuSettings.isMinimized,
                                    "overflow-x-scroll":
                                        !menuSettings.isMinimized,
                                    // "gap-12":
                                    //     menuSettings.selectedGroup ==
                                    //     Group.Edit,
                                })}
                                use:motion
                            >
                                <AnimateSharedLayout>
                                    {#if menuSettings.selectedGroup == Group.Build}
                                        {#each Object.entries(BuildTab) as [key, value]}
                                            <li
                                                class="relative h-full flex-center cursor-pointer flex-1"
                                            >
                                                <button
                                                    class="z-20 w-full h-full flex-center py-1"
                                                    on:click={() => {
                                                        menuSettings.selectedBuildTab =
                                                            value;
                                                    }}
                                                >
                                                    <Image
                                                        src={value}
                                                        loadAsync
                                                        alt={key}
                                                        class="object-contain w-auto h-auto max-w-full max-h-full"
                                                    ></Image>
                                                </button>
                                                {#if menuSettings.selectedBuildTab == value}
                                                    <Motion
                                                        let:motion
                                                        layoutId="selected-build-tab"
                                                        transition={{
                                                            duration:
                                                                $shouldReducedMotion
                                                                    ? 0
                                                                    : 0.2,
                                                        }}
                                                        animate={{
                                                            opacity: 0.1,
                                                            zIndex: 10,
                                                        }}
                                                    >
                                                        <span
                                                            class="absolute w-full h-full bg-white rounded-lg selected-build-tab opacity-10"
                                                            use:motion
                                                        >
                                                        </span>
                                                    </Motion>
                                                {/if}
                                            </li>{/each}
                                    {:else if menuSettings.selectedGroup == Group.Edit}
                                        {#each Object.keys(EditTab) as key}
                                            <button
                                                class="relative h-full flex-center px-4 cursor-pointer w-full"
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
                                                {#if menuSettings.selectedEditTab == key}
                                                    <Motion
                                                        let:motion
                                                        layoutId="selected-edit-tab"
                                                        transition={{
                                                            duration:
                                                                $shouldReducedMotion
                                                                    ? 0
                                                                    : 0.2,
                                                        }}
                                                        animate={{
                                                            opacity: 0.1,
                                                            zIndex: 10,
                                                        }}
                                                    >
                                                        <span
                                                            class="absolute w-full h-full bg-white rounded-lg selected-edit-tab opacity-10"
                                                            use:motion
                                                        >
                                                        </span>
                                                    </Motion>
                                                {/if}
                                            </button>
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
                <ul class="side-menu menu-panel side-menu-container">
                    <li>
                        <button
                            class={cx({
                                "h-full w-full cursor-pointer": true,
                                "opacity-30":
                                    menuSettings.selectedGroup != Group.Build,
                                build: true,
                                //"flex-grow flex-shrink w-full": true,
                            })}
                            on:click={() => {
                                menuSettings.selectedGroup = Group.Build;
                            }}
                        >
                            <Build></Build>
                        </button>
                    </li>
                    <li>
                        <button
                            class={cx({
                                "h-full w-full cursor-pointer": true,
                                "opacity-30":
                                    menuSettings.selectedGroup != Group.Edit,
                                edit: true,
                                //"flex-grow flex-shrink  w-full": true,
                            })}
                            on:click={() => {
                                menuSettings.selectedGroup = Group.Edit;
                            }}
                        >
                            <Edit></Edit>
                        </button>
                    </li>
                    <li>
                        <button
                            class={cx({
                                "h-full w-full cursor-pointer": true,
                                "opacity-30":
                                    menuSettings.selectedGroup != Group.Delete,
                                delete: true,
                                //"flex-grow flex-shrink  w-full": true,
                            })}
                            on:click={() => {
                                menuSettings.selectedGroup = Group.Delete;
                            }}
                        >
                            <Delete></Delete>
                        </button>
                    </li>

                    <!--  -->
                </ul>

                <div class="buttons menu-panel">
                    <!--  -->
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
            <div
                class={cx({
                    "self-end": true,
                    "place-bttn-place":
                        menuSettings.selectedGroup != Group.Delete,
                    "place-bttn-delete":
                        menuSettings.selectedGroup == Group.Delete,
                })}
                use:motion
            >
                <!--  -->
            </div>
        </Animate>
    </div>
</div>

<style>
    .grid-container {
        display: grid;
        grid-template-columns: 64px auto;
        grid-template-rows: 48px 200px;
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

    .side-menu-container {
        display: grid;
        grid-template-rows: auto auto auto;
        grid-template-areas:
            "build"
            "edit"
            "delete";
    }
</style>
