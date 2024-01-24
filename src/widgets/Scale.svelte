<script lang="ts">
    import { clamp } from "shared-lib";
    import * as wasm from "wasm-lib";

    export let state: wasm.StateWrapper;

    import Input from "../components/OldInput.svelte";

    let scale = 1;
    $: console.log(scale);

    let textInput: Input;
</script>

<div class="absolute text-white">
    <div class="abs-centered-rel flex flex-col items-center gap-8 bottom-12">
        <Input
            class="w-28 p-2 pointer-events-all backdrop-blur-lg text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
            max={4}
            min={0.25}
            maxLength={5}
            defaultValue="1"
            validInput={s => {
                return !isNaN(parseFloat(s));
            }}
            on:input={e => {
                let s = parseFloat(e.detail);
                scale = clamp(s, 0.25, 4);
            }}
            aria-label="Scale input"
            bind:this={textInput}
        />
        <input
            type="range"
            class="pointer-events-all scale-slider w-[400px] cursor-pointer appearance-none bg-transparent focus:outline-none"
            style:width="500px"
            max={4}
            min={0.25}
            step={0.05}
            bind:value={scale}
            on:input={() => {
                textInput.setValue(scale);
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
            inset 0px 0px 0px 3px #000;
    }

    input[type="range"]::-webkit-slider-thumb {
        @apply appearance-none rounded-full border-none bg-button-green bg-center bg-no-repeat outline-none;
        margin-top: -27px;
        background-image: url("/assets/ui/edit/flip.svg");
        background-size: 60px;

        box-shadow:
            inset 0px 0px 0px 3px #fff,
            inset 0px 0px 0px 6px #000;
        height: 70px;
        width: 70px;
    }

    input[type="range"]:active::-webkit-slider-thumb {
        @apply bg-button-cyan-press;
        box-shadow:
            inset 0px 0px 0px 3px #fff,
            inset 0px 0px 0px 6px #000;
        outline-offset: 0.125rem;
    }

    input[type="range"]::-moz-range-track {
        @apply box-content h-4 rounded-3xl bg-transparent;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 0px 0px 6px #000,
            inset 0px 0px 0px 3px #000;
    }

    input[type="range"]::-moz-range-thumb {
        @apply appearance-none rounded-full border-none bg-button-green bg-center bg-no-repeat outline-none;
        background-image: url("/assets/ui/edit/flip.svg");
        background-size: 60px;
        box-shadow:
            inset 0px 0px 0px 3px #fff,
            inset 0px 0px 0px 6px #000;
        height: 70px;
        width: 70px;
    }

    input[type="range"]:active::-moz-range-thumb {
        @apply bg-button-cyan-press outline-none;
        box-shadow:
            inset 0px 0px 0px 3px #fff,
            inset 0px 0px 0px 6px #000;
        outline-offset: 0.125rem;
    }
</style>
