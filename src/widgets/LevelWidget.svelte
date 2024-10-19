<script lang="ts">
    import { onDestroy } from "svelte";
    import * as wasm from "wasm-lib";

    export let state: wasm.State;

    export let x: number = 0;
    export let y: number = 0;
    export let scale: number = 1;
    export let scaleWithZoom: boolean = true;

    let zoomScale = 0;
    let pos = [0, 0];

    const getScreenPosZoomCorrected = (
        x: number,
        y: number
    ): [number, number] =>
        [
            ...state.get_screen_pos(x, y).map(v => v / window.devicePixelRatio),
        ] as any;

    const loopFn = () => {
        zoomScale = state.get_zoom_scale();
        pos = getScreenPosZoomCorrected(x, y);

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    // console.log("v: ", pos);

    onDestroy(() => cancelAnimationFrame(loop));
</script>

<div
    class="absolute w-full h-full overflow-visible pointer-events-none flex-center"
    style={`
        left: ${pos[0]}px;
        top: ${-pos[1]}px;
        transform: scale(${scale * (scaleWithZoom ? zoomScale : 1)});
    `}
>
    <slot />
</div>
