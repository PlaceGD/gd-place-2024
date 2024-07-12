<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import debounce from "lodash.debounce";
    import { onDestroy, onMount } from "svelte";
    import TinyGesture from "tinygesture";

    import * as wasm from "wasm-lib";

    import { clamp, hexToRgb, lerp } from "shared-lib/util";
    import { decodeString } from "shared-lib/base_util";
    import { subChunk, unsubChunk } from "../firebase/chunks";
    import {
        TabGroup,
        addDeleteText,
        editorData,
        loginData,
        menuSettings,
        selectedObject,
    } from "../stores";
    import {
        MOVE_KEYBINDS,
        TRANSFORM_KEYBINDS,
        WidgetType,
    } from "../place_menu/edit/edit_tab";

    import Toast from "../utils/toast";
    import { isMobile } from "../utils/document";
    import Widget from "../widgets/Widget.svelte";
    import { addCallback } from "../state";
    import Rotate from "../widgets/Rotate.svelte";
    import Scale from "../widgets/Scale.svelte";
    import Warp from "../widgets/Warp.svelte";
    import DeleteTexts from "../widgets/DeleteTexts.svelte";
    import ObjectInfo from "../widgets/ObjectInfo.svelte";
    import { getPlacedUsername } from "../firebase/object";
    import { handleSub, handleUnsub, moveCamera } from "./view_controls";

    export let state: wasm.StateWrapper;
    export let canvas: HTMLCanvasElement;
    export let isFocused: boolean = false;

    let dragging: null | {
        prevMouseX: number;
        prevMouseY: number;
        prevCameraX: number;
        prevCameraY: number;
        thresholdReached: boolean;
    } = null;

    let [mouseX, mouseY] = [0, 0];

    let zoomGoal = state.get_zoom();
    const zoomTween = tweened(0, {
        duration: 100,
        easing: cubicOut,
    });

    const getWorldMousePos = () => {
        // console.log(mouseX, mouseY);
        return state.get_world_pos(
            mouseX - canvas.offsetWidth / 2,
            -(mouseY - canvas.offsetHeight / 2)
        );
    };

    const changeZoom = (z: number) => {
        let [mx, my] = getWorldMousePos();
        let [cx, cy] = state.get_camera_pos();
        let prev_zoom_scale = state.get_zoom_scale();

        state.set_zoom(z);

        let zoom_scale_change = state.get_zoom_scale() / prev_zoom_scale;

        moveCamera(
            state,
            lerp(mx, cx, 1 / zoom_scale_change),
            lerp(my, cy, 1 / zoom_scale_change)
        );
    };

    $: {
        changeZoom($zoomTween);
    }

    $: {
        if ($menuSettings.selectedGroup == TabGroup.Delete) {
            state.set_preview_visibility(false);
        } else {
            $selectedObject = null;
            state.deselect_object();
        }
    }

    const changeBgColor = (
        event: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        let c = hexToRgb(event.currentTarget.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_bg_color(r, g, b);
        }
    };
    const changeGround1Color = (
        event: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        let c = hexToRgb(event.currentTarget.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_ground1_color(r, g, b);
        }
    };
    const changeGround2Color = (
        event: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        let c = hexToRgb(event.currentTarget.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_ground2_color(r, g, b);
        }
    };

    setInterval(() => {
        state.get_chunks_to_sub(); // this just updates time of visible chiunks, doesnt subscriber
        handleUnsub(state);
    }, 50);

    const placePreview = () => {
        let [mx, my] = getWorldMousePos();

        let obj = new wasm.GDObjectOpt(
            $menuSettings.selectedObject,
            Math.floor(mx / 30) * 30 + 15,
            Math.floor(my / 30) * 30 + 15,
            0,
            0,
            0,
            18,
            wasm.ZLayer.B4,
            0,
            wasm.GDColor.white(),
            wasm.GDColor.white()
        );
        $menuSettings.selectedMainColor = {
            hue: 0,
            x: 0,
            y: 0,
            opacity: 1,
            blending: false,
        };
        $menuSettings.selectedDetailColor = {
            hue: 0,
            x: 0,
            y: 0,
            opacity: 1,
            blending: false,
        };
        $menuSettings.zLayer = wasm.ZLayer.B4;
        $menuSettings.zOrder = 0;
        // $menuSettings.zLayer = wasm.ZLayer.B1;

        state.set_preview_object(obj);
        state.set_preview_visibility(true);
    };

    onMount(() => {
        let data = new URLSearchParams(window.location.search);

        if (data.get("x")) {
            $editorData.x = parseFloat(data.get("x")!);
        }
        if (data.get("y")) {
            $editorData.y = parseFloat(data.get("y")!);
        }
        if (data.get("zoom")) {
            $editorData.zoom = parseFloat(data.get("zoom")!);
        }

        zoomGoal = $editorData.zoom;
        zoomTween.set(zoomGoal);
        state.set_zoom($editorData.zoom);
        state.set_camera_pos($editorData.x, $editorData.y);

        handleSub(state);
    });

    const handleShowObjPreview = () => {
        if (!dragging) return;

        if (!dragging.thresholdReached) {
            if ($menuSettings.selectedGroup == TabGroup.Delete) {
                let [mx, my] = getWorldMousePos();

                let selected = state.try_select_at(mx, my);
                if (selected != undefined) {
                    // console.log(selected.key());
                    $selectedObject = {
                        id: selected.id,
                        mainColor: selected.main_color,
                        detailColor: selected.detail_color,
                        namePlaced: null,
                        zLayer: selected.z_layer,
                        zOrder: selected.z_order,
                    };
                    getPlacedUsername(selected.key(), v => {
                        if ($selectedObject != null) {
                            $selectedObject.namePlaced = v;
                        }
                    });
                } else {
                    $selectedObject = null;
                }
            } else {
                placePreview();
            }
        }

        dragging = null;
    };

    const startDrag = (x: number, y: number) => {
        let [prevX, prevY] = state.get_camera_pos();
        dragging = {
            prevCameraX: prevX,
            prevCameraY: prevY,
            prevMouseX: x,
            prevMouseY: y,
            thresholdReached: false,
        };
    };

    const handleDrag = (x: number, y: number) => {
        mouseX = x;
        mouseY = y;
        if (dragging != null) {
            if (dragging.thresholdReached) {
                let z = state.get_zoom_scale();

                moveCamera(
                    state,
                    (1 / z) * (dragging.prevMouseX - x) + dragging.prevCameraX,
                    (1 / z) * (-dragging.prevMouseY + y) + dragging.prevCameraY
                );
            } else {
                if (
                    Math.hypot(
                        x - dragging.prevMouseX,
                        y - dragging.prevMouseY
                    ) > 30.0
                ) {
                    dragging.thresholdReached = true;
                    dragging.prevMouseX = x;
                    dragging.prevMouseY = y;
                }
            }
        }
    };

    const dpr = window.devicePixelRatio;

    onMount(() => {
        // svelte-gestures is not updated for svelte 4.0
        if (isMobile()) {
            const gestures = new TinyGesture(
                document.getElementById("gesture-target")!,
                {
                    mouseSupport: false,
                }
            );

            // TODO: make these numbers better on different screens?
            gestures.on("pinch", () => {
                console.log(gestures.scale!);
                //zoomGoal = clamp(zoomGoal - gestures.scale! / 100, -36, 36);

                zoomTween.set(gestures.scale!);
                //savePos();
            });

            gestures.on("panstart", () => {
                startDrag(
                    gestures.touchMoveX! * dpr,
                    gestures.touchMoveY! * dpr
                );
            });

            gestures.on("panmove", () => {
                handleDrag(
                    gestures.touchMoveX! * dpr,
                    gestures.touchMoveY! * dpr
                );
            });

            gestures.on("tap", () => {
                mouseX = gestures.touchStartX!;
                mouseY = gestures.touchStartY!;
                handleShowObjPreview();
            });
        }
    });

    let editWidgetPos: [number, number] = [0, 0];
    let editWidgetScale = 1;
    let editWidgetVisible = false;

    let originScreen: [number, number] = [0, 0];
    let textZoomScale = 0;

    let cb = addCallback(state => {
        let obj = state.get_preview_object();

        let p = state.get_screen_pos(obj.x, obj.y);
        editWidgetPos = [p[0], p[1]];

        editWidgetScale = 1 + state.get_zoom() / 80;
        editWidgetVisible = state.is_preview_visible();

        textZoomScale = state.get_zoom_scale();
        p = state.get_screen_pos(0, 0);
        originScreen = [p[0], p[1]];
    });
    onDestroy(() => cb.remove());
</script>

<!-- `pointer...` for mobile + desktop, `mouse...` for desktop -->
<svelte:window
    on:mouseup={e => {
        handleShowObjPreview();
    }}
    on:mousemove={e => {
        handleDrag(e.clientX, e.clientY);
    }}
    on:resize={() => {
        handleSub(state);
    }}
    on:keydown={e => {
        if (document.activeElement?.tagName == "INPUT") return;

        for (let v of [
            ...Object.values(TRANSFORM_KEYBINDS),
            ...Object.values(MOVE_KEYBINDS).flatMap(v => Object.values(v)),
        ]) {
            if (
                e.key.toLowerCase() == v.shortcut.key.toLowerCase() &&
                e.shiftKey == v.shortcut.shift &&
                e.altKey == v.shortcut.alt
            ) {
                e.preventDefault();
                let obj = state.get_preview_object();
                v.cb(obj);
                state.set_preview_object(obj);
            }
        }
    }}
/>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div
    class="absolute w-full h-full touch-none"
    id="gesture-target"
    tabindex="0"
    on:focus={() => (isFocused = true)}
    on:blur={() => (isFocused = false)}
    on:mousemove={e => {
        mouseX = e.clientX;
        mouseY = e.clientY;
    }}
    on:mousedown={e => {
        if (e.button == 0) {
            startDrag(e.clientX, e.clientY);
        }
    }}
    on:wheel={e => {
        zoomGoal = clamp(zoomGoal - (e.deltaY / 100) * 2, -36, 36);
        zoomTween.set(zoomGoal);
    }}
/>

<div class="absolute flex flex-col">
    <input
        type="color"
        tabindex="-1"
        on:input={e => {
            changeBgColor(e);
        }}
    />
    <input
        type="color"
        tabindex="-1"
        on:input={e => {
            changeGround1Color(e);
        }}
    />
    <input
        type="color"
        tabindex="-1"
        on:input={e => {
            changeGround2Color(e);
        }}
    />
</div>

<div class="absolute w-full h-full overflow-visible pointer-events-none">
    {#if editWidgetVisible}
        <Widget position={editWidgetPos} scale={editWidgetScale}>
            {#if $menuSettings.selectedWidget == WidgetType.Rotate}
                <Rotate />
            {:else if $menuSettings.selectedWidget == WidgetType.Scale}
                <Scale />
            {:else if $menuSettings.selectedWidget == WidgetType.Warp}
                <Warp widgetScale={editWidgetScale} />
            {/if}
        </Widget>
    {/if}
    <Widget position={originScreen} scale={textZoomScale}>
        <DeleteTexts />
    </Widget>
    {#if $loginData.currentUserData != null}
        <Widget position={[60, -60]} scale={1.0} screenCenter={false}>
            <ObjectInfo bind:state />
        </Widget>
    {/if}
</div>
