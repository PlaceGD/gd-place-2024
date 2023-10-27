<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import PlaceMenu from "./place_menu/PlaceMenu.svelte";

    import * as wasm from "../wasm-lib/pkg/wasm_lib";
    import ViewControls from "./level_view/ViewControls.svelte";

    let state: wasm.StateWrapper | null = null;

    $: {
        console.log(state);
    }
</script>

<div class="h-full w-full absolute">
    <LevelView bind:state />
    {#if state != null}
        <ViewControls bind:state />

        <PlaceMenu />
    {/if}
</div>
