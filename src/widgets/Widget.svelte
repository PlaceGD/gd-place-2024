<script lang="ts">
    import Scale from "./Scale.svelte";
    import { withState, addCallback } from "../state";
    import { onDestroy } from "svelte";

    let widgetPos = [0, 0];
    let widgetScale = 1;
    let isVisible = false;

    let cb = addCallback(state => {
        let obj = state.get_preview_object();
        widgetPos = [...state.get_screen_pos(obj.x, obj.y)];
        widgetScale = state.get_zoom_scale() / 2;
        isVisible = state.is_preview_visible();
    });

    onDestroy(() => cb.remove());
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
        <Scale isXY={true} />
    {/if}
</div>
