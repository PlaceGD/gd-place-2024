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
        },
        MOVE_NORMAL: {
            image: "move_normal",
            amount: "1",
        },
        MOVE_BIG: {
            image: "move_big",
            amount: "5",
            class: "min-[1650px]:hidden",
        },
    };

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<div class="w-full h-full p-4 flex gap-1">
    <div class="grow-1 shrink-0 flex gap-2">
        {#each Object.keys(MOVE_BUTTONS) as button}
            <div class="move-button-grid w-auto aspect-square">
                <button
                    class={cx({
                        "up flex-center z-20 w-12 h-12 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30": true,
                        [MOVE_BUTTONS[button].class ?? ""]: true,
                    })}
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="rotate-180"
                    />
                </button>
                <button
                    class={cx({
                        "down flex-center z-20 w-12 h-12 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30": true,
                        [MOVE_BUTTONS[button].class ?? ""]: true,
                    })}
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                    />
                </button>
                <button
                    class={cx({
                        "right flex-center z-20 w-12 h-12 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30": true,
                        [MOVE_BUTTONS[button].class ?? ""]: true,
                    })}
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="-rotate-90"
                    />
                </button>
                <button
                    class={cx({
                        "left flex-center z-20 w-12 h-12 rounded-md bg-white/5 hover:bg-white/15 active:bg-white/30": true,
                        [MOVE_BUTTONS[button].class ?? ""]: true,
                    })}
                    tabindex={canSelectByTab}
                >
                    <Image
                        src="assets/ui/edit/{MOVE_BUTTONS[button].image}.svg"
                        class="rotate-90"
                    />
                </button>
                <p
                    class={cx({
                        "font-pusab text-stroke text-lg text-white amount": true,
                        [MOVE_BUTTONS[button].class ?? ""]: true,
                    })}
                >
                    {MOVE_BUTTONS[button].amount}
                </p>
            </div>
        {/each}
    </div>
    <ul class="flex-1 flex gap-2 transform-grid-container">
        {#each TRANSFORM_BUTTONS as button, i (i)}
            <li class="w-16 h-16 md:w-14 md:h-14 xs:w-10 xs:h-10">
                <button
                    class="flex-center w-full h-full p-2 md:p-1.5 xs:p-1 z-20 rounded-md bg-button-green active:bg-button-cyan-active bounce-active"
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
    <div class="flex-1 flex gap-2 text-2xl">
        <button
            class="flex-1 flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md p-2 min-w-32"
            tabindex={canSelectByTab}
            aria-label="Rotate Object"
        >
            <p class="font-pusab text-stroke">Rotate</p>
            <Image
                src="assets/ui/edit/rotate.svg"
                alt="Rotate Icon"
                class="w-16 aspect-square"
            />
        </button>
        <button
            class="flex-1 flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md p-2 min-w-32"
            tabindex={canSelectByTab}
            aria-label="Scale Object"
        >
            <p class="font-pusab text-stroke">Scale</p>
            <Image
                src="assets/ui/edit/scale.svg"
                alt="Scale Icon"
                class="w-16 aspect-square"
            />
        </button>
        <button
            class="flex-1 flex-center gap-2 flex-col w-full h-full bg-button-green active:bg-button-cyan-press rounded-md p-2 min-w-32"
            tabindex={canSelectByTab}
            aria-label="Warp Object"
        >
            <p class="font-pusab text-stroke">Warp</p>
            <Image
                src="assets/ui/edit/warp.svg"
                alt="Warp Icon"
                class="w-16 aspect-square"
            />
        </button>
    </div>
</div>

<style lang="postcss">
    .transform-grid-container {
        @apply grid justify-between p-4 md:p-3 xs:p-2;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    .move-button-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-rows: 1fr 1fr 1fr;
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
</style>
