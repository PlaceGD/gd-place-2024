<script lang="ts">
    import { afterUpdate, beforeUpdate } from "svelte";
    import RangeSlider from "svelte-range-slider-pips";
    import ColorPicker from "svelte-awesome-color-picker";
    import ColorPickerWrapper from "./ColorPickerWrapper.svelte";
    import Cross from "../icons/Cross.svelte";
    import { clamp, remEuclid } from "shared-lib/util";
    import DarkInput from "./DarkInput.svelte";
    import { notNaNAnd } from "../utils/misc";
    import Palette from "../icons/Palette.svelte";
    import WhiteButton from "./Buttons/WhiteButton.svelte";
    import { getRandomGradientColors } from "../utils/gradient";

    export let maxStops: number;

    let activeElement: HTMLElement | null;

    beforeUpdate(() => {
        if (
            document.activeElement != null &&
            !document.activeElement.isEqualNode(document.body)
        ) {
            activeElement = document.activeElement as HTMLElement;
        }
    });
    afterUpdate(() => {
        if (activeElement != null) {
            activeElement.focus();
        }
    });

    let rotateKnob: HTMLDivElement;
    let gradientAngle = 90;
    let isRotating = false;

    let sliderContainer: HTMLDivElement;

    export let gradientStops: number[];
    export let gradientColors: string[];
    export let gradientIDs: number[];

    let openColorPickers = gradientStops.map(() => false);

    $: gradientColorListString = gradientStops
        .map((pos, idx) => [pos, gradientColors[idx]] as [number, string])
        .sort(([a], [b]) => a - b)
        .map(([pos, color]) => `${color} ${pos}%`)
        .join(", ");

    export let rotatedGradientString;
    $: rotatedGradientString = `linear-gradient(${gradientAngle}deg, ${gradientColorListString})`;
    $: previewGradientString = `linear-gradient(90deg, ${gradientColorListString})`;

    let lastClickedElement: HTMLElement | null = null;
    const handleClickedElement = (
        e: PointerEvent & { currentTarget: EventTarget & Window }
    ) => {
        lastClickedElement = e.target as HTMLElement | null;
    };

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

    const NUMBER_HARD_VALID_INPUT = /^\d*$/;
    const STOP_POSITION_SOFT_VALID_INPUT = (s: string) => {
        return notNaNAnd(s, n => 0 <= n && n <= 100);
    };

    const GRAD_ANGLE_SOFT_VALID_INPUT = (s: string) => {
        return notNaNAnd(s, n => 0 <= n && n <= 360);
    };
</script>

<svelte:window
    on:pointerdown={handleClickedElement}
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
    <div class="flex flex-row w-full gap-2">
        <div class="flex flex-col items-center h-full gap-2 pr-6 sm:pr-4">
            <div class="flex items-center justify-center h-16 xs:h-12">
                <div
                    class="relative flex flex-center"
                    on:pointerdown={() => {
                        isRotating = true;
                    }}
                    bind:this={rotateKnob}
                >
                    <div
                        class="box-content flex w-12 border-2 border-white rounded-full cursor-pointer xs:w-9 aspect-square flex-center"
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
            </div>
            <DarkInput
                maxLength={3}
                softValidInput={GRAD_ANGLE_SOFT_VALID_INPUT}
                hardValidInput={NUMBER_HARD_VALID_INPUT}
                bind:value={gradientAngle}
                class="text-base w-14 xs:text-sm h-min"
            />
        </div>
        <div
            class="w-full h-16 min-h-0 mb-16 xs:h-12 cursor-copy xs:mb-5"
            style={`--bg: ${previewGradientString}`}
            on:pointerdown={handlePointerDown}
            on:pointerup={e => {
                // if you click on another element and drag onto the slider container
                // this will prevent a stop being added
                if (
                    lastClickedElement == null ||
                    !lastClickedElement.isEqualNode(sliderContainer)
                ) {
                    return;
                }

                if (clickedNub) {
                    clickedNub = false;
                    return;
                }

                if (gradientStops.length > maxStops - 1) return;

                let rect = sliderContainer.getBoundingClientRect();
                let p = Math.round(
                    clamp(((e.clientX - rect.left) / rect.width) * 100, 0, 100)
                );

                gradientStops.push(p);
                gradientColors.push("#ffffff");
                gradientIDs.push(Math.random());
                openColorPickers.push(false);

                gradientStops = gradientStops;
                gradientColors = gradientColors;
                gradientIDs = gradientIDs;
            }}
            bind:this={sliderContainer}
        >
            <RangeSlider
                id="gradient-slider"
                bind:values={gradientStops}
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
    </div>

    <div class="flex w-full h-full min-h-0">
        <ul
            class="overflow-y-auto rounded-lg alternating-bg stop-list thin-scrollbar"
        >
            {#each gradientStops
                .map((pos, idx) => [pos, idx])
                .sort(([a], [b]) => a - b) as [_, idx] (gradientIDs[idx])}
                <li class="grid grid-cols-3">
                    <div
                        class="flex items-center justify-center flex-auto p-1 cursor-pointer gradient-picker-color"
                        on:pointerup={() => {
                            openColorPickers[idx] = true;
                        }}
                    >
                        <Palette
                            stroke-width={1}
                            class="p-1 shrink-0 xs:w-9 xs:h-9"
                        />
                        <ColorPicker
                            bind:hex={gradientColors[idx]}
                            label=""
                            isAlpha={false}
                            components={{ wrapper: ColorPickerWrapper }}
                            textInputModes={["hex"]}
                            bind:isOpen={openColorPickers[idx]}
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <DarkInput
                            maxLength={3}
                            softValidInput={STOP_POSITION_SOFT_VALID_INPUT}
                            hardValidInput={NUMBER_HARD_VALID_INPUT}
                            bind:value={gradientStops[idx]}
                            class="w-full text-base xs:text-sm"
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <button
                            class="relative grid items-center w-8 rounded-lg justify-items-center aspect-square white-button"
                            on:click={() => {
                                if (gradientStops.length <= 2) return;

                                gradientStops.splice(idx, 1);
                                gradientColors.splice(idx, 1);
                                gradientIDs.splice(idx, 1);
                                openColorPickers.splice(idx, 1);
                                gradientStops = gradientStops;
                                gradientColors = gradientColors;
                                gradientIDs = gradientIDs;
                            }}
                        >
                            <Cross
                                class="text-[#ff4747] w-full h-full p-1 stroke-[1.5px]"
                            />
                        </button>
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
        @apply absolute bottom-1/2 right-1/2 w-full translate-x-1/2 translate-y-1/2 rounded-full border-4 border-white bg-transparent p-1 outline outline-2 outline-black ring-black/20 transition-shadow hover:ring-4 active:ring-4 active:ring-white;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-3 flex h-7 w-11 -translate-x-1/2 translate-y-0 cursor-move items-center justify-center rounded-md border-2 border-white bg-black text-base text-white opacity-100 transition-none xs:hidden xs:text-sm;
    }

    .gradient-picker-color :global(.container) :global(.alpha) {
        background: none;
    }
</style>
