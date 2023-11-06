<script lang="ts">
    import { default as cx } from "classnames";
    import { ZLayer } from "../../wasm-lib/pkg/wasm_lib";
    import Image from "../components/Image.svelte";

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
</script>

<div class="w-full h-full gap-12 p-4 flex-center">
    <div class="relative flex justify-end w-16 h-16">
        <Image
            src="/assets/default_icon.png"
            class={cx({
                "absolute object-contain w-auto h-auto aspect-square max-h-auto max-w-auto translate-x-1/4 translate-y-1/4": true,
                "z-20": Object.values(LAYERS.b).indexOf(selectedLayer) > -1,
            })}
        />
        <Image
            src="/textures/main/83.png"
            class="absolute object-contain w-auto h-auto aspect-square max-h-auto max-w-auto -translate-x-1/4 -translate-y-1/4"
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
        <button>
            <Image src="/assets/ui/edit/up_small.png" />
        </button>
        <h1 class="text-3xl font-pusab text-stroke">100</h1>
        <button>
            <Image src="/assets/ui/edit/down_small.png" />
        </button>
    </div>
</div>
