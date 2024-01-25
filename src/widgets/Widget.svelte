<script lang="ts">
    import Scale from "./Scale.svelte";
    import { withState, addCallback } from "../state";
    import { onDestroy } from "svelte";
    import Rotate from "./Rotate.svelte";
    import Warp from "./Warp.svelte";

    let widgetPos = [0, 0];
    let widgetScale = 1;
    let isVisible = false;

    let cb = addCallback(state => {
        let obj = state.get_preview_object();
        widgetPos = [...state.get_screen_pos(obj.x, obj.y)];
        widgetScale = 1 + state.get_zoom() / 80;
        // console.log(widgetScale);
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
        <!-- <Scale isXY={false} /> -->
        <Warp {widgetScale} />
    {/if}
</div>
