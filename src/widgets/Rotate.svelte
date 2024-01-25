<script lang="ts">
    import Input from "../components/Input.svelte";
    import {
        addCallback as addUpdateCallback,
        setPreviewObject,
    } from "../state";
    import { onDestroy } from "svelte";
    import { widgetData } from "../stores";
    import { clamp, getCenterPos, snap } from "shared-lib";

    let rotating: number | null = null;

    const getMouseAngle = (e: MouseEvent) => {
        let [cX, cY] = getCenterPos(circleElement);
        return (-Math.atan2(e.clientY - cY, e.clientX - cX) * 180) / Math.PI;
    };

    // $: console.log($widgetData.angle);

    let cb = addUpdateCallback(state => {
        let obj = state.get_preview_object();
        let xAngle = obj.x_basis_angle();

        // obj.rotate(0.1);
        // setPreviewObject(obj);
        if ($widgetData.angle != $widgetData.prevAngle) {
            // console.log($widgetData.angle);
            // console.log(xAngle);
            obj.rotate(-$widgetData.angle + xAngle);
            setPreviewObject(obj);
        }
    });

    onDestroy(() => cb.remove());

    let circleElement: HTMLDivElement;
</script>

<svelte:window
    on:mouseup={() => {
        rotating = null;
    }}
    on:mousemove={e => {
        if (rotating != null) {
            $widgetData.angle = snap(getMouseAngle(e) - rotating, 5);
        }
    }}
/>

<div class="absolute text-white">
    <div
        class="abs-centered-rel w-72 h-72 border-white/50 border-dashed border-2 rounded-full"
        bind:this={circleElement}
    />
    <button
        class="the_dragger bg-center bg-no-repeat abs-centered-rel w-16 h-16 {rotating !=
        null
            ? 'bg-button-cyan-press'
            : 'bg-button-green'} rounded-full pointer-events-all cursor-pointer"
        style={`
            transform: translate(${(Math.cos((-$widgetData.angle * Math.PI) / 180) * 284) / 2 - 32}px, ${(Math.sin((-$widgetData.angle * Math.PI) / 180) * 284) / 2 - 32}px)
        `}
        on:mousedown={e => {
            rotating = getMouseAngle(e) - $widgetData.angle;
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
