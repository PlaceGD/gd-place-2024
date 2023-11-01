<script lang="ts">
    import { default as cx } from "classnames";
    import { onMount } from "svelte";
    import {
        AnimateSharedLayout,
        Motion,
        useAnimation,
        useReducedMotion,
    } from "svelte-motion";

    export let hueDivisions: number;
    let hueSteps = 360 / hueDivisions;

    let colors: string[] = [];
    for (let i = 0; i < hueDivisions; i++) {
        colors.push(`hsl(${i * hueSteps}, 100%, 50%)`);
    }

    let selectedColor = 0;

    // "slider"
    // let sliderContainer: HTMLUListElement;
    let sliderKnob: HTMLDivElement;

    let sliderContainerWidth: number = 0;

    $: sliderContainerWidth && moveSliderKnob(selectedColor);

    const moveSliderKnob = (to: number) => {
        let colorLen = sliderContainerWidth / hueDivisions;

        let base = colorLen * (to + 0.5);

        base -= sliderKnob.clientWidth / 2;

        sliderKnob.style.left = `${base + 0.5}px`;
    };

    //divide-x divide-black
</script>

<ul class="relative flex w-full h-full" bind:offsetWidth={sliderContainerWidth}>
    {#each colors as c, i}
        <li
            class={cx({
                "flex-1 transition-all flex": true,
                "rounded-l-lg": i == 0,
                "rounded-r-lg": i == colors.length - 1,
            })}
            style={`background: ${c}`}
            on:drag={e => {
                console.log(e);
            }}
        >
            <button
                class="w-full h-full"
                on:click={() => {
                    selectedColor = i;
                    moveSliderKnob(selectedColor);
                }}
            ></button>
        </li>
    {/each}
    <div
        class="absolute h-full bg-white rounded-full slider-knob aspect-square"
        bind:this={sliderKnob}
    />
</ul>
