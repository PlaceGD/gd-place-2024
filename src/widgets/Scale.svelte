<script lang="ts">
    import { clamp } from "shared-lib";
    import * as wasm from "wasm-lib";

    import Input from "../components/Input.svelte";
    import {
        addCallback as addUpdateCallback,
        setPreviewObject,
        withState,
    } from "../state";
    import { onDestroy } from "svelte";
    import { widgetData } from "../stores";

    export let isXY: boolean;

    const HARD_VALID_INPUT = /^(|-?\d*(\.\d*)?)$/;
    const SOFT_VALID_INPUT = (min: number, max: number) => (s: string) => {
        let f = parseFloat(s);
        if (isNaN(f)) {
            return false;
        }
        return min <= f && f <= max;
    };

    const MIN = 0.4999;
    const MAX = 2.0001;

    let singleMin = 0.5;

    let cb = addUpdateCallback(state => {
        let obj = state.get_preview_object();
        let xLen = obj.x_basis_len();
        let yLen = obj.y_basis_len();

        if (isXY) {
            if ($widgetData.scaleX != $widgetData.prevScaleX) {
                obj.set_x_scale($widgetData.scaleX);
                setPreviewObject(obj);
            }
            if ($widgetData.scaleY != $widgetData.prevScaleY) {
                obj.set_y_scale($widgetData.scaleY);
                setPreviewObject(obj);
            }
        } else {
            let larger = Math.max(xLen, yLen);
            let smaller = Math.min(xLen, yLen);

            singleMin = (0.5 * larger) / smaller;

            if ($widgetData.scale != $widgetData.prevScale) {
                obj.scale($widgetData.scale / larger);
                if (xLen >= yLen) {
                    obj.set_x_scale($widgetData.scale);
                } else {
                    obj.set_y_scale($widgetData.scale);
                }
                console.log(obj.x_basis_len());
                setPreviewObject(obj);
            }
        }
    });

    onDestroy(() => cb.remove());
</script>

<div class="absolute text-white">
    {#if isXY}
        <div
            class="abs-centered-rel flex flex-col items-center gap-8 bottom-48"
        >
            <div class="flex flex-row items-center gap-4 bottom-12">
                <span class="font-pusab text-3xl text-stroke">Scale X:</span>
                <Input
                    class="w-32 p-2 pointer-events-all backdrop-blur-lg text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
                    maxLength={$widgetData.maxScaleLen}
                    hardValidInput={HARD_VALID_INPUT}
                    softValidInput={SOFT_VALID_INPUT(MIN, MAX)}
                    aria-label="Scale input"
                    bind:value={$widgetData.scaleX}
                />
            </div>

            <input
                type="range"
                class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
                style:width="500px"
                max={MAX}
                min={MIN}
                step={0.05}
                bind:value={$widgetData.scaleX}
                aria-label="Scale slider"
            />
        </div>
        <div
            class="abs-centered-rel flex flex-col items-center gap-8 bottom-12"
        >
            <div class="flex flex-row items-center gap-4 bottom-12">
                <span class="font-pusab text-3xl text-stroke">Scale Y:</span>
                <Input
                    class="w-32 p-2 pointer-events-all backdrop-blur-lg text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
                    maxLength={$widgetData.maxScaleLen}
                    hardValidInput={HARD_VALID_INPUT}
                    softValidInput={SOFT_VALID_INPUT(MIN, MAX)}
                    aria-label="Scale input"
                    bind:value={$widgetData.scaleY}
                />
            </div>

            <input
                type="range"
                class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
                style:width="500px"
                max={MAX}
                min={MIN}
                step={0.05}
                bind:value={$widgetData.scaleY}
                aria-label="Scale slider"
            />
        </div>
    {:else}
        <div
            class="abs-centered-rel flex flex-col items-center gap-8 bottom-12"
        >
            <div class="flex flex-row items-center gap-4 bottom-12">
                <span class="font-pusab text-3xl text-stroke">Scale:</span>
                <Input
                    class="w-32 p-2 pointer-events-all backdrop-blur-lg text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
                    maxLength={$widgetData.maxScaleLen}
                    hardValidInput={HARD_VALID_INPUT}
                    softValidInput={SOFT_VALID_INPUT(singleMin, MAX)}
                    aria-label="Scale input"
                    bind:value={$widgetData.scale}
                />
            </div>

            <input
                type="range"
                class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
                style:width="500px"
                max={MAX}
                min={singleMin}
                step={0.05}
                bind:value={$widgetData.scale}
                aria-label="Scale slider"
            />
        </div>
    {/if}
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
        background-image: url("/assets/ui/edit/flip.svg");
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
        @apply bg-button-cyan-press;
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
        background-image: url("/assets/ui/edit/flip.svg");
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
        @apply bg-button-cyan-press outline-none;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
        outline-offset: 0.125rem;
    }
</style>
