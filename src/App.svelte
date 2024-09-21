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
    // import Announcement from "./Announcement.svelte";

    alertHasDarkReader();

    initWasm();
    fetchAndParseSpritesheet().then(data => {
        $rawSpritesheetData = data;
    });

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;

    $: max = $wasmProgress.max + $spritesheetProgress.max;
    $: progress = $wasmProgress.progress + $spritesheetProgress.progress;

    let gay = 0;
    setInterval(() => {
        gay += 1 / 60;
    }, 1000 / 60);
</script>

<!-- <Turnstiles /> -->

<ToastContainers />
<!-- <Announcement /> -->

<DataPopup />

<div class="relative w-screen h-screen overflow-hidden">
    {#if !loaded}
        <!-- <div class="absolute">
            <input
                type="range"
                min={0}
                {max}
                value={progress}
                aria-label="Download progress of data required for GD Place"
                aria-valuetext={`${(progress / max) * 100}%`}
                tabindex="-1"
            />
        </div> -->

        <div
            class="w-[500px] h-[30px] p-[2px] m-16 rounded-[8px]"
            style={`background: conic-gradient(from ${gay / 3}turn, black 0%, lime 2%, black 10%);`}
        >
            <div class="bg-black w-full h-full rounded-[6px]"></div>
        </div>
    {/if}
    <Editor bind:wasmLoaded={loaded} />
</div>

<style lang="postcss">
    :global(._toastContainer) {
        font-family: Saira;
        color: white;
        border-radius: 8px;
    }

    @media screen(lg) {
        :global(._toastContainer) {
            font-size: 14px;
        }
    }

    @media screen(sm) {
        :global(._toastContainer) {
            font-size: 12px;
        }
    }
    :global(._toastItem) {
        backdrop-filter: blur(10px);
    }
</style>
