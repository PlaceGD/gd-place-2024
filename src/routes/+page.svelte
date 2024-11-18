<script lang="ts">
    import { onMount } from "svelte";
    import Editor from "../Editor.svelte";

    import {
        wasmProgress,
        initWasm,
        fetchAndParseSpritesheet,
        spritesheetProgress,
    } from "../load_wasm";
    import { rawSpritesheetData } from "../stores";

    onMount(() => {
        initWasm();
        fetchAndParseSpritesheet().then(data => {
            $rawSpritesheetData = data;
        });
    });

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;
</script>

<Editor bind:wasmLoaded={loaded} />

<style>
</style>
