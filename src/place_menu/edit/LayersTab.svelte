<script lang="ts">
    import { onMount } from "svelte";
    import { ZLayer } from "wasm-lib";
    import { AnimateSharedLayout } from "svelte-motion";

    import Image from "../../components/Image.svelte";
    import SlidingSelector from "../../components/SlidingSelector.svelte";

    import { menuSettings } from "../../stores";
    import { LayerType } from "./edit_tab";

    const LAYER_NAME: any = {
        [LayerType.T]: "T",
        [LayerType.B]: "B",
    };

    const LAYER_COUNT: any = {
        [LayerType.T]: 4,
        [LayerType.B]: 5,
    };

    const selectedLayerToZLayer = (layer: LayerType, id: number): ZLayer => {
        if (layer == LayerType.T) {
            return [ZLayer.T1, ZLayer.T2, ZLayer.T3][id - 1];
        } else {
            return [ZLayer.B4, ZLayer.B3, ZLayer.B2, ZLayer.B1][id - 1];
        }
    };

    // clamp layer count
    $: {
        if (
            $menuSettings.layerType == LayerType.T &&
            $menuSettings.layerIdx == LAYER_COUNT[LayerType.B] - 1
        ) {
            $menuSettings.layerIdx = LAYER_COUNT[LayerType.T] - 1;
        }

        $menuSettings.zLayer = selectedLayerToZLayer(
            $menuSettings.layerType,
            $menuSettings.layerIdx
        );
    }

    const toValidInt = (s: string) => {
        if (s == "" || s == "-") {
            return 0;
        }
        return parseInt(s);
    };

    const re = /^(-?)\d*$/;

    const validLayerInput = (s: string) => {
        if (!re.test(s)) {
            return false;
        }
        let n = toValidInt(s);
        if (n < -50 || n > 50) {
            return false;
        }
        return true;
    };

    const enterIfValid = (e: any) => {
        if (!validLayerInput(e.currentTarget.value)) {
            e.currentTarget.value = prevValidInputData;
        } else {
            prevValidInputData = e.currentTarget.value;
        }
    };

    let prevValidInputData: string = $menuSettings.zOrder.toString();
    onMount(() => {
        inputElement.value = prevValidInputData;
    });
    $: {
        $menuSettings.zOrder = toValidInt(prevValidInputData);
    }
    let inputElement: HTMLInputElement;
</script>

<div
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items layers-tab-container md:text-lg sm:text-md"
>
    <ul class="flex flex-col h-full buttons w-min">
        <AnimateSharedLayout>
            <li
                class="relative flex-1 w-20 h-full md:w-18 xs:w-12 font-pusab flex-center"
            >
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke flex-center"
                    on:click={() => ($menuSettings.layerType = LayerType.B)}
                >
                    <Image
                        src="/assets/ui/layer_tab/bottom.svg"
                        class="object-contain w-full max-w-full max-h-full"
                        lazyLoad
                        skeleton
                    ></Image>
                </button>
                {#if $menuSettings.layerType == LayerType.B}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
            <li
                class="relative flex-1 w-20 h-full md:w-16 xs:w-12 font-pusab flex-center"
            >
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke flex-center"
                    on:click={() => ($menuSettings.layerType = LayerType.T)}
                >
                    <Image
                        src="/assets/ui/layer_tab/top.svg"
                        class="object-contain w-full max-w-full max-h-full"
                        lazyLoad
                        skeleton
                    ></Image>
                </button>
                {#if $menuSettings.layerType == LayerType.T}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
        </AnimateSharedLayout>
    </ul>
    <ul class="flex h-full ids">
        <AnimateSharedLayout>
            {#each Array(LAYER_COUNT[$menuSettings.layerType]).fill(0) as _, i}
                <li
                    class="relative flex-1 w-full h-full flex-center font-pusab"
                >
                    <button
                        class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                        on:click={() => ($menuSettings.layerIdx = i)}
                    >
                        <h1 class="font-pusab">{i + 1}</h1>

                        <h2 class="opacity-50 font-pusab xs:text-sm">
                            ({LAYER_NAME[$menuSettings.layerType]}{i + 1})
                        </h2>
                    </button>
                    {#if $menuSettings.layerIdx == i}
                        <SlidingSelector layoutId="button-selector"
                        ></SlidingSelector>
                    {/if}
                </li>
            {/each}
        </AnimateSharedLayout>
    </ul>

    <div class="flex flex-col items-center justify-center pt-8 zindex g-8">
        <div class="flex items-center justify-center">
            <button
                class="h-full flex-center"
                on:click={() => {
                    if ($menuSettings.zOrder > -50) {
                        $menuSettings.zOrder -= 1;
                        prevValidInputData = $menuSettings.zOrder.toString();
                        inputElement.value = prevValidInputData;
                    }
                }}
            >
                <Image src="/assets/ui/edit/move_small.svg" class="rotate-90" />
            </button>
            <input
                type="text"
                class="w-20 p-2 text-3xl text-center rounded-lg outline-none md:w-16 xs:w-14 md:text-2xl sm:text-xl xs:text-lg font-pusab text-stroke bg-black/40"
                max={100}
                maxlength={4}
                on:input={enterIfValid}
                bind:this={inputElement}
            />
            <button
                class="h-full flex-center"
                on:click={() => {
                    if ($menuSettings.zOrder < 50) {
                        $menuSettings.zOrder += 1;
                        prevValidInputData = $menuSettings.zOrder.toString();
                        inputElement.value = prevValidInputData;
                    }
                }}
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

<style>
    .layers-tab-container {
        display: grid;
        grid-auto-columns: 1fr;
        grid-auto-rows: 1fr;
        grid-template-columns: min-content 1fr;
        grid-template-areas:
            "buttons ids"
            "buttons zindex";
    }

    .buttons {
        grid-area: buttons;
    }

    .ids {
        grid-area: ids;
    }

    .zindex {
        grid-area: zindex;
    }
</style>
