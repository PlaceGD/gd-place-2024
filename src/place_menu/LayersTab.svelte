<script lang="ts">
    import { default as cx } from "classnames";
    import { ZLayer } from "../../wasm-lib/pkg/wasm_lib";
    import Image from "../components/Image.svelte";
    import { onMount } from "svelte";

    const LAYERS = {
        b: {
            B4: ZLayer.B4,
            B3: ZLayer.B3,
            B2: ZLayer.B2,
            B1: ZLayer.B1,
        },
        t: {
            T1: ZLayer.T1,
            T2: ZLayer.T2,
            T3: ZLayer.T3,
        },
    };

    export let selectedLayer: ZLayer;
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
<div class="w-full h-full gap-8 p-4 flex-center">
    <div class="w-24 h-24 flex-center">
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
            <!-- Skrunkly🙂 -->
            {#each Object.entries(LAYERS.t) as [name, layer]}
                <li
                    class={cx({
                        "w-16 h-16 flex-center rounded-lg": true,
                        "bg-white/30": layer == selectedLayer,
                    })}
                >
                    <button
                        class="w-full h-full text-2xl font-pusab text-stroke"
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
                        class="w-full h-full text-2xl font-pusab text-stroke"
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
    </div>
</div>

<style>
    .player-grid {
        display: grid;
        grid-template-columns: 1fr;
        grid-template-rows: 1fr;
    }
</style>
