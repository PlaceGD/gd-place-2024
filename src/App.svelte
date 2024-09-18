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
    import ToastContainer from "./components/ToastContainer.svelte";
    import { rawSpritesheetData } from "./stores";

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
</script>

<!-- <Turnstiles /> -->

<ToastContainer />

<DataPopup />

<div class="relative w-screen h-screen overflow-hidden">
    {#if !loaded}
        <div class="absolute">
            <input
                type="range"
                min={0}
                {max}
                value={progress}
                aria-label="Download progress of data required for GD Place"
                aria-valuetext={`${(progress / max) * 100}%`}
                tabindex="-1"
            />
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
</style>
