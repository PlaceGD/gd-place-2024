<!-- <script lang="ts">
    import { TRANSFORM_BUTTONS } from "./edit_tab";
    import Image from "../../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import { menuSettings } from "../../stores";

    export let state: wasm.StateWrapper;

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<ul
    class="w-full h-full gap-4 xs:gap-2 overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar transform-grid-container"
    tabindex="-1"
>
    {#each TRANSFORM_BUTTONS as button, i (i)}
        <li class="w-16 h-16 md:w-14 md:h-14 xs:w-10 xs:h-10">
            <button
                class={"flex-center w-full h-full p-2 md:p-1.5 xs:p-1 z-20 rounded-md bg-button-green bounce-active"}
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

<style lang="postcss">
    .transform-grid-container {
        @apply grid justify-between p-4 md:p-3 xs:p-2;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    @media screen(sm) {
        .transform-grid-container {
            grid-template-columns: repeat(auto-fill, 56px);
        }
    }

    @media screen(xs) {
        .transform-grid-container {
            grid-template-columns: repeat(auto-fill, 48px);
        }
    }
</style> -->

<script lang="ts">
    import { default as cx } from "classnames";
    import { TRANSFORM_BUTTONS } from "./edit_tab";
    import Image from "../../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import { menuSettings } from "../../stores";

    export let state: wasm.StateWrapper;

    interface Button {
        image: string;
        amount: string;
        class?: string;
    }
    type Buttons = { [key: string]: Button };

    const MOVE_BUTTONS: Buttons = {
        MOVE_TINY: {
            image: "move_mini",
            amount: "1/60",
        },
        MOVE_SMALL: {
            image: "move_small",
            amount: "1/15",
            class: "hide-small",
        },
        MOVE_NORMAL: {
            image: "move_normal",
            amount: "1",
        },
        MOVE_BIG: {
            image: "move_big",
            amount: "5",
            class: "hide-big",
        },
    };

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<div class="w-full h-full transform-container overflow-x-auto">
    <div
        class="grow-1 shrink-0 flex gap-2 md:gap-4 xs:gap-1 move md:flex-center"
    >
        {#each Object.keys(MOVE_BUTTONS) as button}
            <div
                class={cx({
                    "move-button-grid w-auto aspect-square": true,
                    [MOVE_BUTTONS[button].class ?? ""]: true,
                })}
            >
                <button
                    class="move-button shrink-0 up flex-center z-20 w-12 aspect-square md:w-9 sm:w-7 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30"
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="rotate-180"
                    />
                </button>
                <button
                    class="move-button shrink-0 down flex-center z-20 w-12 aspect-square md:w-9 sm:w-7 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30"
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                    />
                </button>
                <button
                    class="move-button shrink-0 right flex-center z-20 w-12 aspect-square md:w-9 sm:w-7 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30"
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="-rotate-90"
                    />
                </button>
                <button
                    class="move-button shrink-0 left flex-center z-20 w-12 aspect-square md:w-9 sm:w-7 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30"
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="rotate-90"
                    />
                </button>
                <p
                    class="font-pusab text-stroke text-lg text-white amount md:text-sm"
                >
                    {MOVE_BUTTONS[button].amount}
                </p>
            </div>
        {/each}
    </div>
    <ul
        class="flex-1 gap-2 transform-grid-container h-full justify-center transforms md:items-center md:justify-center flex items-center flex-wrap"
    >
        {#each TRANSFORM_BUTTONS as button, i (i)}
            <li class="aspect-square md:justify-self-center transform-button">
                <button
                    class="flex-center w-full h-full p-2 md:p-1.5 sm:p-1 xs:p-0 z-20 rounded-md bg-button-green active:bg-button-cyan-active bounce-active"
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
    <div
        class="flex-1 flex gap-2 widget-button-container widgets md:justify-self-end"
    >
        <button
            class="flex-1 flex flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md min-w-32 widget-button"
            tabindex={canSelectByTab}
            aria-label="Rotate Object"
        >
            <p class="font-pusab text-stroke">Rotate</p>
            <Image
                src="assets/ui/edit/rotate.svg"
                alt="Rotate Icon"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
        <button
            class="flex-1 flex flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md min-w-32 widget-button"
            tabindex={canSelectByTab}
            aria-label="Scale Object"
        >
            <p class="font-pusab text-stroke">Scale</p>
            <Image
                src="assets/ui/edit/scale.svg"
                alt="Scale Icon"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
        <button
            class="flex-1 flex flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md min-w-32 widget-button"
            tabindex={canSelectByTab}
            aria-label="Warp Object"
        >
            <p class="font-pusab text-stroke">Warp</p>
            <Image
                src="assets/ui/edit/warp.svg"
                alt="Warp Icon"
                class="aspect-square md:w-10 xs:hidden"
            />
        </button>
    </div>
</div>

<style lang="postcss">
    .transform-container {
        @apply grid gap-6 p-4;
        grid-template-areas: "move transforms widgets";
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
        @apply flex-col text-2xl;
    }

    .transform-button {
        @apply w-16;
    }

    .move-button-grid {
        display: grid;
        grid-template-columns: min-content min-content min-content;
        grid-template-rows: min-content min-content min-content;
        grid-template-areas:
            ". up ."
            "left amount right"
            ". down .";
        justify-items: center;
        align-items: center;
        gap: 4px;
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
            @apply min-w-0 flex-row text-xl;
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

        .transform-grid-container {
            grid-template-columns: repeat(auto-fill, 38px);
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
            @apply min-w-0 items-end;
        }

        .widget-button {
            @apply w-36 text-base;
        }
    }

    @media screen(xs) {
        .widget-button {
            @apply w-16 text-xs;
        }
    }
</style>
