<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import PlaceMenu from "./place_menu/PlaceMenu.svelte";

    import * as wasm from "../wasm-lib/pkg/wasm_lib";
    import ViewControls from "./level_view/ViewControls.svelte";

    let state: wasm.StateWrapper | null = null;
    let canvas: HTMLCanvasElement;

    $: {
        console.log(state);
    }
</script>

<div class="h-full w-full absolute">
    <LevelView bind:state bind:canvas />
    {#if state != null}
        <ViewControls bind:state bind:canvas />

        <PlaceMenu />
    {/if}
</div>
