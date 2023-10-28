<script lang="ts">
    import * as wasm from "../../wasm-lib/pkg/wasm_lib";
    import { clamp, hexToRgb, lerp } from "../util";

    export let state: wasm.StateWrapper;
    export let canvas: HTMLCanvasElement;

    let dragging: null | {
        prev_mouse_x: number;
        prev_mouse_y: number;
        prev_camera_x: number;
        prev_camera_y: number;
    } = null;

    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";

    let [mouseX, mouseY] = [0, 0];

    let zoomGoal = state.get_zoom();
    const zoomTween = tweened(0, {
        duration: 100,
        easing: cubicOut,
    });

    let changeZoom = (z: number) => {
        let [mx, my] = state.get_world_pos(
            mouseX - canvas.offsetWidth / 2,
            -(mouseY - canvas.offsetHeight / 2)
        );
        let [cx, cy] = state.get_camera_pos();
        let prev_zoom_scale = state.get_zoom_scale();

        state.set_zoom(z);

        let zoom_scale_change = state.get_zoom_scale() / prev_zoom_scale;

        state.set_camera_pos(
            lerp(mx, cx, 1 / zoom_scale_change),
            lerp(my, cy, 1 / zoom_scale_change)
        );
    };

    $: {
        changeZoom($zoomTween);
    }

    const gubuh = (event: any) => {
        let c = hexToRgb(event.target.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_bg_color(r, g, b, 255);
        }
    };
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
        }
    }}
/>

<div
    class="h-full w-full absolute touch-none"
    on:pointermove={e => {
        mouseX = e.pageX;
        mouseY = e.pageY;
    }}
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
        zoomGoal = clamp(zoomGoal - (e.deltaY / 100) * 2, -36, 36);
        zoomTween.set(zoomGoal);
    }}
/>

<input
    type="color"
    class="absolute"
    on:input={e => {
        gubuh(e);
    }}
/>
