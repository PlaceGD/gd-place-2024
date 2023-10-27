<script lang="ts">
    import * as wasm from "../../wasm-lib/pkg/wasm_lib";

    export let state: wasm.StateWrapper;

    let dragging: null | {
        prev_mouse_x: number;
        prev_mouse_y: number;
        prev_camera_x: number;
        prev_camera_y: number;
    } = null;
</script>

<svelte:window
    on:pointerup={() => {
        dragging = null;
    }}
    on:pointermove={e => {
        if (dragging != null) {
            let z = state.get_zoom_scale();
            state.set_camera_pos(
                (1 / z) * (dragging.prev_mouse_x - e.pageX) +
                    dragging.prev_camera_x,
                (1 / z) * (-dragging.prev_mouse_y + e.pageY) +
                    dragging.prev_camera_y
            );
            // console.log(state.get_camera_x(), state.get_camera_y());
        }
    }}
/>

<div
    class="h-full w-full absolute touch-none"
    on:pointerdown={e => {
        let [x, y] = state.get_camera_pos();
        dragging = {
            prev_camera_x: x,
            prev_camera_y: y,
            prev_mouse_x: e.pageX,
            prev_mouse_y: e.pageY,
        };
    }}
    on:wheel={e => {
        // console.log(e.deltaY);
        state.set_zoom(state.get_zoom() - (e.deltaY / 100) * 6);
        // console.log(state.get_camera_x() * state.get_zoom_scale());
    }}
/>
