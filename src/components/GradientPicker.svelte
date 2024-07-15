<!-- <script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import { default as cx } from "classnames";
    import ColorPicker from "svelte-awesome-color-picker";
    import ColorPickerWrapper from "./ColorPickerWrapper.svelte";
    import Input from "./Input.svelte";
    import Button from "./Button.svelte";
    import { clamp, map, remEuclid } from "shared-lib/util";

    export let maxStops: number;

    let rotateKnob: HTMLDivElement;

    let sliderContainer: HTMLButtonElement;
    let sliderStops: HTMLDivElement[] = [];

    let isDraggingStop = false;

    let gradientAngle = 90;
    let isRotating = false;

    let gradientPositions = [0, 50, 100];
    let gradientColors = ["#ff0000", "#00ff00", "#0000ff"];

    let selectedStop: number | null = null;

    $: gradientColorListString = gradientPositions
        .map((pos, idx) => [pos, gradientColors[idx]] as [number, string])
        .sort(([a], [b]) => a - b)
        .map(([pos, color]) => `${color} ${pos}%`)
        .join(", ");

    export let rotatedGradientString;
    $: rotatedGradientString = `linear-gradient(${gradientAngle}deg, ${gradientColorListString})`;
    $: previewGradientString = `linear-gradient(90deg, ${gradientColorListString})`;

    const handleSliderDrag = (e: PointerEvent) => {
        if (isDraggingStop) {
            const rect = sliderContainer.getBoundingClientRect();

            const ex = e.clientX - rect.left;

            const perc = map(ex, 0, rect.left, 0, 100);

            // sliderStops[selectedStop!].style.left = `${ex}px`;
            gradientPositions[selectedStop!] = perc;
        }
    };

    const handleAddStop = (e: PointerEvent) => {
        if (gradientPositions.length > maxStops - 1) return;

        let rect = sliderContainer.getBoundingClientRect();
        let p = Math.round(
            clamp(((e.clientX - rect.left) / rect.width) * 100, 0, 100)
        );

        gradientPositions.push(p);
        gradientColors.push("#ffffff");

        gradientPositions = gradientPositions;
        gradientColors = gradientColors;
    };

    const handleRotating = (e: PointerEvent) => {
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
    };
</script>

<svelte:window
    on:pointerup={() => {
        isDraggingStop = false;
        isRotating = false;
    }}
    on:pointermove={e => {
        if (isRotating) {
            handleRotating(e);
        }
        if (isDraggingStop) {
            handleSliderDrag(e);
        }
    }}
/>

<div class="grid w-full h-full pointer-events-auto gradient-picker">
    <button
        class="relative flex min-h-0 mb-16 border-2 border-black rounded-md h-14 xs:h-14 cursor-copy xs:mb-14"
        style:background={`${previewGradientString}`}
        bind:this={sliderContainer}
        on:pointerup={e => {
            if (e.button !== 0) return;
            if (isDraggingStop) return;

            handleAddStop(e);
        }}
        aria-label="Add gradient stop"
    >
        {#each gradientPositions as pos, idx}
            <div
                class={cx({
                    "absolute h-[calc(100%_+_12px)] p-1 transition-shadow -translate-x-1/2 bottom-1/2 translate-y-1/2 border-4 border-white rounded-full outline outline-2 outline-black hover:ring-4 ring-black/20 cursor-move": true,
                    "ring-4 ring-white": selectedStop === idx,
                })}
                style:left={`${(sliderContainer?.clientWidth ?? 0) * (pos / 100)}px`}
                style:background={`${gradientColors[idx]}`}
                on:pointerdown={e => {
                    e.stopPropagation();
                    if (e.button !== 0) return;

                    selectedStop = idx;
                    isDraggingStop = true;
                }}
                bind:this={sliderStops[idx]}
            />
        {/each}
    </button>
    <div class="absolute dark left-1/4">
        <div id="color-picker-portal" />
    </div>

    <div class="flex w-full h-full min-h-0 overflow-hidden">
        <div class="flex flex-col items-center justify-center gap-2 pr-4">
            <div
                class="relative flex flex-center"
                on:pointerdown={() => {
                    isRotating = true;
                }}
                bind:this={rotateKnob}
            >
                <div
                    class="box-content flex w-10 border-2 border-white rounded-full cursor-pointer xs:w-9 aspect-square flex-center"
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
                class="p-2 text-base text-center rounded-lg outline-none xs:p-1 w-14 text-stroke bg-black/40 outline-2 outline outline-white/20 -outline-offset-2 xs:text-sm"
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
                            class="w-full p-2 text-base text-center rounded-lg outline-none xs:p-1 text-stroke bg-black/40 outline outline-white/20 -outline-offset-2 xs:text-sm"
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
    .gradient-picker {
        grid-template-rows: min-content 1fr;
    }

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
        @apply relative cursor-move  rounded-md bg-transparent;
    }

    :global(#gradient-slider .rangeNub::before) {
        content: "";
        height: calc(100% + 8px);
        @apply absolute bottom-1/2 right-1/2 w-full translate-x-1/2 translate-y-1/2 rounded-full border-4 border-white bg-transparent p-1 outline outline-2 outline-black;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-3 flex h-7 w-11 -translate-x-1/2 translate-y-0 cursor-move items-center justify-center rounded-md border-2 border-white bg-black text-base text-white opacity-100 transition-none xs:text-sm;
    }

    .gradient-picker-color :global(.container) :global(.alpha) {
        background: none;
    }
</style> -->

<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import ColorPicker from "svelte-awesome-color-picker";
    import ColorPickerWrapper from "./ColorPickerWrapper.svelte";
    import Input from "./Input.svelte";
    import Button from "./Button.svelte";
    import { clamp, remEuclid } from "shared-lib/util";

    export let maxStops: number;

    let rotateKnob: HTMLDivElement;
    let gradientAngle = 90;
    let isRotating = false;

    let sliderContainer: HTMLDivElement;

    let gradientPositions = [0, 50, 100];
    let gradientColors = ["#ff0000", "#00ff00", "#0000ff"];

    $: gradientColorListString = gradientPositions
        .map((pos, idx) => [pos, gradientColors[idx]] as [number, string])
        .sort(([a], [b]) => a - b)
        .map(([pos, color]) => `${color} ${pos}%`)
        .join(", ");

    export let rotatedGradientString;
    $: rotatedGradientString = `linear-gradient(${gradientAngle}deg, ${gradientColorListString})`;
    $: previewGradientString = `linear-gradient(90deg, ${gradientColorListString})`;

    let clickedNub = false;

    const handlePointerDown = (
        e: PointerEvent & { currentTarget: EventTarget & HTMLDivElement }
    ) => {
        const target = e.target as HTMLElement | null;
        if (
            target != null &&
            (target.classList.contains("rangeNub") ||
                target.classList.contains("rangeFloat"))
        ) {
            clickedNub = true;
        }
    };
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

<div class="grid w-full h-full pointer-events-auto gradient-picker">
    <div
        class="h-16 min-h-0 mb-16 xs:h-14 cursor-copy xs:mb-14"
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

    <div class="flex w-full h-full min-h-0 overflow-hidden">
        <div class="flex flex-col items-center justify-center gap-2 pr-4">
            <div
                class="relative flex flex-center"
                on:pointerdown={() => {
                    isRotating = true;
                }}
                bind:this={rotateKnob}
            >
                <div
                    class="box-content flex w-10 border-2 border-white rounded-full cursor-pointer xs:w-9 aspect-square flex-center"
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
                class="p-2 text-base text-center rounded-lg outline-none xs:p-1 w-14 text-stroke bg-black/40 outline-2 outline outline-white/20 -outline-offset-2 xs:text-sm"
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
                            class="w-full p-2 text-base text-center rounded-lg outline-none xs:p-1 text-stroke bg-black/40 outline outline-white/20 -outline-offset-2 xs:text-sm"
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
    .gradient-picker {
        grid-template-rows: min-content 1fr;
    }

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
        @apply relative cursor-move  rounded-md bg-transparent;
    }

    :global(#gradient-slider .rangeNub::before) {
        content: "";
        height: calc(100% + 8px);
        @apply absolute bottom-1/2 right-1/2 w-full translate-x-1/2 translate-y-1/2 rounded-full border-4 border-white bg-transparent p-1 outline outline-2 outline-black ring-black/50 transition-shadow hover:ring-4 active:ring-white active:hover:ring-4;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-3 flex h-7 w-11 -translate-x-1/2 translate-y-0 cursor-move items-center justify-center rounded-md border-2 border-white bg-black text-base text-white opacity-100 transition-none xs:text-sm;
    }

    .gradient-picker-color :global(.container) :global(.alpha) {
        background: none;
    }
</style>
