<script lang="ts">
    import * as wasm from "wasm-lib";

    export let state: wasm.StateWrapper;

    import { addCallback } from "../state";

    import Scale from "./Scale.svelte";

    let widgetPos = [0, 0];
    let widgetScale = 1;

    let isVisible = false;

    addCallback(state => {
        isVisible = state.is_preview_visible();
        if (isVisible) {
            const obj = state.get_preview_object();
            widgetPos = [...state.get_screen_pos(obj.x, obj.y)];
            widgetScale = state.get_zoom_scale() / 2;
        }
    });
</script>

<div
    class="absolute overflow-visible w-full h-full flex-center pointer-events-none"
    style={`
        left: ${widgetPos[0]}px;
        top: ${-widgetPos[1]}px;
        transform: scale(${widgetScale});
    `}
>
    {#if isVisible}
        <Scale bind:state></Scale>
    {/if}
</div>
