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

    const TOTAL_OPS = 2.2;

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;

    $: progress = $wasmProgress.progress + $spritesheetProgress.progress;
</script>

<ToastContainers />
<DataPopup />

<div class="relative w-screen h-screen overflow-hidden">
    {#if !loaded}
        <div
            class="relative flex flex-col w-full h-full gap-8 flex-center p-4 xs:p-2 bg-[#00368a]"
        >
            <div
                class="infinite-scroll"
                style={`
                --bg: url(${loadingBgImageUrl});
            `}
            ></div>
            <div class="relative w-60 h-60 sm:h-56 sm:w-56 xs:h-48 xs:w-48">
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html jetpackAnimText}
            </div>
            <div
                class="relative h-[20px] md:h-[18px] xs:h-[15px] w-2/3 max-w-[850px] rounded-full mt-16 xs:mt-12 overflow-hidden"
                style={`
                box-shadow: 0 0 0 3px black, 0 0 0 9px #FED83E, 0 0 0 12px black, 0 0 50px 16px #0006;
                // outline: 3px solid black, 9px solid white;
            `}
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
    .infinite-scroll {
        position: absolute;
        top: 0;
        left: 0;
        height: 100vh;
        width: 100vw;
        background-image: var(--bg);
        background-size: cover;
        background-repeat: repeat-x;
        animation: scroll-background 20s linear infinite;
    }
    @keyframes scroll-background {
        from {
            background-position: 0 50%;
        }
        to {
            background-position: -100vw 50%;
        }
    }
</style>
