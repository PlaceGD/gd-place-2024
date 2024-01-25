<script lang="ts">
    import {
        addCallback as addUpdateCallback,
        setPreviewObject,
        withState,
    } from "../state";
    import * as wasm from "wasm-lib";

    export let widgetScale: number;

    import { onDestroy } from "svelte";
    import { widgetData } from "../stores";
    import { clamp, getCenterPos, round, signedClamp, snap } from "shared-lib";

    let draggingX: [number, number, number, number] | null = null;
    let draggingY: [number, number, number, number] | null = null;

    let ix = 0;
    let iy = 0;
    let jx = 0;
    let jy = 0;

    const settem = (obj: wasm.GDObject) => {
        ix = obj.ix;
        iy = obj.iy;
        jx = obj.jx;
        jy = obj.jy;
    };

    let cb = addUpdateCallback(state => {
        let obj = state.get_preview_object();

        settem(obj);
    });

    onDestroy(() => cb.remove());

    let xElement: HTMLElement;
    let yElement: HTMLElement;
</script>

<svelte:window
    on:mouseup={() => {
        draggingX = null;
        draggingY = null;
    }}
    on:mousemove={e => {
        if (draggingX != null) {
            let [dix, diy, dpix, dpiy] = draggingX;
            withState(state => {
                let obj = state.get_preview_object();
                obj.ix = dpix + (e.clientX - dix) / 100 / widgetScale;
                obj.iy = dpiy - (e.clientY - diy) / 100 / widgetScale;

                obj.set_x_scale(
                    snap(signedClamp(obj.x_basis_len(), 0.5, 2.0), 0.05)
                );
                obj.set_x_angle(snap(obj.x_basis_angle(), 5));
                settem(obj);
                setPreviewObject(obj);
            });
        }
        if (draggingY != null) {
            let [djx, djy, dpjx, dpjy] = draggingY;
            withState(state => {
                let obj = state.get_preview_object();
                obj.jx = dpjx + (e.clientX - djx) / 100 / widgetScale;
                obj.jy = dpjy - (e.clientY - djy) / 100 / widgetScale;

                obj.set_y_scale(
                    snap(signedClamp(obj.y_basis_len(), 0.5, 2.0), 0.05)
                );
                obj.set_y_angle(snap(obj.y_basis_angle(), 5));
                settem(obj);
                setPreviewObject(obj);
            });
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
            ? 'bg-button-cyan-press'
            : 'bg-button-green'} rounded-md pointer-events-all cursor-pointer"
        style={`
            left: ${100 * ix}px;
            top: ${-100 * iy}px;
        `}
        bind:this={xElement}
        on:mousedown={e => {
            let [cX, cY] = getCenterPos(xElement);
            draggingX = [e.clientX, e.clientY, ix, iy];
        }}
    >
        <div
            class="relative flex flex-col items-start gap-5 left-11 top-[-13px]"
        >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round((Math.atan2(iy, ix) * 180) / Math.PI, 2)}°</span
            >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round(Math.hypot(iy, ix), 2)}</span
            >
        </div>
    </button>
    <button
        class="the_dragger bg-center bg-no-repeat abs-centered-rel w-8 h-8 {draggingY !=
        null
            ? 'bg-button-cyan-press'
            : 'bg-button-green'} rounded-md pointer-events-all cursor-pointer"
        style={`
        left: ${100 * jx}px;
        top: ${-100 * jy}px;
    `}
        bind:this={yElement}
        on:mousedown={e => {
            draggingY = [e.clientX, e.clientY, jx, jy];
        }}
    >
        <div
            class="relative flex flex-col items-start gap-5 left-11 top-[-13px]"
        >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round((Math.atan2(jy, jx) * 180) / Math.PI, 2)}°</span
            >
            <span class="font-pusab text-lg max-h-0 text-stroke"
                >{round(Math.hypot(jy, jx), 2)}</span
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
