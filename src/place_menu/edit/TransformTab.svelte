<script lang="ts">
    import { default as cx } from "classnames";
    import {
        EditTab,
        MOVE_BUTTONS,
        TRANSFORM_BUTTONS,
        Widget,
    } from "./edit_tab";
    import Image from "../../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import { menuSettings } from "../../stores";
    import { onDestroy } from "svelte";

    export let state: wasm.StateWrapper;

    let selectedWidget = Widget.None;

    $: {
        $menuSettings.selectedWidget = selectedWidget;
    }

    onDestroy(() => {
        $menuSettings.selectedWidget = Widget.None;
    });

    const changeWidget = (widget: Widget) => {
        if (selectedWidget !== widget) selectedWidget = widget;
        else selectedWidget = Widget.None;
    };

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<div class="transform-container">
    <div
        class="flex items-center gap-2 grow-1 shrink-0 md:gap-4 xs:gap-1 move md:flex-center"
    >
        {#each Object.keys(MOVE_BUTTONS) as button}
            <div
                class={cx({
                    "move-button-grid": true,
                    [MOVE_BUTTONS[button].class ?? ""]: true,
                })}
            >
                <button
                    class="z-20 w-12 rounded-md shrink-0 up flex-center aspect-square md:w-9 sm:w-7 white-button"
                    tabindex={canSelectByTab}
                    aria-label="{MOVE_BUTTONS[button].name} up"
                >
                    <Image
                        src="/assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        alt="^"
                        class="rotate-180"
                    />
                </button>
                <button
                    class="z-20 w-12 rounded-md shrink-0 down flex-center aspect-square md:w-9 sm:w-7 white-button"
                    tabindex={canSelectByTab}
                    aria-label="{MOVE_BUTTONS[button].name} down"
                >
                    <Image
                        src="/assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        alt="V"
                    />
                </button>
                <button
                    class="z-20 w-12 rounded-md shrink-0 right flex-center aspect-square md:w-9 sm:w-7 white-button"
                    tabindex={canSelectByTab}
                    aria-label="{MOVE_BUTTONS[button].name} right"
                >
                    <Image
                        src="/assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="-rotate-90"
                        alt=">"
                    />
                </button>
                <button
                    class="z-20 w-12 rounded-md shrink-0 left flex-center aspect-square md:w-9 sm:w-7 white-button"
                    tabindex={canSelectByTab}
                    aria-label="{MOVE_BUTTONS[button].name} left"
                >
                    <Image
                        src="/assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="rotate-90"
                        alt="<"
                    />
                </button>
                <p
                    class="text-lg text-center text-white font-pusab text-stroke amount md:text-sm"
                >
                    {MOVE_BUTTONS[button].amount}
                </p>
            </div>
        {/each}
    </div>
    <ul
        class="flex flex-wrap items-center justify-center flex-1 h-full gap-2 transforms"
    >
        {#each TRANSFORM_BUTTONS as button, i (i)}
            <li class="transform-button">
                <button
                    class="flex-center w-full h-full p-2 md:p-1.5 sm:p-0 xs:p-0 z-20 rounded-md bg-button-green active:bg-button-green-dark bounce-active"
                    on:click={() => {
                        let obj = state.get_preview_object();
                        button.cb(obj);
                        state.set_preview_object(obj);
                    }}
                    tabindex={canSelectByTab}
                    aria-label={button.name}
                >
                    <Image
                        class="object-contain max-w-full max-h-full"
                        src={`/assets/ui/edit/${button.image}.svg`}
                        style={`transform: rotate(${button.angle}deg)${
                            button.flipped ? " scaleX(-1)" : ""
                        }`}
                        lazyLoad
                        skeleton
                    />
                </button>
            </li>
        {/each}
    </ul>
    <div class="widget-button-container widgets">
        <button
            class={cx({
                "widget-button bounce-active": true,
                "bg-button-green active:bg-button-green-dark":
                    selectedWidget !== Widget.Rotate,
                "bg-button-cyan active:bg-button-cyan-dark":
                    selectedWidget === Widget.Rotate,
            })}
            tabindex={canSelectByTab}
            aria-label="Rotate Object"
            on:click={() => changeWidget(Widget.Rotate)}
            role="checkbox"
            aria-checked={selectedWidget == Widget.Rotate}
        >
            <p class="font-pusab text-stroke">Rotate</p>
            <Image
                src="/assets/ui/edit/rotate.svg"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
        <button
            class={cx({
                "widget-button bounce-active": true,
                "bg-button-green active:bg-button-green-dark":
                    selectedWidget !== Widget.Scale,
                "bg-button-cyan active:bg-button-cyan-dark":
                    selectedWidget === Widget.Scale,
            })}
            tabindex={canSelectByTab}
            aria-label="Scale Object"
            on:click={() => changeWidget(Widget.Scale)}
            role="checkbox"
            aria-checked={selectedWidget == Widget.Scale}
        >
            <p class="font-pusab text-stroke">Scale</p>
            <Image
                src="/assets/ui/edit/scale.svg"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
        <button
            class={cx({
                "widget-button bounce-active": true,
                "bg-button-green active:bg-button-green-dark":
                    selectedWidget !== Widget.Warp,
                "bg-button-cyan active:bg-button-cyan-dark":
                    selectedWidget === Widget.Warp,
            })}
            tabindex={canSelectByTab}
            aria-label="Warp Object"
            on:click={() => changeWidget(Widget.Warp)}
            role="checkbox"
            aria-checked={selectedWidget == Widget.Warp}
        >
            <p class="font-pusab text-stroke">Warp</p>
            <Image
                src="/assets/ui/edit/warp.svg"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
    </div>
</div>

<style lang="postcss">
    .transform-container {
        @apply grid h-full w-full gap-4 p-4;
        grid-template-areas: "move transforms widgets";
    }

    .widget-button-container {
        @apply flex w-full flex-1 gap-2 justify-self-end;
    }

    @media not screen(md) {
        .transform-container {
            grid-template-columns: 1fr 1fr 1fr;
            grid-template-rows: 1fr;
        }
    }

    .move {
        grid-area: move;
    }
    .transforms {
        grid-area: transforms;
    }
    .widgets {
        grid-area: widgets;
    }

    .widget-button {
        @apply flex h-full w-full flex-1 flex-col flex-col items-center justify-center gap-2 rounded-md text-2xl;
    }

    .transform-button {
        @apply aspect-square w-16 justify-self-center;
    }

    .move-button-grid {
        @apply grid aspect-square w-auto  items-center justify-center gap-1;
        grid-template-columns: min-content min-content min-content;
        grid-template-rows: min-content min-content min-content;
        grid-template-areas:
            ". up ."
            "left amount right"
            ". down .";
    }

    .amount {
        grid-area: amount;
    }
    .up {
        grid-area: up;
    }
    .left {
        grid-area: left;
    }
    .down {
        grid-area: down;
    }
    .right {
        grid-area: right;
    }

    @media only screen and (max-width: 1650px) {
        .hide-big {
            @apply hidden;
        }
    }

    @media only screen and (max-width: 1510px) {
        .hide-small {
            @apply hidden;
        }
    }

    @media only screen and (max-width: 1340px) {
        .transform-container {
            @apply gap-4 p-1;
        }

        .transform-button {
            @apply w-14;
        }

        .widget-button-container {
            @apply flex-col;
        }

        .widget-button {
            @apply flex-row text-xl;
        }
    }

    /* yay we hit md!!!!!!!!!!!!!!!!!!!!! */
    @media screen(md) {
        .transform-container {
            @apply gap-2 p-2;
        }

        .transform-button {
            @apply w-10;
        }

        .widget-button-container {
            @apply min-w-44;
        }

        .transform-container {
            grid-template-columns: 1fr min-content;
            grid-template-rows: 1fr 1fr;
            grid-template-areas:
                "move widgets"
                "transforms widgets";
        }
    }

    @media screen(sm) {
        .transform-button {
            @apply w-8;
        }

        .widget-button-container {
            @apply w-auto min-w-min items-end;
        }

        .widget-button {
            @apply w-36 text-base;
        }
    }

    @media screen(xs) {
        .transform-container {
            @apply overflow-x-auto;
        }

        .widget-button {
            @apply w-16 text-xs;
        }
    }
</style>
