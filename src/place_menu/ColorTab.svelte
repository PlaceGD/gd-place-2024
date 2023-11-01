<!--
    https://github.com/sveltejs/svelte/issues/9203
    color tab goes over 14 each blocks so it has to be separated
-->
<script lang="ts">
    import { default as cx } from "classnames";
    import RangeSlider from "svelte-range-slider-pips";

    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import { PALETTE } from "./edit_tab";
    import OpacitySlider from "../components/OpacitySlider.svelte";
    import HueSlider from "../components/HueSlider.svelte";

    export let currentMainColor: string;
    export let currentDetailColor: string;

    enum ColorTab {
        Main,
        Detail,
    }
    export let selectedTab: ColorTab = ColorTab.Main;
</script>

<div
    class="items-center w-full h-full p-4 text-xl gap-x-4 items colors-tab-container"
>
    <ul class="flex flex-col h-full buttons">
        <li class="flex-1 flex-center font-pusab">
            <button class="main text-stroke">Main</button>
        </li>
        <li class="flex-1 flex-center font-pusab">
            <button class="detail text-stroke">Detail</button>
        </li>
    </ul>

    <div
        class="flex w-full h-3 opacity opacity-slider-container"
        style={`--currentColor: #${
            selectedTab == ColorTab.Main ? currentMainColor : currentDetailColor
        }`}
    >
        <RangeSlider
            min={0.2}
            max={1}
            step={0.1}
            hoverable={false}
            id={"opacity-slider"}
        />
        <!-- <OpacitySlider
            currentColor={selectedTab == ColorTab.Main
                ? currentMainColor
                : currentDetailColor}
        ></OpacitySlider> -->
    </div>

    <div class="w-full h-3 hue">
        <HueSlider hueDivisions={10}></HueSlider>
    </div>

    <ul class="flex flex-wrap w-full palette">
        <!-- {#each PALETTE as col}
        <li class={"w-12 h-12"} style={`background: #${col}`}></li>
        {/each} -->
    </ul>
    <div class="flex flex-col blending">
        <ToggleSwitch id="blending_cb"></ToggleSwitch>
        <label for="blending_cb" class="font-pusab text-stroke">
            Blending
        </label>
    </div>
</div>

<style>
    .opacity-slider-container {
        background: repeating-conic-gradient(#474747 0% 25%, transparent 0% 50%)
            50% / 20px 20px;
        @apply rounded-lg;
    }

    :global(#opacity-slider) {
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        background: linear-gradient(
            to right,
            #00000000 0%,
            var(--currentColor) 100%
        );
        @apply m-0 h-full w-full;
    }
    :global(#opacity-slider .rangeHandle) {
        @apply rounded-full border-2 border-black;
    }
    :global(#opacity-slider .rangeHandle::before) {
        @apply aspect-square h-full;
    }

    .colors-tab-container {
        display: grid;
        grid-auto-columns: 1fr;
        grid-auto-rows: 1fr;
        grid-template-columns: min-content auto auto min-content;
        grid-template-areas:
            "buttons opacity palette blending"
            "buttons hue palette blending";
    }

    .palette {
        grid-area: palette;
    }
    .blending {
        grid-area: blending;
    }
    .buttons {
        grid-area: buttons;
    }
    .opacity {
        grid-area: opacity;
    }
    .hue {
        grid-area: hue;
    }
</style>
