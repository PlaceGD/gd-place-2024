<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import PlaceMenu from "./place_menu/PlaceMenu.svelte";

    import * as wasm from "../wasm-lib/pkg/wasm_lib";
    import ViewControls from "./level_view/ViewControls.svelte";

    let state: wasm.StateWrapper | null = null;
    let canvas: HTMLCanvasElement;

    // FUNCTION TOAST(BEANS) {
    //     console.log("toast");
    // }
</script>

<div class="absolute w-full h-full">
    <LevelView bind:state bind:canvas />
    {#if state != null}
        <ViewControls bind:state bind:canvas />

        <PlaceMenu />
    {/if}
</div>
