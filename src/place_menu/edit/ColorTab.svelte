<!--
    https://github.com/sveltejs/svelte/issues/9203
    color tab goes over 14 each blocks so it has to be separated
-->
<script lang="ts">
    import { default as cx } from "classnames";
    import RangeSlider from "svelte-range-slider-pips";

    import ToggleSwitch from "../../components/ToggleSwitch.svelte";
    import HueSlider from "../../components/HueSlider.svelte";
    import PaletteGrid from "../../components/PaletteGrid.svelte";
    import SlidingSelector from "../../components/SlidingSelector.svelte";
    import { AnimateSharedLayout } from "svelte-motion";
    import colors from "../../gd/colors.json";

    export let currentMainColor: {
        hue: number;
        x: number;
        y: number;
        opacity: number;
        blending: boolean;
    };
    export let currentDetailColor: {
        hue: number;
        x: number;
        y: number;
        opacity: number;
        blending: boolean;
    };

    enum ColorTab {
        Main,
        Detail,
    }
    export let selectedTab: ColorTab = ColorTab.Main;

    $: currentColor =
        selectedTab == ColorTab.Main ? currentMainColor : currentDetailColor;
    $: currentRgb =
        colors.list[currentColor.hue].palette[currentColor.y][currentColor.x];
</script>

<div
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items colors-tab-container md:text-lg sm:text-md"
>
    <ul class="flex flex-col h-full buttons">
        <AnimateSharedLayout>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                    on:click={() => (selectedTab = ColorTab.Main)}>Main</button
                >
                {#if selectedTab == ColorTab.Main}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke"
                    on:click={() => (selectedTab = ColorTab.Detail)}
                    >Detail</button
                >
                {#if selectedTab == ColorTab.Detail}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
        </AnimateSharedLayout>
    </ul>

    <div class="flex flex-col justify-center h-full gap-8 options">
        <div
            class="flex w-full h-3 md:h-5 opacity opacity-slider-container"
            style={`
                --currentColor: rgb(${currentRgb.join(", ")});
                --currentColorFaded: rgb(${currentRgb.join(", ")}, 0.2);
            `}
        >
            {#if selectedTab == ColorTab.Main}
                <RangeSlider
                    min={0.2}
                    max={1}
                    step={0.1}
                    hoverable={false}
                    id={"opacity-slider"}
                    values={[currentMainColor.opacity]}
                    on:change={e => {
                        currentMainColor.opacity = e.detail.value;
                    }}
                    pips
                />
            {:else}
                <RangeSlider
                    min={0.2}
                    max={1}
                    step={0.1}
                    hoverable={false}
                    id={"opacity-slider"}
                    values={[currentDetailColor.opacity]}
                    on:change={e => {
                        currentDetailColor.opacity = e.detail.value;
                    }}
                    pips
                />
            {/if}
        </div>

        {#if selectedTab == ColorTab.Main}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider bind:currentHue={currentMainColor.hue}></HueSlider>
            </div>
        {:else}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider bind:currentHue={currentDetailColor.hue}></HueSlider>
            </div>
        {/if}

        <div class="flex flex-center">
            <div class="flex flex-col items-center">
                {#if selectedTab == ColorTab.Main}
                    <ToggleSwitch
                        id="blending_cb"
                        bind:isToggled={currentMainColor.blending}
                    ></ToggleSwitch>
                {:else}
                    <ToggleSwitch
                        id="blending_cb"
                        bind:isToggled={currentDetailColor.blending}
                    ></ToggleSwitch>
                {/if}
                <label for="blending_cb" class="font-pusab text-stroke">
                    Blending
                </label>
            </div>

            <!-- <div class="color-preview flex-center">
                <div
                    class="w-10 h-10 rounded-full"
                    style={`
                        background: rgba(${currentRgb.join(", ")}, ${
                            currentColor.opacity
                        });
                    `}
                />
            </div> -->
        </div>
    </div>

    <div class="h-full palette">
        {#if selectedTab == ColorTab.Main}
            <PaletteGrid
                hue={currentMainColor.hue}
                bind:currentRow={currentMainColor.y}
                bind:currentColumn={currentMainColor.x}
            />
        {:else}
            <PaletteGrid
                hue={currentDetailColor.hue}
                bind:currentRow={currentDetailColor.y}
                bind:currentColumn={currentDetailColor.x}
            />
        {/if}
    </div>
    <!-- <div class="w-full h-full color-preview flex-center">
        <div
            class="w-12 h-12 rounded-full md:h-10 md:w-10"
            style={`
                background: rgba(${currentRgb.join(", ")}, ${
                    currentColor.opacity
                });
            `}
        />
    </div> -->
</div>

<style lang="postcss">
    .opacity-slider-container {
        background: repeating-conic-gradient(#474747 0% 25%, transparent 0% 50%)
            50% / 20px 20px;
        @apply rounded-lg;
    }

    :global(#opacity-slider) {
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        --range-pip: #ffffff4a;
        --range-pip-active: #ffffff4a;
        background: linear-gradient(
            to right,
            var(--currentColorFaded) 0%,
            var(--currentColor) 100%
        );
        @apply m-0 h-full w-full;
    }
    :global(#opacity-slider .rangeHandle) {
        @apply top-1/2 aspect-square h-full w-auto rounded-full border-2 border-black p-2;
    }
    :global(#opacity-slider .rangeHandle::before) {
        @apply aspect-square h-full;
    }
    :global(#opacity-slider .rangePips .pip.selected) {
        height: 0.4em;
    }

    .colors-tab-container {
        display: grid;
        grid-auto-columns: 1fr;
        grid-auto-rows: 1fr;
        grid-template-columns: min-content 1fr 1fr;
        grid-template-areas:
            "buttons options palette"
            "buttons options palette";
    }

    @media screen(lg) {
        .colors-tab-container {
            grid-template-columns: min-content 1.3fr 0.7fr;
        }
    }

    .palette {
        grid-area: palette;
    }
    .options {
        grid-area: options;
    }
    .blending {
        grid-area: blending;
    }
    .buttons {
        grid-area: buttons;
    }
    .color-preview {
        grid-area: color-preview;
    }
    .color-preview > * {
        box-shadow:
            10px -10px 0px 0px rgba(0, 0, 0, 0.2) inset,
            0px 0px 0px 4px #000,
            0px 0px 0px 6px #fff;
    }
</style>
