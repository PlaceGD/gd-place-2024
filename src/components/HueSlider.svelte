<script lang="ts">
    import { default as cx } from "classnames";
    import { onMount } from "svelte";
    import {
        AnimateSharedLayout,
        Motion,
        useAnimation,
        useReducedMotion,
    } from "svelte-motion";
    import { clamp } from "shared-lib/util";
    import { colors } from "shared-lib/gd";
    import { spring, type Spring } from "svelte/motion";

    export let currentHue: number;

    export let tabIndex: number = 0;

    // "slider"
    let sliderKnob: HTMLDivElement;

    let sliderContainer: HTMLUListElement;
    let sliderContainerWidth: number = 0;

    $: sliderContainerWidth && moveSliderKnob(currentHue);

    let isPressingSlider = false;

    const calcKnobOffset = (to: number): number => {
        let colorLen = sliderContainerWidth / colors.hues;

        let offset = colorLen * (currentHue + 0.5);

        return offset - sliderKnob.offsetWidth / 2;
    };

    let springPos: Spring<number>;
    onMount(() => {
        springPos = spring(calcKnobOffset(currentHue), {
            stiffness: 0.15,
            damping: 0.4,
        });
    });

    const moveSliderKnob = (to: number) => {
        if (springPos != null) {
            springPos.set(calcKnobOffset(to));
        }
    };

    const handleDrag = (e: PointerEvent) => {
        if (isPressingSlider) {
            let ex = e.clientX - sliderContainer.getBoundingClientRect().left;

            let colorLen = sliderContainerWidth / colors.hues;

            let color = clamp(Math.floor(ex / colorLen), 0, colors.hues - 1);

            currentHue = color;
        }
    };

    $: {
        if (sliderKnob != null) {
            sliderKnob.style.left = `${$springPos}px`;
        }
    }
</script>

<svelte:window
    on:pointermove={handleDrag}
    on:pointerup={() => {
        isPressingSlider = false;
    }}
/>

<ul
    class="relative flex w-full h-full divide-x divide-black touch-none"
    bind:this={sliderContainer}
    bind:offsetWidth={sliderContainerWidth}
    on:pointerdown={() => {
        isPressingSlider = true;
    }}
>
    {#each colors.list as { hue }, i}
        <li
            class={cx({
                "w-auto h-full flex-1 transition-all flex": true,
                "rounded-l-lg": i == 0,
                "rounded-r-lg": i == colors.hues - 1,
            })}
            style={`background: hsl(${hue}, 100%, 50%)`}
        >
            <button
                class="w-full h-full"
                on:click={() => {
                    currentHue = i;
                }}
                tabindex={tabIndex}
                aria-label={`Color with Hue ${currentHue}`}
            ></button>
        </li>
    {/each}
    <div
        class="absolute bottom-auto w-auto h-full p-2 bg-white !border-2 !border-black rounded-full slider-knob -translate-y-2/4 top-1/2"
        bind:this={sliderKnob}
    />
</ul>

<style>
    .slider-knob {
        transition: transform ease-in-out;
    }
</style>
