<!--
    https://github.com/sveltejs/svelte/issues/9203
    color tab goes over 14 each blocks so it has to be separated
-->
<script lang="ts">
    import { default as cx } from "classnames";
    import RangeSlider from "svelte-range-slider-pips";
    import { AnimateSharedLayout } from "svelte-motion";

    import ToggleSwitch from "../../components/ToggleSwitch.svelte";
    import HueSlider from "../../components/HueSlider.svelte";
    import PaletteGrid from "../../components/PaletteGrid.svelte";
    import SlidingSelector from "../../components/SlidingSelector.svelte";

    import { colors } from "shared-lib/gd";

    import { menuSettings } from "../../stores";

    enum ColorTab {
        Main,
        Detail,
    }

    let selectedTab: ColorTab = ColorTab.Main;

    $: currentColor =
        selectedTab == ColorTab.Main
            ? $menuSettings.selectedMainColor
            : $menuSettings.selectedDetailColor;

    $: currentRgb =
        colors.list[currentColor.hue].palette[currentColor.y][currentColor.x];

    $: isBlack = currentRgb[0] == 0 && currentRgb[1] == 0 && currentRgb[2] == 0;

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;

    $: {
        if (isBlack)
            // if we use `currentColor` here svelte complains of a cyclic dependency ¯\_(ツ)_/¯
            (selectedTab == ColorTab.Main
                ? $menuSettings.selectedMainColor
                : $menuSettings.selectedDetailColor
            ).blending = false;
    }
</script>

<div
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items colors-tab-container md:text-lg sm:text-md"
>
    <ul class="flex flex-col h-full buttons">
        <AnimateSharedLayout>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke xs:text-sm"
                    on:click={() => (selectedTab = ColorTab.Main)}
                    tabindex={canSelectByTab}
                    aria-label="Main Color Channel"
                >
                    Main
                </button>
                {#if selectedTab == ColorTab.Main}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke xs:text-sm"
                    on:click={() => (selectedTab = ColorTab.Detail)}
                    tabindex={canSelectByTab}
                    aria-label="Detail Color Channel"
                >
                    Detail
                </button>
                {#if selectedTab == ColorTab.Detail}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
        </AnimateSharedLayout>
    </ul>

    <div class="flex flex-col justify-center h-full gap-8 xs:gap-6 sliders">
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
                    values={[$menuSettings.selectedMainColor.opacity]}
                    on:change={e => {
                        $menuSettings.selectedMainColor.opacity =
                            e.detail.value;
                    }}
                    pips
                    disabled={$menuSettings.isMinimized}
                    ariaLabels={["Main Channel Opacity"]}
                />
            {:else}
                <RangeSlider
                    min={0.2}
                    max={1}
                    step={0.1}
                    hoverable={false}
                    id={"opacity-slider"}
                    values={[$menuSettings.selectedDetailColor.opacity]}
                    on:change={e => {
                        $menuSettings.selectedDetailColor.opacity =
                            e.detail.value;
                    }}
                    pips
                    disabled={$menuSettings.isMinimized}
                    ariaLabels={["Detail Channel Opacity"]}
                />
            {/if}
        </div>

        {#if selectedTab == ColorTab.Main}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider
                    bind:currentHue={$menuSettings.selectedMainColor.hue}
                    tabIndex={canSelectByTab}
                ></HueSlider>
            </div>
        {:else}
            <div class="flex w-full h-3 md:h-5 hue">
                <HueSlider
                    bind:currentHue={$menuSettings.selectedDetailColor.hue}
                    tabIndex={canSelectByTab}
                ></HueSlider>
            </div>
        {/if}
    </div>

    <div class="flex flex-center blending">
        <div class="flex flex-col items-center">
            {#if selectedTab == ColorTab.Main}
                <ToggleSwitch
                    id="blending_cb"
                    bind:isToggled={$menuSettings.selectedMainColor.blending}
                    disabled={isBlack}
                    tabIndex={canSelectByTab}
                ></ToggleSwitch>
            {:else}
                <ToggleSwitch
                    id="blending_cb"
                    bind:isToggled={$menuSettings.selectedDetailColor.blending}
                    disabled={isBlack}
                    tabIndex={canSelectByTab}
                ></ToggleSwitch>
            {/if}
            <h2 class="font-pusab text-stroke xs:text-sm">Blending</h2>
        </div>
    </div>

    <div class="h-full palette">
        {#if selectedTab == ColorTab.Main}
            <PaletteGrid
                bind:hue={$menuSettings.selectedMainColor.hue}
                bind:currentRow={$menuSettings.selectedMainColor.y}
                bind:currentColumn={$menuSettings.selectedMainColor.x}
                tabIndex={canSelectByTab}
            />
        {:else}
            <PaletteGrid
                bind:hue={$menuSettings.selectedDetailColor.hue}
                bind:currentRow={$menuSettings.selectedDetailColor.y}
                bind:currentColumn={$menuSettings.selectedDetailColor.x}
                tabIndex={canSelectByTab}
            />
        {/if}
    </div>
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
