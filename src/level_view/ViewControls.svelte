<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import debounce from "lodash.debounce";
    import { onDestroy, onMount } from "svelte";

    import * as wasm from "wasm-lib";

    import { clamp, hexToRgb, lerp } from "shared-lib/util";
    import {
        BG_TRIGGER,
        GROUND_TRIGGER,
        GROUND_2_TRIGGER,
    } from "shared-lib/nexusgen";
    import { decodeString } from "shared-lib/base_util";
    import { subChunk, unsubChunk } from "../firebase/chunks";
    import {
        TabGroup,
        addDeleteText,
        editorData,
        editorSettings,
        loginData,
        selectedObject,
        menuSelectedObject,
        menuMainColor,
        menuDetailColor,
        menuZLayer,
        menuZOrder,
        menuTabGroup,
        menuOpenWidget,
        addTriggerRun,
        bgColor,
        ground1Color,
        ground2Color,
    } from "../stores";
    import {
        MOVE_KEYBINDS,
        TRANSFORM_KEYBINDS,
        WidgetType,
    } from "../place_menu/edit/edit_tab";

    import Toast from "../utils/toast";
    import { isMobile } from "../utils/document";
    import Widget from "../widgets/Widget.svelte";
    // import { addCallback } from "../state";
    import Rotate from "../widgets/Rotate.svelte";
    import Scale from "../widgets/Scale.svelte";
    import Warp from "../widgets/Warp.svelte";
    import DeleteTexts from "../widgets/DeleteTexts.svelte";
    import ObjectInfo from "../widgets/ObjectInfo.svelte";
    import { getPlacedUsername } from "../firebase/object";
    import { handleSub, handleUnsub, moveCamera } from "./view_controls";
    import { pinch } from "svelte-gestures";
    import { isValidObject, objects } from "shared-lib/gd";
    import TriggerRuns from "../widgets/TriggerRuns.svelte";
    import { setCheckedPreviewObject } from "../utils/misc";

    export let state: wasm.State;
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
    $: {
        changeZoom($zoomTween);
    }

    const getWorldMousePos = () => {
        // console.log(mouseX, mouseY);
        return state.get_world_pos(
            mouseX - (canvas.offsetWidth * window.devicePixelRatio) / 2,
            -(mouseY - (canvas.offsetHeight * window.devicePixelRatio) / 2)
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

    setInterval(() => {
        state.get_chunks_to_sub(); // this just updates time of visible chiunks, doesnt subscriber
        handleUnsub(state);
    }, 50);

    const placePreview = (mx: number, my: number) => {
        let obj = new wasm.GDObjectOpt(
            $menuSelectedObject,
            Math.floor(mx / 30) * 30 +
                15 +
                objects[$menuSelectedObject].placeOffsetX,
            Math.floor(my / 30) * 30 +
                15 +
                objects[$menuSelectedObject].placeOffsetY,
            0,
            0,
            0,
            18,
            wasm.ZLayer.B4,
            0,
            wasm.GDColor.white(),
            wasm.GDColor.white()
        );
        $menuMainColor = {
            hue: 0,
            x: 0,
            y: 0,
            opacity: 1,
            blending: false,
        };
        $menuDetailColor = {
            hue: 0,
            x: 0,
            y: 0,
            opacity: 1,
            blending: false,
        };
        $menuZLayer = wasm.ZLayer.B4;
        $menuZOrder = 0;
        // $menuZLayer = wasm.ZLayer.B1;

        if (setCheckedPreviewObject(state, obj)) {
            state.set_preview_visibility(true);
        }
    };

    let selectDepth = 0;
    const trySelectAt = (mx: number, my: number, hit: wasm.HitObjectInfo[]) => {
        if (hit.length == 0) {
            $selectedObject = null;
            state.deselect_object();
            return;
        }
        if (selectDepth >= hit.length) {
            selectDepth = 0;
        }
        selectDepth += 1;
        let selected = hit[selectDepth - 1];

        state.set_selected_object(selected.key());

        $selectedObject = {
            id: selected.obj.id,
            mainColor: selected.obj.main_color,
            detailColor: selected.obj.detail_color,
            namePlaced: null,
            zLayer: selected.obj.z_layer,
            zOrder: selected.obj.z_order,
        };
        getPlacedUsername(selected.key(), v => {
            if ($selectedObject != null) {
                $selectedObject.namePlaced = v;
            }
        });
    };

    const tryRunTriggers = (hit: wasm.HitObjectInfo[]): boolean => {
        let triggersRun = false;
        for (let i of hit) {
            switch (i.obj.id) {
                case BG_TRIGGER: {
                    $bgColor = {
                        r: i.obj.main_color.r,
                        g: i.obj.main_color.g,
                        b: i.obj.main_color.b,
                    };

                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
                case GROUND_TRIGGER: {
                    $ground1Color = {
                        r: i.obj.main_color.r,
                        g: i.obj.main_color.g,
                        b: i.obj.main_color.b,
                    };
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
                case GROUND_2_TRIGGER: {
                    $ground2Color = {
                        r: i.obj.main_color.r,
                        g: i.obj.main_color.g,
                        b: i.obj.main_color.b,
                    };
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
            }
        }
        return triggersRun;
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
            let [mx, my] = getWorldMousePos();
            let hit = state.objects_hit_at(mx, my, 0.0);
            if ($menuTabGroup == TabGroup.Delete) {
                // console.log(hit);
                trySelectAt(mx, my, hit);

                // let selected = state.try_select_at(mx, my);
                // if (selected != undefined) {
                //     // console.log(selected.key());
                //     $selectedObject = {
                //         id: selected.id,
                //         mainColor: selected.main_color,
                //         detailColor: selected.detail_color,
                //         namePlaced: null,
                //         zLayer: selected.z_layer,
                //         zOrder: selected.z_order,
                //     };
                //     getPlacedUsername(selected.key(), v => {
                //         if ($selectedObject != null) {
                //             $selectedObject.namePlaced = v;
                //         }
                //     });
                // } else {
                //     $selectedObject = null;
                // }
            } else {
                if (!tryRunTriggers(hit)) {
                    placePreview(mx, my);
                }
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

    // const dpr = window.devicePixelRatio;

    // onMount(() => {
    //     // svelte-gestures is not updated for svelte 4.0
    //     if (isMobile()) {
    //         const gestures = new TinyGesture(
    //             document.getElementById("gesture-target")!,
    //             {
    //                 mouseSupport: false,
    //             }
    //         );

    //         // TODO: make these numbers better on different screens?
    //         gestures.on("pinch", () => {
    //             console.log(gestures.scale!);
    //             //zoomGoal = clamp(zoomGoal - gestures.scale! / 100, -36, 36);

    //             zoomTween.set(gestures.scale!);
    //             //savePos();
    //         });

    //         gestures.on("panstart", () => {
    //             startDrag(
    //                 gestures.touchMoveX! * dpr,
    //                 gestures.touchMoveY! * dpr
    //             );
    //         });

    //         gestures.on("panmove", () => {
    //             handleDrag(
    //                 gestures.touchMoveX! * dpr,
    //                 gestures.touchMoveY! * dpr
    //             );
    //         });

    //         gestures.on("tap", () => {
    //             mouseX = gestures.touchStartX!;
    //             mouseY = gestures.touchStartY!;
    //             handleShowObjPreview();
    //         });
    //     }
    // });

    let editWidgetPos: [number, number] = [0, 0];
    let editWidgetScale = 1;
    let editWidgetVisible = false;

    let originScreen: [number, number] = [0, 0];
    let textZoomScale = 0;

    const loopFn = () => {
        let obj = state.get_preview_object();

        let p = state.get_screen_pos(obj.x, obj.y);
        editWidgetPos = [
            p[0] / window.devicePixelRatio,
            p[1] / window.devicePixelRatio,
        ];

        editWidgetScale = 1 + state.get_zoom() / 80;
        editWidgetVisible = state.is_preview_visible();

        textZoomScale = state.get_zoom_scale();
        p = state.get_screen_pos(0, 0);
        originScreen = [p[0], p[1]];

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => cancelAnimationFrame(loop));
</script>

<!-- `pointer...` for mobile + desktop, `mouse...` for desktop -->
<svelte:window
    on:mouseup={e => {
        handleShowObjPreview();
    }}
    on:mousemove={e => {
        handleDrag(
            e.clientX * window.devicePixelRatio,
            e.clientY * window.devicePixelRatio
        );
    }}
    on:resize={() => {
        handleSub(state);
    }}
    on:keydown={e => {
        if (
            document.activeElement instanceof HTMLInputElement &&
            document.activeElement.type == "text"
        ) {
            return;
        }

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
                setCheckedPreviewObject(state, obj);
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
        mouseX = e.clientX * window.devicePixelRatio;
        mouseY = e.clientY * window.devicePixelRatio;
    }}
    on:mousedown={e => {
        if (e.button == 0) {
            startDrag(
                e.clientX * window.devicePixelRatio,
                e.clientY * window.devicePixelRatio
            );
        }
    }}
    on:wheel|passive={e => {
        zoomGoal = clamp(zoomGoal - (e.deltaY / 100) * 2, -4, 36);
        zoomTween.set(zoomGoal);
    }}
    use:pinch
    on:pinch={e => {
        console.log(e.detail.scale);
        alert(`cock ${e.detail.scale}`);
    }}
    on:pinchup={e => {}}
/>

<div class="absolute flex flex-col">
    <!-- <input
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
    /> -->
    <!-- <button
        ><img
            class="h-32"
            src="https://cdn.discordapp.com/attachments/996434758227734661/1268214124186177617/426422705_694498856197195_8206388669893447202_n.jpg?ex=66ab9bf1&is=66aa4a71&hm=7af9776f348ea0c9e65294531c9acbf88a48cf45586e7f299018a827c975cd45&"
            alt=""
            on:click={() => {
                // window.app.textContent = " ";
                // window.app.appendChild(document.createElement("img"));
                // window.app.children[0].src =
                //     "https://ih1.redbubble.net/image.2177348283.3648/flat,750x1000,075,f.jpg";
            }}
        /></button
    > -->
</div>

<div class="absolute w-full h-full overflow-visible pointer-events-none">
    {#if editWidgetVisible}
        <Widget position={editWidgetPos} scale={editWidgetScale}>
            {#if $menuOpenWidget == WidgetType.Rotate}
                <Rotate bind:state />
            {:else if $menuOpenWidget == WidgetType.Scale}
                <Scale bind:state />
            {:else if $menuOpenWidget == WidgetType.Warp}
                <Warp bind:state widgetScale={editWidgetScale} />
            {/if}
        </Widget>
    {/if}
    {#if !$editorSettings.hideDeleteText}
        <Widget position={originScreen} scale={textZoomScale}>
            <DeleteTexts />
        </Widget>
    {/if}
    <Widget position={originScreen} scale={textZoomScale}>
        <TriggerRuns />
    </Widget>
    {#if $loginData.currentUserData != null}
        <!-- <Widget position={[30, -30]} scale={1.0} screenCenter={false}>
            <ObjectInfo bind:state />
        </Widget> -->
    {/if}
</div>
