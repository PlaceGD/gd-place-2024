<script lang="ts">
    import { round } from "shared-lib/util";
    import * as wasm from "wasm-lib";

    import { onDestroy } from "svelte";
    import { setCheckedPreviewObject } from "../utils/misc";

    import flipIconUrl from "../place_menu/assets/edit_tab/flip.svg?url";
    import { EDIT_TAB_ICONS } from "../place_menu/edit/edit_tab";

    export let state: wasm.State;
    let slider: HTMLInputElement;

    const MIN = 0.5;
    const MAX = 2.0;

    let scale = 0;

    const loopFn = () => {
        let obj = state.get_preview_object();
        let xLen = obj.x_scale_exp;
        let yLen = obj.y_scale_exp;

        let larger = Math.max(xLen, yLen);

        scale = 2 ** (larger / 12);
        slider.value = `${scale}`;

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => cancelAnimationFrame(loop));
</script>

<div class="absolute text-white">
    <div class="flex flex-col items-center gap-8 abs-centered-rel bottom-12">
        <div class="flex flex-row items-center gap-4 bottom-12">
            <span class="text-3xl font-pusab text-stroke">Scale:</span>
            <span class="w-20 text-3xl text-center font-pusab text-stroke"
                >{round(scale, 3)}</span
            >
        </div>

        <input
            type="range"
            bind:this={slider}
            class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
            style={`width: 500px; --flip: url("${EDIT_TAB_ICONS["flip"]}");`}
            max={MAX}
            min={MIN}
            step={0.0001}
            tabindex="-1"
            on:input={e => {
                let newScale = parseFloat(e.currentTarget.value);

                let obj = state.get_preview_object();
                let xLen = obj.x_scale_exp;
                let yLen = obj.y_scale_exp;
                let larger = Math.max(xLen, yLen);

                let scaleExp = Math.round(Math.log2(newScale) * 12);

                obj.scale(scaleExp - larger);
                setCheckedPreviewObject(state, obj);
                scale = 2 ** (scaleExp / 12);

                slider.value = `${scale}`;
            }}
            aria-label="Scale slider"
        />
    </div>
</div>

<style lang="postcss">
    input[type="range"]::-webkit-slider-runnable-track {
        @apply box-content h-4 rounded-3xl bg-transparent;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
    }

    input[type="range"]::-webkit-slider-thumb {
        @apply appearance-none rounded-full border-none bg-button-green bg-center bg-no-repeat outline-none;
        margin-top: -27px;
        background-image: var(--flip);
        background-size: 70px;

        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
        height: 70px;
        width: 70px;
    }

    input[type="range"]:active::-webkit-slider-thumb {
        @apply bg-button-cyan;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
        outline-offset: 0.125rem;
    }

    input[type="range"]::-moz-range-track {
        @apply box-content h-4 rounded-3xl bg-transparent;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
    }

    input[type="range"]::-moz-range-thumb {
        @apply appearance-none rounded-full border-none bg-button-green bg-center bg-no-repeat outline-none;
        background-image: var(--flip);
        background-size: 70px;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
        height: 70px;
        width: 70px;
    }

    input[type="range"]:active::-moz-range-thumb {
        @apply bg-button-cyan outline-none;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
        outline-offset: 0.125rem;
    }
</style>
