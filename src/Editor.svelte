<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import PlaceMenu from "./place_menu/PlaceMenu.svelte";

    import * as wasm from "wasm-lib";
    import ViewControls from "./level_view/ViewControls.svelte";

    export let wasmLoaded: boolean;

    let state: wasm.StateWrapper | null = null;
    let canvas: HTMLCanvasElement;
</script>

<div class="absolute w-full h-full">
    {#if wasmLoaded}
        <LevelView bind:state bind:canvas />
    {/if}
    {#if state != null}
        <ViewControls bind:state bind:canvas />
    {/if}
    <PlaceMenu bind:state />
</div>
