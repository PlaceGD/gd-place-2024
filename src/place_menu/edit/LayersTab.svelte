<script lang="ts">
    import { onMount } from "svelte";
    import { ZLayer } from "wasm-lib";

    import Image from "../../components/Image.svelte";
    import Input from "../../components/Input.svelte";
    import SlidingSelector from "../../components/SlidingSelector.svelte";

    import { menuSettings } from "../../stores";
    import { clamp } from "shared-lib/util";

    const layerName = (layer: ZLayer) => {
        switch (layer) {
            case ZLayer.B1:
                return "B1";
            case ZLayer.B2:
                return "B2";
            case ZLayer.B3:
                return "B3";
            case ZLayer.B4:
                return "B4";
            case ZLayer.B5:
                return "B5";
            case ZLayer.T1:
                return "T1";
            case ZLayer.T2:
                return "T2";
            case ZLayer.T3:
                return "T3";
            case ZLayer.T4:
                return "T4";
            default:
                return "T1";
        }
    };
    const equivalentTop = (layer: ZLayer) => {
        switch (layer) {
            case ZLayer.B1:
                return ZLayer.T1;
            case ZLayer.B2:
                return ZLayer.T2;
            case ZLayer.B3:
                return ZLayer.T3;
            case ZLayer.B4:
                return ZLayer.T4;
            case ZLayer.B5:
                return ZLayer.T4;
            default:
                return layer;
        }
    };
    const equivalentBottom = (layer: ZLayer) => {
        switch (layer) {
            case ZLayer.T1:
                return ZLayer.B1;
            case ZLayer.T2:
                return ZLayer.B2;
            case ZLayer.T3:
                return ZLayer.B3;
            case ZLayer.T4:
                return ZLayer.B4;
            default:
                return layer;
        }
    };
    const BOTTOMS = [ZLayer.B1, ZLayer.B2, ZLayer.B3, ZLayer.B4, ZLayer.B5];
    const isBottom = (layer: ZLayer) => BOTTOMS.includes(layer);
    const layerCount = (bottom: boolean) => (bottom ? 5 : 4);
    const layerFrom = (bottom: boolean, idx: number) =>
        (bottom ? (v: any) => v : equivalentTop)(BOTTOMS[idx]);
    const layerIdx = (layer: ZLayer) =>
        isBottom(layer) ? -layer + 5 : layer - 4;

    const HARD_VALID_INPUT = /^-?\d*$/;
    const SOFT_VALID_INPUT = (s: string) => {
        let n = parseInt(s);
        if (isNaN(n)) {
            return false;
        }
        return -50 <= n && n <= 50;
    };

    $: $menuSettings.zOrder = clamp($menuSettings.zOrder, -50, 50);

    // clamp layer count

    $: canSelectByTab = $menuSettings.isMinimized ? -1 : 0;
</script>

<div
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items layers-tab-container md:text-lg sm:text-base"
>
    <ul class="flex flex-col h-full buttons w-min">
        <li
            class="relative flex-1 w-20 h-full md:w-18 xs:w-12 font-pusab flex-center"
        >
            <button
                class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke flex-center"
                on:click={() =>
                    ($menuSettings.zLayer = equivalentBottom(
                        $menuSettings.zLayer
                    ))}
                tabindex={canSelectByTab}
                aria-label="Layer Below Player"
            >
                <Image
                    src="/assets/ui/layer_tab/bottom.svg"
                    class="object-contain max-w-full max-h-full"
                    lazyLoad
                    skeleton
                ></Image>
            </button>
            {#if isBottom($menuSettings.zLayer)}
                <div class="sliding-selector"></div>
            {/if}
        </li>
        <li
            class="relative flex-1 w-20 h-full md:w-16 xs:w-12 font-pusab flex-center"
        >
            <button
                class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke flex-center"
                on:click={() =>
                    ($menuSettings.zLayer = equivalentTop(
                        $menuSettings.zLayer
                    ))}
                tabindex={canSelectByTab}
                aria-label="Layer Above Player"
            >
                <Image
                    src="/assets/ui/layer_tab/top.svg"
                    class="object-contain max-w-full max-h-full"
                    lazyLoad
                    skeleton
                ></Image>
            </button>
            {#if !isBottom($menuSettings.zLayer)}
                <div class="sliding-selector"></div>
            {/if}
        </li>
    </ul>

    <div class="flex flex-col w-full h-full gap-2 content">
        <ul class="flex h-full ids">
            {#each Array(layerCount(isBottom($menuSettings.zLayer))).fill(0) as _, i}
                <li
                    class="relative flex-1 w-full h-full flex-center font-pusab"
                >
                    <button
                        class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                        on:click={() =>
                            ($menuSettings.zLayer = layerFrom(
                                isBottom($menuSettings.zLayer),
                                i
                            ))}
                        tabindex={canSelectByTab}
                        aria-label={`${isBottom($menuSettings.zLayer) ? "B" : "T"}${i + 1}`}
                    >
                        <h1 class="font-pusab lg:text-2xl">{i + 1}</h1>

                        <h2 class="opacity-50 font-pusab xs:text-sm">
                            {`${isBottom($menuSettings.zLayer) ? "B" : "T"}${i + 1}`}
                        </h2>
                    </button>
                    {#if layerIdx($menuSettings.zLayer) == i + 1}
                        <div class="sliding-selector"></div>
                    {/if}
                </li>
            {/each}
        </ul>

        <div class="flex flex-col items-center justify-center zindex g-8">
            <div class="flex items-center justify-center">
                <button
                    class="h-full flex-center"
                    on:click={() => {
                        $menuSettings.zOrder -= 1;
                    }}
                    tabindex={canSelectByTab}
                    aria-label="Decrease Z-Index"
                >
                    <Image
                        src="/assets/ui/edit/move_small.svg"
                        class="rotate-90"
                    />
                </button>

                <Input
                    class="w-20 p-2 text-3xl text-center rounded-lg outline-none md:w-16 xs:w-14 md:text-2xl sm:text-xl xs:text-lg font-pusab text-stroke bg-black/40"
                    maxLength={3}
                    tabIndex={canSelectByTab}
                    hardValidInput={HARD_VALID_INPUT}
                    softValidInput={SOFT_VALID_INPUT}
                    aria-label="Scale input"
                    bind:value={$menuSettings.zOrder}
                />
                <button
                    class="h-full flex-center"
                    on:click={() => {
                        $menuSettings.zOrder += 1;
                    }}
                    tabindex={canSelectByTab}
                    aria-label="Increase Z-Index"
                >
                    <Image
                        src="/assets/ui/edit/move_small.svg"
                        class="-rotate-90"
                    />
                </button>
            </div>
            <h1 class="text-2xl md:text-xl xs:text-lg font-pusab text-stroke">
                Z Index
            </h1>
        </div>
    </div>
</div>

<style>
    .layers-tab-container {
        display: grid;
        grid-auto-columns: 1fr;
        grid-auto-rows: 1fr;
        grid-template-columns: min-content 1fr;
        grid-template-areas:
            "buttons content"
            "buttons content";
    }

    .buttons {
        grid-area: buttons;
    }

    .content {
        grid-area: content;
    }
    /* 
    .zindex {
        grid-area: zindex;
    } */
</style>
