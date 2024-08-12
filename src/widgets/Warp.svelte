<script lang="ts">
    import * as wasm from "wasm-lib";

    export let widgetScale: number;
    export let state: wasm.State;

    import { onDestroy, onMount } from "svelte";
    import { clamp, remEuclid, round, snap } from "shared-lib/util";
    import { isValidObject, objects } from "shared-lib/gd";
    import { setCheckedPreviewObject } from "../utils/misc";

    let draggingX: [number, number, number, number] | null = null;
    let draggingY: [number, number, number, number] | null = null;

    let [ix, iy, jx, jy] = [1, 0, 0, 1];

    const settem = (obj: wasm.GDObjectOpt) => {
        [ix, iy, jx, jy] = wasm.convert_opt_transform(
            obj.x_scale_exp,
            obj.x_angle,
            obj.y_scale_exp,
            obj.y_angle
        );
    };

    const loopFn = () => {
        let obj = state.get_preview_object();

        settem(obj);

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => cancelAnimationFrame(loop));

    let xElement: HTMLElement;
    let yElement: HTMLElement;
</script>

<svelte:window
    on:mouseup={() => {
        draggingX = null;
        draggingY = null;
    }}
    on:mousemove={e => {
        let snapDegrees =
            objects[state.get_preview_object().id].hitboxType == "Solid"
                ? 90
                : 5;

        if (draggingX != null) {
            let [dix, diy, dpix, dpiy] = draggingX;

            let obj = state.get_preview_object();
            let nx = dpix + (e.clientX - dix) / 100 / widgetScale;
            let ny = dpiy - (e.clientY - diy) / 100 / widgetScale;
            obj.x_scale_exp = clamp(
                Math.round(Math.log2(Math.hypot(nx, ny)) * 12),
                -12,
                12
            );
            obj.x_angle =
                snap((Math.atan2(ny, nx) * 180) / Math.PI, snapDegrees) / 5;
            if (setCheckedPreviewObject(state, obj)) {
                settem(obj);
            }
        }
        if (draggingY != null) {
            let [djx, djy, dpjx, dpjy] = draggingY;

            let obj = state.get_preview_object();
            let nx = dpjx + (e.clientX - djx) / 100 / widgetScale;
            let ny = dpjy - (e.clientY - djy) / 100 / widgetScale;
            obj.y_scale_exp = clamp(
                Math.round(Math.log2(Math.hypot(nx, ny)) * 12),
                -12,
                12
            );
            obj.y_angle =
                snap((Math.atan2(ny, nx) * 180) / Math.PI, snapDegrees) / 5;
            if (setCheckedPreviewObject(state, obj)) {
                settem(obj);
            }
        }
    }}
/>

<div class="absolute text-white">
    <div class="abs-centered-rel w-1 h-1 rounded-full bg-white" />
    <div
        class="absolute w-0 h-0 max-w-0 max-h-0 rounded-full bg-white"
        style={`
            transform: translate(${(ix / 2) * 100}px, ${-(iy / 2) * 100}px) rotate(${-Math.atan2(iy, ix)}rad);
        `}
    >
        <div
            class="abs-centered-rel h-1 rounded-full bg-white"
            style={`
                width: ${Math.hypot(ix, iy) * 100}px;
            `}
        />
    </div>
    <div
        class="absolute w-0 h-0 max-w-0 max-h-0 rounded-full bg-white"
        style={`
            transform: translate(${(jx / 2) * 100}px, ${-(jy / 2) * 100}px) rotate(${-Math.atan2(jy, jx)}rad);
        `}
    >
        <div
            class="abs-centered-rel h-1 rounded-full bg-white"
            style={`
                width: ${Math.hypot(jx, jy) * 100}px;
            `}
        />
    </div>
    <button
        class="the_dragger bg-center bg-no-repeat abs-centered-rel w-8 h-8 {draggingX !=
        null
            ? 'bg-button-cyan'
            : 'bg-button-green'} rounded-md pointer-events-all cursor-pointer"
        style={`
            left: ${100 * ix}px;
            top: ${-100 * iy}px;
        `}
        tabindex="-1"
        bind:this={xElement}
        on:mousedown={e => {
            draggingX = [e.clientX, e.clientY, ix, iy];
        }}
    >
        <div
            class="relative flex flex-col items-start gap-5 left-11 top-[-13px]"
            tabindex="-1"
        >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round((Math.atan2(iy, ix) * 180) / Math.PI, 2)}°</span
            >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round(Math.hypot(iy, ix), 3)}</span
            >
        </div>
    </button>
    <button
        class="the_dragger bg-center bg-no-repeat abs-centered-rel w-8 h-8 {draggingY !=
        null
            ? 'bg-button-cyan'
            : 'bg-button-green'} rounded-md pointer-events-all cursor-pointer"
        style={`
        left: ${100 * jx}px;
        top: ${-100 * jy}px;
    `}
        tabindex="-1"
        bind:this={yElement}
        on:mousedown={e => {
            draggingY = [e.clientX, e.clientY, jx, jy];
        }}
    >
        <div
            class="relative flex flex-col items-start gap-5 left-11 top-[-13px]"
            tabindex="-1"
        >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round((Math.atan2(jy, jx) * 180) / Math.PI, 2)}°</span
            >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round(Math.hypot(jy, jx), 3)}</span
            >
        </div>
    </button>
</div>

<style>
    .the_dragger {
        /* background-image: url("/assets/ui/edit/dot.svg"); */
        /* background-size: 30px; */
        box-shadow:
            0px 0px 0px 3px #fff,
            0px 2px 8px 6px #0004,
            inset 0px 0px 0px 3px #000;
    }
</style>
