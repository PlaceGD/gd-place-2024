<script lang="ts">
    import Scale from "./Scale.svelte";
    import { withState, addCallback } from "../state";
    import { onDestroy } from "svelte";
    import Rotate from "./Rotate.svelte";
    import Warp from "./Warp.svelte";
    import { menuSettings } from "../stores";
    import { WidgetType } from "../place_menu/edit/edit_tab";

    export let position: [number, number];
    export let scale: number;
</script>

<div
    class="absolute w-full h-full overflow-visible pointer-events-none flex-center"
    style={`
        left: ${position[0]}px;
        top: ${-position[1]}px;
        transform: scale(${scale});
    `}
>
    <slot />
</div>
