<script lang="ts">
    import { default as cx } from "classnames";
    import { ZLayer } from "wasm-lib";
    import Image from "../components/Image.svelte";
    import { onMount } from "svelte";
    import SlidingSelector from "../components/SlidingSelector.svelte";
    import { AnimateSharedLayout } from "svelte-motion";

    enum Layer {
        T,
        B,
    }

    const LAYER_COUNT = {
        [Layer.T]: 4,
        [Layer.B]: 5,
    };

    const selectedLayerToZLayer = (layer: Layer, id: number): ZLayer => {
        if (layer == Layer.T) {
            return [ZLayer.B4, ZLayer.B3, ZLayer.B2, ZLayer.B1][id - 1];
        } else {
            return [ZLayer.T3, ZLayer.T2, ZLayer.T1][id - 1];
        }
    };

    // const LAYERS = {
    //     b: {
    //         //B5: ZLayer.B5,
    //         B4: ZLayer.B4,
    //         B3: ZLayer.B3,
    //         B2: ZLayer.B2,
    //         B1: ZLayer.B1,
    //     },
    //     t: {
    //         T1: ZLayer.T1,
    //         T2: ZLayer.T2,
    //         T3: ZLayer.T3,
    //     },
    // };

    let selectedLayer: Layer = Layer.B;
    let selectedLayerIdx: number = 0;

    // clamp layer count
    $: {
        if (
            selectedLayer == Layer.T &&
            selectedLayerIdx == LAYER_COUNT[Layer.B] - 1
        ) {
            selectedLayerIdx = LAYER_COUNT[Layer.T] - 1;
        }
    }

    export let selectedZLayer: ZLayer;
    export let zOrder: number;

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

    let prevValidInputData: string = zOrder.toString();
    onMount(() => {
        inputElement.value = prevValidInputData;
    });
    $: {
        zOrder = toValidInt(prevValidInputData);
    }
    let inputElement: HTMLInputElement;
</script>

<!-- "z-20": Object.values(LAYERS.b).indexOf(selectedLayer) > -1, -->
<div
    class="items-center w-full h-full p-4 text-xl md:p-2 gap-x-4 items layers-tab-container md:text-lg sm:text-md"
>
    <ul class="flex flex-col h-full buttons">
        <AnimateSharedLayout>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                    on:click={() => (selectedLayer = Layer.T)}
                    >Above Player</button
                >
                {#if selectedLayer == Layer.T}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
            <li class="relative flex-1 w-full h-full flex-center font-pusab">
                <button
                    class="z-20 w-full h-full p-2 rounded-lg sm:p-1 detail text-stroke"
                    on:click={() => (selectedLayer = Layer.B)}
                    >Below Player</button
                >
                {#if selectedLayer == Layer.B}
                    <SlidingSelector layoutId="button-selector"
                    ></SlidingSelector>
                {/if}
            </li>
        </AnimateSharedLayout>
    </ul>
    <ul class="flex h-full ids">
        <AnimateSharedLayout>
            {#each Array(LAYER_COUNT[selectedLayer]).fill(0) as _, i}
                <li
                    class="relative flex-1 w-full h-full flex-center font-pusab"
                >
                    <button
                        class="z-20 w-full h-full p-2 rounded-lg sm:p-1 main text-stroke"
                        on:click={() => (selectedLayerIdx = i)}>{i + 1}</button
                    >
                    {#if selectedLayerIdx == i}
                        <SlidingSelector layoutId="button-selector"
                        ></SlidingSelector>
                    {/if}
                </li>
            {/each}
        </AnimateSharedLayout>
    </ul>
    <div class="flex flex-col items-center justify-center zindex pt-8 g-8">
        <div class="flex items-center justify-center">
            <button
                class="h-full flex-center"
                on:click={() => {
                    if (zOrder > -50) {
                        zOrder -= 1;
                        prevValidInputData = zOrder.toString();
                        inputElement.value = prevValidInputData;
                    }
                }}
            >
                <Image src="/assets/ui/edit/move_small.svg" class="rotate-90" />
            </button>
            <input
                type="text"
                class="w-20 p-2 text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
                max={100}
                maxlength={4}
                on:input={enterIfValid}
                bind:this={inputElement}
            />
            <button
                class="h-full flex-center"
                on:click={() => {
                    if (zOrder < 50) {
                        zOrder += 1;
                        prevValidInputData = zOrder.toString();
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
        <h1 class="text-2xl font-pusab text-stroke">Z Index (-50 to 50)</h1>
    </div>
    <!-- <div class="w-24 h-24 flex-center">
        <Image
            src={`/assets/ui/layer_tab/${
                Object.values(LAYERS.b).indexOf(selectedLayer) > -1
                    ? "bottom"
                    : "top"
            }.png`}
            class="object-contain w-auto h-auto aspect-square max-h-auto max-w-auto"
        />
    </div>
    <div class="flex flex-col items-center justify-center">
        <ul class="flex">
       
            {#each Object.entries(LAYERS.t) as [name, layer]}
                <li
                    class={cx({
                        "w-16 h-16 flex-center rounded-lg": true,
                        "bg-white/30": layer == selectedLayer,
                    })}
                >
                    <button
                        class="w-full h-full text-2xl md:text-xl font-pusab text-stroke"
                        on:click={() => {
                            selectedLayer = layer;
                        }}>{name}</button
                    >
                </li>
            {/each}
        </ul>
        <ul class="flex">
            {#each Object.entries(LAYERS.b) as [name, layer]}
                <li
                    class={cx({
                        "w-16 h-16 flex-center rounded-lg": true,
                        "bg-white/30": layer == selectedLayer,
                    })}
                >
                    <button
                        class="w-full h-full text-2xl md:text-xl font-pusab text-stroke"
                        on:click={() => {
                            selectedLayer = layer;
                        }}>{name}</button
                    >
                </li>
            {/each}
        </ul>
    </div>
    <div class="flex flex-col items-center justify-center">
        <button
            class="w-full h-full flex-center"
            on:click={() => {
                if (zOrder < 50) {
                    zOrder += 1;
                    prevValidInputData = zOrder.toString();
                    inputElement.value = prevValidInputData;
                }
            }}
        >
            <Image src="/assets/ui/edit/move_small.svg" class="rotate-180" />
        </button>
        <input
            type="text"
            class="w-20 p-2 text-3xl text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
            max={100}
            maxlength={4}
            on:input={enterIfValid}
            bind:this={inputElement}
        />
        <button
            class="w-full h-full flex-center"
            on:click={() => {
                if (zOrder > -50) {
                    zOrder -= 1;
                    prevValidInputData = zOrder.toString();
                    inputElement.value = prevValidInputData;
                }
            }}
        >
            <Image src="/assets/ui/edit/move_small.svg" />
        </button>
    </div> -->
</div>

<!-- <style>
    .player-grid {
        display: grid;
        grid-template-columns: 1fr;
        grid-template-rows: 1fr;
    }
</style> -->
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
