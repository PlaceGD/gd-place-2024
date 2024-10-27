<script lang="ts">
    import { onMount } from "svelte";
    import Editor from "../Editor.svelte";
    import { alertHasDarkReader } from "../utils/document";
    import DataPopup from "../DataPopup.svelte";

    import {
        wasmProgress,
        initWasm,
        fetchAndParseSpritesheet,
        spritesheetProgress,
    } from "../load_wasm";
    import ToastContainers from "../components/ToastContainers.svelte";
    import { rawSpritesheetData } from "../stores";
    // import JetpackAnim from "./JetpackAnim.svelte";
    import jetpackAnimText from "./assets/jetpack_anim.svg?raw";
    import ColoredName from "../components/ColoredName.svelte";
    import loadingBgImageUrl from "./assets/bg.png?url";

    import { fade } from "svelte/transition";
    import { tweened } from "svelte/motion";
    import { cubicIn, linear, sineIn } from "svelte/easing";
    import { map } from "shared-lib/util";
    import Guide from "../guide/Guide.svelte";
    import { beginGuide } from "../guide/guide";
    import { DEBUG } from "../utils/debug";

    let openTrans = tweened(
        0,
        // "bgColor",
        { duration: 500, easing: cubicIn }
    );

    onMount(() => {
        alertHasDarkReader();

        initWasm();
        fetchAndParseSpritesheet().then(data => {
            $rawSpritesheetData = data;
        });
    });

    const TOTAL_OPS = 2.2;

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;

    $: if (loaded) {
        setTimeout(() => {
            $openTrans = 1;
        }, 500);
    }

    $: progress =
        $wasmProgress.progress +
        $spritesheetProgress.progress +
        ($rawSpritesheetData != null ? 0.2 : 0.0);

    let viewSize = [1000, 1000];
</script>

<svelte:window
    on:keyup={e => {
        if (e.ctrlKey && e.shiftKey && e.key === "F") {
            $DEBUG = !$DEBUG;
        }
    }}
/>

<!-- <button class="text-white absolute z-[53]" on:click={beginGuide}>TEST</button> -->

<ToastContainers />
<DataPopup />

<div
    class="relative w-screen h-screen overflow-hidden"
    bind:offsetWidth={viewSize[0]}
    bind:offsetHeight={viewSize[1]}
>
    {#if $openTrans < 1}
        <div
            class="absolute z-50 flex flex-col w-full h-full gap-8 p-4 pb-20 flex-center xs:p-2"
        >
            <div
                class="infinite-scroll"
                style={`
                    --bg: url(${loadingBgImageUrl});
                    --bg-pos-x: -${Math.max(...viewSize)}px;
                    opacity: ${1 - $openTrans};
                `}
            ></div>
            <div
                class="relative w-60 h-60 sm:h-56 sm:w-56 xs:h-48 xs:w-48"
                style={`
                    transform: translateY(${-$openTrans * 1 * viewSize[1]}px);
                    opacity: ${map($openTrans, 0, 0.3, 1, 0)};
                `}
            >
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html jetpackAnimText}
            </div>
            <div
                class="relative min-h-[20px] md:min-h-[18px] xs:min-h-[15px] w-2/3 max-w-[850px] rounded-full mt-16 xs:mt-12 overflow-hidden"
                style={`
                    box-shadow: 0 0 0 3px black, 0 0 0 9px #FED83E, 0 0 0 12px black, 0 0 50px 16px #0006;
                    transform: translateY(${$openTrans * 1 * viewSize[1]}px);
                    opacity: ${map($openTrans, 0, 0.3, 1, 0)};
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
            <div
                class="flex flex-col gap-2 flex-center"
                style={`
                    transform: translateY(${$openTrans * 1 * viewSize[1]}px);
                    opacity: ${map($openTrans, 0, 0.3, 1, 0)};
                `}
            >
                <div
                    class="relative text-6xl md:text-5xl sm:text-4xl xs:text-3xl font-pusab"
                >
                    <ColoredName
                        username="Loading"
                        colorOverride="linear-gradient(180deg, #fea20d 20%, #fee348 80%)"
                    ></ColoredName>
                </div>
            </div>
            <div
                class="absolute w-full text-xl text-center text-white bottom-4 xs:text-base font-pusab text-stroke"
                style={`
                    transform: translateY(${$openTrans * 100}px);
                    opacity: ${map($openTrans, 0, 0.3, 1, 0)};
                `}
            >
                Created by Flow, Spu7Nix, DreamingInsanity ❤
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
        background-color: #00368a;
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
            background-position: var(--bg-pos-x) 50%;
        }
    }
</style>
