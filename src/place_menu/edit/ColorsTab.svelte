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

    import { colors } from "shared-lib/gd";

    import {
        menuDetailColor,
        menuMainColor,
        menuMinimized,
        menuSelectedObject,
    } from "../../stores";
    import { COLOR_TRIGGERS } from "shared-lib/nexusgen";

    import * as wasm from "wasm-lib";
    import { GUIDE_ELEM_IDS } from "../../guide/guide";
    export let state: wasm.State;

    enum ColorTab {
        Main,
        Detail,
    }

    let selectedTab: ColorTab = ColorTab.Main;

    $: currentColor =
        selectedTab == ColorTab.Main ? $menuMainColor : $menuDetailColor;

    $: currentRgb =
        colors.list[currentColor.hue].palette[currentColor.y][currentColor.x];

    $: isBlack = currentRgb[0] == 0 && currentRgb[1] == 0 && currentRgb[2] == 0;

    const handleOnBlack = (b: boolean) => {
        if (b) {
            if (selectedTab == ColorTab.Main) {
                $menuMainColor.blending = false;
            } else {
                $menuDetailColor.blending = false;
            }
        }
    };

    $: colorTriggerSelected = COLOR_TRIGGERS.includes($menuSelectedObject);

    $: handleOnBlack(isBlack);
</script>

<fieldset
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items colors-tab-container md:text-lg sm:text-base"
    disabled={$menuMinimized}
    data-guide={GUIDE_ELEM_IDS.colorsTab}
>
    <ul class="flex flex-col h-full buttons">
        <li class="relative flex-1 w-full h-full flex-center font-pusab">
            <button
                class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke xs:text-sm"
                on:click={() => (selectedTab = ColorTab.Main)}
                aria-label="Main Color Channel"
                title="Main Color Channel"
            >
                Main
            </button>
            {#if selectedTab == ColorTab.Main}
                <div class="sliding-selector"></div>
            {/if}
        </li>
        {#if !colorTriggerSelected}
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke xs:text-sm"
                    on:click={() => (selectedTab = ColorTab.Detail)}
                    aria-label="Detail Color Channel"
                    title="Detail Color Channel"
                >
                    Detail
                </button>
                {#if selectedTab == ColorTab.Detail}
                    <div class="sliding-selector"></div>
                {/if}
            </li>
        {/if}
    </ul>

    <div class="flex flex-col justify-center h-full gap-8 xs:gap-6 sliders">
        {#if !colorTriggerSelected}
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
                        values={[$menuMainColor.opacity]}
                        on:change={e => {
                            $menuMainColor.opacity = e.detail.value;
                        }}
                        pips
                        disabled={$menuMinimized}
                        ariaLabels={["Main Channel Opacity"]}
                    />
                {:else}
                    <RangeSlider
                        min={0.2}
                        max={1}
                        step={0.1}
                        hoverable={false}
                        id={"opacity-slider"}
                        values={[$menuDetailColor.opacity]}
                        on:change={e => {
                            $menuDetailColor.opacity = e.detail.value;
                        }}
                        pips
                        disabled={$menuMinimized}
                        ariaLabels={["Detail Channel Opacity"]}
                    />
                {/if}
            </div>
        {/if}
        {#if selectedTab == ColorTab.Main}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider bind:currentHue={$menuMainColor.hue} bind:state />
            </div>
        {:else}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider bind:currentHue={$menuDetailColor.hue} bind:state />
            </div>
        {/if}
    </div>

    <div class="flex flex-center blending">
        <div class="flex flex-col items-center">
            {#if selectedTab == ColorTab.Main}
                <ToggleSwitch
                    id="blending_cb"
                    bind:isToggled={$menuMainColor.blending}
                    disabled={isBlack || colorTriggerSelected}
                    aria-label="Toggle Blending for Main Channel"
                ></ToggleSwitch>
            {:else}
                <ToggleSwitch
                    id="blending_cb"
                    bind:isToggled={$menuDetailColor.blending}
                    disabled={isBlack || colorTriggerSelected}
                    aria-label="Toggle Blending for Detail Channel"
                ></ToggleSwitch>
            {/if}
            <h2 class="font-pusab text-stroke xs:text-sm">Blending</h2>
        </div>
    </div>

    <div class="h-full palette">
        {#if selectedTab == ColorTab.Main}
            <PaletteGrid
                bind:hue={$menuMainColor.hue}
                bind:currentRow={$menuMainColor.y}
                bind:currentColumn={$menuMainColor.x}
                bind:state
            />
        {:else}
            <PaletteGrid
                bind:hue={$menuDetailColor.hue}
                bind:currentRow={$menuDetailColor.y}
                bind:currentColumn={$menuDetailColor.x}
                bind:state
            />
        {/if}
    </div>
</fieldset>

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
        @apply top-1/2 h-full w-auto rounded-full border-2 border-black p-2;
    }
    :global(#opacity-slider .rangeHandle::before) {
        @apply h-full;
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
            "buttons sliders palette"
            "buttons blending palette";
    }

    @media screen(lg) {
        .colors-tab-container {
            grid-template-columns: min-content 1.3fr 0.7fr;
        }
    }
    @media screen(xs) {
        .colors-tab-container {
            grid-template-columns: min-content 1fr min-content;
            grid-template-areas:
                "buttons sliders blending"
                "buttons palette palette";
        }
    }

    .palette {
        grid-area: palette;
    }
    /* .options {
        grid-area: options;
    } */
    .sliders {
        grid-area: sliders;
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
