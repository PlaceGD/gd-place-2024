<script lang="ts">
    import Editor from "./Editor.svelte";
    import { alertHasDarkReader } from "./utils/document";
    import DataPopup from "./DataPopup.svelte";

    import {
        wasmProgress,
        initWasm,
        fetchAndParseSpritesheet,
        spritesheetProgress,
    } from "./load_wasm";
    import ToastContainers from "./components/ToastContainers.svelte";
    import { rawSpritesheetData } from "./stores";
    // import JetpackAnim from "./JetpackAnim.svelte";
    import jetpackAnimText from "./jetpack_anim.svg?raw";
    import ColoredName from "./components/ColoredName.svelte";
    import loadingBgImageUrl from "./bg.png?url";

    alertHasDarkReader();

    initWasm();
    fetchAndParseSpritesheet().then(data => {
        $rawSpritesheetData = data;
    });

    const TOTAL_OPS = 2.5;

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;

    $: progress = $wasmProgress.progress + $spritesheetProgress.progress;

    let bgContainerSize: [number, number] = [0, 0];
</script>

<ToastContainers />
<DataPopup />

<div class="relative w-screen h-screen overflow-hidden">
    {#if !loaded}
        <div
            class="relative flex flex-col w-full h-full gap-8 p-4 flex-center xs:p-2 bg-[#00368a]"
        >
            <div
                class="absolute flex w-full h-full bg-transparent silly-background flex-center"
                style={`
                transform: scale(${Math.max(...bgContainerSize) / 1024});
            `}
                bind:offsetWidth={bgContainerSize[0]}
                bind:offsetHeight={bgContainerSize[1]}
            >
                {#each [0, 1, 2] as _}
                    <img
                        src={loadingBgImageUrl}
                        alt="Default Geometry Dash Background"
                        class="min-w-[1024px] min-h-[1024px] silly-bg-image"
                        draggable="false"
                        style:scale="1.001"
                    />
                {/each}
            </div>
            <div class="relative w-60 h-60 sm:h-56 sm:w-56 xs:h-48 xs:w-48">
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html jetpackAnimText}
            </div>
            <div
                class="relative h-[20px] md:h-[18px] xs:h-[15px] w-1/2 max-w-[550px] rounded-full mt-16 xs:mt-12 overflow-hidden"
                style={`box-shadow: 0 0 0 3px black, 0 0 0 9px white, 0 0 0 12px black, 0 0 50px 16px #0006;`}
            >
                <div
                    class="h-full bg-[#01B2FF]"
                    style={`
                    width: ${(progress / TOTAL_OPS) * 100}%;
                    background-image: repeating-linear-gradient(-67deg, #0003 0px, #0003 30px, #0000 30px, #0000 90px, #0003 90px, #0003 120px);
                    background-size: 130px 100px;
                    background-position: 0% 100%;
                `}
                ></div>
            </div>
            <div class="flex flex-col gap-2 flex-center">
                <div
                    class="relative text-6xl md:text-5xl sm:text-4xl xs:text-3xl font-pusab"
                >
                    <ColoredName
                        username="Loading"
                        colorOverride="linear-gradient(180deg, #fea20d 20%, #fee348 80%)"
                    ></ColoredName>
                </div>
                <div
                    class="relative text-xl text-center text-white xs:text-base font-pusab text-stroke"
                >
                    Created with ❤ by Flow, Spu7Nix, DreamingInsanity
                </div>
            </div>
        </div>
    {/if}
    <Editor bind:wasmLoaded={loaded} />
</div>

<style>
    .silly-bg-image {
        animation: moob 20s linear infinite;
    }

    @keyframes moob {
        from {
            transform: translateX(0);
        }
        to {
            transform: translateX(-1024px);
        }
    }
</style>
