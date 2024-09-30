<script lang="ts">
    import { onMount } from "svelte";
    import { ZLayer } from "wasm-lib";

    import Image from "../../components/Image.svelte";
    import DarkInput from "../../components/DarkInput.svelte";

    import { menuMinimized, menuZLayer, menuZOrder } from "../../stores";
    import { clamp } from "shared-lib/util";

    import bottomIconUrl from "../assets/layer_tab/bottom.svg?url";
    import topIconUrl from "../assets/layer_tab/top.svg?url";

    import moveSmallIconUrl from "../assets/edit_tab/move_small.svg?url";
    import { notNaNAnd } from "../../utils/misc";

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
    const BOTTOMS_ORDERED = [
        ZLayer.B5,
        ZLayer.B4,
        ZLayer.B3,
        ZLayer.B2,
        ZLayer.B1,
    ];
    const TOPS_ORDERED = [ZLayer.T1, ZLayer.T2, ZLayer.T3, ZLayer.T4];
    const isBottom = (layer: ZLayer) => BOTTOMS_ORDERED.includes(layer);
    const layerCount = (bottom: boolean) => (bottom ? 5 : 4);

    const HARD_VALID_INPUT = /^-?\d*$/;
    const SOFT_VALID_INPUT = (s: string) => {
        return notNaNAnd(s, n => -50 <= n && n <= 50);
    };

    $: $menuZOrder = clamp($menuZOrder, -50, 50);
</script>

<fieldset
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items layers-tab-container md:text-lg sm:text-base"
    disabled={$menuMinimized}
>
    <ul class="flex flex-col h-full buttons w-min">
        <li
            class="relative flex-1 w-20 h-full md:w-18 xs:w-12 font-pusab flex-center"
        >
            <button
                class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke flex-center"
                on:click={() => ($menuZLayer = equivalentBottom($menuZLayer))}
                aria-label="Layer Below Player"
                title="Layer Below Player"
            >
                <Image
                    src={bottomIconUrl}
                    class="object-contain max-w-full max-h-full"
                    lazyLoad
                    skeleton
                ></Image>
            </button>
            {#if isBottom($menuZLayer)}
                <div class="sliding-selector"></div>
            {/if}
        </li>
        <li
            class="relative flex-1 w-20 h-full md:w-18 xs:w-12 font-pusab flex-center"
        >
            <button
                class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke flex-center"
                on:click={() => ($menuZLayer = equivalentTop($menuZLayer))}
                aria-label="Layer Above Player"
                title="Layer Above Player"
            >
                <Image
                    src={topIconUrl}
                    class="object-contain max-w-full max-h-full"
                    lazyLoad
                    skeleton
                ></Image>
            </button>
            {#if !isBottom($menuZLayer)}
                <div class="sliding-selector"></div>
            {/if}
        </li>
    </ul>

    <div class="flex flex-col w-full h-full gap-2 content">
        <ul class="flex h-full ids">
            {#each Array(layerCount(isBottom($menuZLayer))).fill(0) as _, idx}
                {@const layer = (
                    isBottom($menuZLayer) ? BOTTOMS_ORDERED : TOPS_ORDERED
                )[idx]}

                <li
                    class="relative flex-1 w-full h-full flex-center font-pusab"
                >
                    <button
                        class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                        on:click={() => {
                            $menuZLayer = layer;
                        }}
                        aria-label={layerName(layer)}
                    >
                        <h2 class="font-pusab lg:text-2xl xs:text-sm">
                            {layerName(layer)}
                        </h2>
                    </button>
                    {#if $menuZLayer == layer}
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
                        $menuZOrder -= 1;
                    }}
                    aria-label="Decrease Z-Index"
                >
                    <Image src={moveSmallIconUrl} class="rotate-90" />
                </button>

                <DarkInput
                    class="w-20 text-2xl font-pusab sm:text-xl xs:text-base"
                    maxLength={3}
                    hardValidInput={HARD_VALID_INPUT}
                    softValidInput={SOFT_VALID_INPUT}
                    aria-label="Scale input"
                    bind:value={$menuZOrder}
                />
                <button
                    class="h-full flex-center"
                    on:click={() => {
                        $menuZOrder += 1;
                    }}
                    aria-label="Increase Z-Index"
                >
                    <Image src={moveSmallIconUrl} class="-rotate-90" />
                </button>
            </div>
            <h1 class="text-2xl md:text-xl xs:text-lg font-pusab text-stroke">
                Z Index
            </h1>
        </div>
    </div>
</fieldset>

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
</style>
