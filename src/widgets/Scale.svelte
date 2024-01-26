<script lang="ts">
    import { clamp, round } from "shared-lib";
    import * as wasm from "wasm-lib";

    import Input from "../components/Input.svelte";
    import { addCallback as addUpdateCallback, withState } from "../state";
    import { onDestroy } from "svelte";

    let scale = 1;
    let prevScale = 1;

    const MIN = 0.5;
    const MAX = 2.0;

    let cb = addUpdateCallback(state => {
        let obj = state.get_preview_object();
        let xLen = obj.x_scale_exp;
        let yLen = obj.y_scale_exp;

        let larger = Math.max(xLen, yLen);
        // let smaller = Math.min(xLen, yLen);

        let scaleExp = Math.round(Math.log2(scale) * 12);

        if (scale != prevScale) {
            obj.scale(scaleExp - larger);
            state.set_preview_object(obj);
            scale = 2 ** (scaleExp / 12);
            prevScale = scale;
            return;
        }
        if (scaleExp != larger) {
            scale = 2 ** (larger / 12);
            prevScale = scale;
        }
    });

    onDestroy(() => cb.remove());
</script>

<div class="absolute text-white">
    <div class="abs-centered-rel flex flex-col items-center gap-8 bottom-12">
        <div class="flex flex-row items-center gap-4 bottom-12">
            <span class="font-pusab text-3xl text-stroke">Scale:</span>
            <span class="font-pusab text-3xl text-stroke w-16 text-center"
                >{round(scale, 2)}</span
            >
        </div>

        <input
            type="range"
            class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
            style:width="500px"
            max={MAX}
            min={MIN}
            step={0.0001}
            value={scale}
            on:input={e => {
                scale = parseFloat(e.currentTarget.value);
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
