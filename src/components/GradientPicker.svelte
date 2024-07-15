<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import ColorPicker from "svelte-awesome-color-picker";
    import ColorPickerWrapper from "./ColorPickerWrapper.svelte";
    import Input from "./Input.svelte";
    import Button from "./Button.svelte";
    import { clamp, remEuclid } from "shared-lib/util";

    function random(min: number, max: number) {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    export let maxStops: number;

    let rotateKnob: HTMLDivElement;
    let gradientAngle = 90;
    let isRotating = false;

    let sliderContainer: HTMLDivElement;

    let gradientPositions = [10, 20, 30];
    let gradientColors = ["#ff0000", "#00ff00", "#0000ff"];

    // $: {
    //     gradientPositions = gradientPositions.sort((a, b) => a - b);
    // }

    $: gradientColorListString = gradientPositions
        .map((pos, idx) => [pos, gradientColors[idx]] as [number, string])
        .sort(([a], [b]) => a - b)
        .map(([pos, color]) => `${color} ${pos}%`)
        .join(", ");

    export let rotatedGradientString;
    // $: gradientString = `linear-gradient(${gradientAngle}deg, ${gradientPositions.map((pos, idx) => `${gradientColors[idx]} ${pos}%`).join(", ")})`;
    $: rotatedGradientString = `linear-gradient(${gradientAngle}deg, ${gradientColorListString})`;
    $: previewGradientString = `linear-gradient(90deg, ${gradientColorListString})`;

    let clickedNub = false;

    const handlePointerDown = (
        e: PointerEvent & { currentTarget: EventTarget & HTMLDivElement }
    ) => {
        if ((e.target as HTMLElement | null)?.classList.contains("rangeNub")) {
            clickedNub = true;
        }
    };

    /*
    
        on:pointerdown={handlePointerDown}
        on:pointerup={e => {
            if (clickedNub) {
                clickedNub = false;
                return;
            }

            console.log("ADD STOP", e);
        }}
    */
</script>

<svelte:window
    on:pointerup={() => {
        isRotating = false;
    }}
    on:pointermove={e => {
        if (!isRotating) return;

        const rect = rotateKnob.getBoundingClientRect();

        // Calculate the center of the circle
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 2;

        // Get the mouse position relative to the document
        const mouseX = e.clientX;
        const mouseY = e.clientY;

        const deltaX = mouseX - centerX;
        const deltaY = mouseY - centerY;

        let angle =
            ((Math.atan2(deltaY, deltaX) + Math.PI / 2) * 180) / Math.PI;

        gradientAngle = Math.round(remEuclid(angle, 360));
    }}
/>

<div class="grid w-full h-full pointer-events-auto">
    <div
        class="h-16 min-h-0 cursor-copy"
        style={`--bg: ${previewGradientString}`}
        on:pointerdown={handlePointerDown}
        on:pointerup={e => {
            if (clickedNub) {
                clickedNub = false;
                return;
            }

            if (gradientPositions.length > maxStops - 1) return;

            let rect = sliderContainer.getBoundingClientRect();
            let p = Math.round(
                clamp(((e.clientX - rect.left) / rect.width) * 100, 0, 100)
            );

            gradientPositions.push(p);
            gradientColors.push("#ffffff");

            gradientPositions = gradientPositions;
            gradientColors = gradientColors;
        }}
        bind:this={sliderContainer}
    >
        <RangeSlider
            id="gradient-slider"
            bind:values={gradientPositions}
            springValues={{ stiffness: 1, damping: 1 }}
            step={1}
            min={0}
            max={100}
            float
            hoverable={false}
        ></RangeSlider>
    </div>
    <div class="absolute dark left-1/4">
        <div id="color-picker-portal" />
    </div>

    <div class="flex w-full min-h-0 mt-16 overflow-hidden">
        <div class="flex flex-col items-center justify-center gap-2 pr-4">
            <div
                class="relative flex flex-center"
                on:pointerdown={() => {
                    isRotating = true;
                }}
                bind:this={rotateKnob}
            >
                <div
                    class="box-content flex w-10 border-2 border-white rounded-full cursor-pointer aspect-square flex-center"
                >
                    <div
                        class="relative w-2 h-full"
                        style:transform={`rotate(${gradientAngle}deg)`}
                    >
                        <span
                            class="absolute w-2 bg-white rounded-full top-1 aspect-square"
                        ></span>
                    </div>
                </div>
                <input
                    type="range"
                    min={0}
                    max={360}
                    aria-label="Gradient Angle"
                    class="absolute sr-only"
                    bind:value={gradientAngle}
                />
            </div>
            <Input
                maxLength={3}
                bind:value={gradientAngle}
                class="p-2 text-center rounded-lg outline-none w-14 text-stroke bg-black/40 outline-2 outline outline-white/20 -outline-offset-2"
            />
        </div>
        <ul
            class="overflow-y-scroll rounded-lg alternating-bg stop-list thin-scrollbar"
        >
            {#each gradientPositions
                .map((pos, idx) => [pos, idx])
                .sort(([a], [b]) => a - b) as [pos, idx]}
                <li class="grid grid-cols-3">
                    <div
                        class="flex items-center justify-center flex-auto p-1 gradient-picker-color"
                    >
                        <ColorPicker
                            bind:hex={gradientColors[idx]}
                            label=""
                            isAlpha={false}
                            components={{ wrapper: ColorPickerWrapper }}
                            textInputModes={["hex"]}
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <Input
                            maxLength={3}
                            bind:value={gradientPositions[idx]}
                            class="w-full p-2 text-center rounded-lg outline-none text-stroke bg-black/40 outline outline-white/20 -outline-offset-2"
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <Button
                            type="decline"
                            class="w-8 aspect-square"
                            on:click={() => {
                                if (gradientPositions.length <= 2) return;

                                gradientPositions.splice(idx, 1);
                                gradientColors.splice(idx, 1);
                                gradientPositions = gradientPositions;
                                gradientColors = gradientColors;
                            }}
                        />
                    </div>
                </li>
            {/each}
        </ul>
    </div>
</div>

<style lang="postcss">
    .dark {
        --cp-bg-color: #333;
        --cp-border-color: white;
        --cp-text-color: white;
        --cp-input-color: #555;
        --cp-button-hover-color: #777;
    }

    :global(#gradient-slider) {
        --range-handle: white;
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        @apply pointer-events-none m-0 h-full rounded-md border-2 border-black;
        background: var(--bg);
    }

    :global(#gradient-slider .rangeHandle) {
        @apply pointer-events-auto bottom-0 top-0 h-full w-2 -translate-x-1/2 translate-y-0 rounded-md;
    }

    :global(#gradient-slider .rangeNub) {
        @apply cursor-move rounded-md bg-transparent outline outline-4 outline-white ring-2 ring-black ring-offset-4;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-4 h-8 w-12 -translate-x-1/2 translate-y-0 rounded-md border-2 border-white bg-black text-white opacity-100 transition-none;
    }

    .gradient-picker-color :global(.container) :global(.alpha) {
        background: none;
    }
</style>
