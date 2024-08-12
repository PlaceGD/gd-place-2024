<script lang="ts">
    import Input from "../components/Input.svelte";
    import { onDestroy } from "svelte";
    import { clamp, getCenterPos, snap } from "shared-lib/util";
    import * as wasm from "wasm-lib";
    import { isValidObject, objects } from "shared-lib/gd";
    import { setCheckedPreviewObject } from "../utils/misc";

    export let state: wasm.State;

    let rotating: number | null = null;

    const getMouseAngle = (e: MouseEvent) => {
        let [cX, cY] = getCenterPos(circleElement);
        return (-Math.atan2(e.clientY - cY, e.clientX - cX) * 180) / Math.PI;
    };

    let angle = 0;
    // let newAngle = 0;

    const loopFn = () => {
        let obj = state.get_preview_object();

        angle = obj.x_angle;

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => cancelAnimationFrame(loop));

    let circleElement: HTMLDivElement;
</script>

<svelte:window
    on:mouseup={() => {
        rotating = null;
    }}
    on:mousemove={e => {
        if (rotating != null) {
            let snapDegrees =
                objects[state.get_preview_object().id].hitboxType == "Solid"
                    ? 90
                    : 5;
            let newAngle = snap(getMouseAngle(e) - rotating, snapDegrees) / 5;

            let obj = state.get_preview_object();
            let xAngle = obj.x_angle;

            obj.rotate(newAngle - xAngle);
            if (setCheckedPreviewObject(state, obj)) {
                angle = newAngle;
            }
        }
    }}
/>

<div class="absolute text-white">
    <div
        class="border-2 border-dashed rounded-full abs-centered-rel w-72 h-72 border-white/50"
        bind:this={circleElement}
    />
    <button
        class="the_dragger bg-center bg-no-repeat abs-centered-rel w-16 h-16 {rotating !=
        null
            ? 'bg-button-cyan'
            : 'bg-button-green'} rounded-full pointer-events-all cursor-pointer"
        style={`
            transform: translate(${(Math.cos((-angle * 5 * Math.PI) / 180) * 284) / 2 - 32}px, ${(Math.sin((-angle * 5 * Math.PI) / 180) * 284) / 2 - 32}px)
        `}
        on:mousedown={e => {
            rotating = getMouseAngle(e) - angle * 5;
        }}
    />
</div>

<style>
    .the_dragger {
        background-image: url("/assets/ui/edit/dot.svg");
        background-size: 70px;
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
    }
</style>
