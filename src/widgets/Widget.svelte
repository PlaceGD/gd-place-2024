<script lang="ts">
    import Scale from "./Scale.svelte";
    import { withState, addCallback } from "../state";
    import { onDestroy } from "svelte";
    import Rotate from "./Rotate.svelte";
    import Warp from "./Warp.svelte";
    import { menuSettings } from "../stores";
    import { Widget } from "../place_menu/edit/edit_tab";

    let widgetPos = [0, 0];
    let widgetScale = 1;
    let isVisible = false;

    let cb = addCallback(state => {
        let obj = state.get_preview_object();
        widgetPos = [...state.get_screen_pos(obj.x, obj.y)];
        widgetScale = 1 + state.get_zoom() / 80;
        isVisible = state.is_preview_visible();
    });

    onDestroy(() => cb.remove());
</script>

<div
    class="absolute w-full h-full overflow-visible pointer-events-none flex-center"
    style={`
        left: ${widgetPos[0]}px;
        top: ${-widgetPos[1]}px;
        transform: scale(${widgetScale});
    `}
>
    {#if isVisible}
        {#if $menuSettings.selectedWidget == Widget.Rotate}
            <Rotate />
        {/if}
        {#if $menuSettings.selectedWidget == Widget.Scale}
            <Scale />
        {/if}
        {#if $menuSettings.selectedWidget == Widget.Warp}
            <Warp {widgetScale} />
        {/if}
    {/if}
</div>
