<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import debounce from "lodash.debounce";
    import { onDestroy, onMount } from "svelte";

    import * as wasm from "wasm-lib";

    import { clamp, hexToRgb, lerp, semitonesToFactor } from "shared-lib/util";
    import {
        BG_TRIGGER,
        GROUND_TRIGGER,
        GROUND_2_TRIGGER,
        SFX_TRIGGER,
        SFX_TRIGGER_SOUNDS,
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
        menuSelectedSFX,
        placedByHover,
        eventElapsed,
        canPlaceEditDelete,
        menuSpeed,
        lastRunColorTrigger,
        setLevelColor,
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
    import { getPlacedUsername } from "../firebase/object";
    import {
        handleSub,
        handleUnsub,
        mouseX,
        mouseY,
        moveCamera,
        zoomCentral,
        zoomGoal,
        zoomTween,
    } from "./view_controls";
    import { pinch, pan } from "svelte-gestures";
    import { isValidObject, objects } from "shared-lib/gd";
    import TriggerRuns from "../widgets/TriggerRuns.svelte";
    import { setCheckedPreviewObject } from "../utils/misc";
    import { playSound } from "../utils/audio";
    import PlacedByText from "../widgets/PlacedByText.svelte";
    import { scale } from "svelte/transition";

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

    let pinchZooming: null | number = null;

    $zoomGoal = state.get_zoom();
    $: {
        changeZoom($zoomTween);
        // state.
    }

    setInterval(() => {
        console.log("vagooby", [...state.get_world_pos(0, 0)]);
    }, 1000);

    const getWorldMousePos = () => {
        // console.log($mouseX, $mouseY);
        return state.get_world_pos(
            $mouseX - (canvas.offsetWidth * window.devicePixelRatio) / 2,
            -($mouseY - (canvas.offsetHeight * window.devicePixelRatio) / 2)
        );
    };
    const setMousePos = (e: { clientX: number; clientY: number }) => {
        $mouseX = e.clientX * window.devicePixelRatio;
        $mouseY = e.clientY * window.devicePixelRatio;
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
        placedByHover.set(null);
    };

    setInterval(() => {
        state.get_chunks_to_sub(); // this just updates time of visible chiunks, doesnt subscriber
        handleUnsub(state);
    }, 50);

    const placePreview = (mx: number, my: number) => {
        let obj = state.get_preview_object();

        obj.x =
            Math.floor(mx / 30) * 30 +
            15 +
            objects[$menuSelectedObject].placeOffsetX;
        obj.y =
            Math.floor(my / 30) * 30 +
            15 +
            objects[$menuSelectedObject].placeOffsetY;
        obj.x_scale_exp = 0;
        obj.x_angle = 0;
        obj.y_scale_exp = 0;
        obj.y_angle = 18;
        // let obj = new wasm.GDObjectOpt(
        //     $menuSelectedObject,
        //     Math.floor(mx / 30) * 30 +
        //         15 +
        //         objects[$menuSelectedObject].placeOffsetX,
        //     Math.floor(my / 30) * 30 +
        //         15 +
        //         objects[$menuSelectedObject].placeOffsetY,
        //     0,
        //     0,
        //     0,
        //     18,
        //     wasm.ZLayer.B4,
        //     0,
        //     wasm.GDColor.white(),
        //     wasm.GDColor.white()
        // );
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
        $menuZLayer = wasm.ZLayer.B2;
        $menuZOrder = 0;
        $menuSelectedSFX = 0;
        $menuSpeed = 0;

        if (obj.id == SFX_TRIGGER) {
            $menuSelectedSFX = Math.floor(
                Math.random() * SFX_TRIGGER_SOUNDS.length
            );
            playSound({
                url: `/assets/audio/sfx/${SFX_TRIGGER_SOUNDS[$menuSelectedSFX]}.ogg`,
                exclusive_channel: "preview sfx",
                speed: semitonesToFactor($menuSpeed),
            });
        }

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
        let sfx_hits_count = hit.filter(i => i.obj.id == SFX_TRIGGER).length;
        let sfx_hit_idx = 0;
        for (let i of hit) {
            switch (i.obj.id) {
                case BG_TRIGGER: {
                    setLevelColor(state, BG_TRIGGER, [
                        i.obj.main_color.r,
                        i.obj.main_color.g,
                        i.obj.main_color.b,
                    ]);

                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);

                    break;
                }
                case GROUND_TRIGGER: {
                    setLevelColor(state, GROUND_TRIGGER, [
                        i.obj.main_color.r,
                        i.obj.main_color.g,
                        i.obj.main_color.b,
                    ]);
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
                case GROUND_2_TRIGGER: {
                    setLevelColor(state, GROUND_2_TRIGGER, [
                        i.obj.main_color.r,
                        i.obj.main_color.g,
                        i.obj.main_color.b,
                    ]);
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
                case SFX_TRIGGER: {
                    let rand =
                        Math.sin(Math.sin(sfx_hit_idx * 6.97) * 6.97) / 2 + 1;

                    playSound({
                        url: `/assets/audio/sfx/${SFX_TRIGGER_SOUNDS[i.obj.main_color.r]}.ogg`,
                        volume: 1.0 / Math.sqrt(sfx_hits_count),
                        speed: semitonesToFactor(i.obj.main_color.g - 12),
                    });
                    sfx_hit_idx += 1;
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

        $zoomGoal = $editorData.zoom;
        state.set_zoom($editorData.zoom);
        state.set_camera_pos($editorData.x, $editorData.y);

        handleSub(state);
    });

    const handleMouseUp = () => {
        if (!dragging) return;

        if (!dragging.thresholdReached && $canPlaceEditDelete) {
            let [mx, my] = getWorldMousePos();
            let hit = state.objects_hit_at(mx, my, 0.0);
            if ($menuTabGroup == TabGroup.Delete) {
                // console.log(hit);
                trySelectAt(mx, my, hit);
            } else {
                if (!tryRunTriggers(hit)) {
                    placePreview(mx, my);
                }
            }
        }

        dragging = null;
    };

    const startDrag = (
        x: number,
        y: number,
        thresholdReached: boolean = false
    ) => {
        let [prevX, prevY] = state.get_camera_pos();
        dragging = {
            prevCameraX: prevX,
            prevCameraY: prevY,
            prevMouseX: x,
            prevMouseY: y,
            thresholdReached,
        };
    };

    const handleDrag = (x: number, y: number) => {
        $mouseX = x;
        $mouseY = y;
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

    const checkHover = debounce(() => {
        let [mx, my] = getWorldMousePos();
        let hit = state.objects_hit_at(mx, my, 0.0);

        if (hit.length == 0) {
            placedByHover.set(null);
            return;
        }

        let top = hit[hit.length - 1];

        getPlacedUsername(top.key(), v => {
            placedByHover.set({ username: v, x: top.obj.x, y: top.obj.y });
        });
    }, 100);

    let editWidgetPos: [number, number] = [0, 0];
    let editWidgetScale = 1;
    let editWidgetVisible = false;

    let originScreen: [number, number] = [0, 0];
    let textZoomScale = 0;

    let buh: [number, number] = [0, 0];

    const getScreenPosZoomCorrected = (
        x: number,
        y: number
    ): [number, number] =>
        [
            ...state.get_screen_pos(x, y).map(v => v / window.devicePixelRatio),
        ] as any;

    const loopFn = () => {
        let obj = state.get_preview_object();

        editWidgetPos = getScreenPosZoomCorrected(obj.x, obj.y);

        editWidgetScale = (1 + state.get_zoom() / 80) / window.devicePixelRatio;
        editWidgetVisible = state.is_preview_visible();

        buh = getScreenPosZoomCorrected(
            $lastRunColorTrigger.bg?.x ?? 0,
            $lastRunColorTrigger.bg?.y ?? 0
        );

        textZoomScale = state.get_zoom_scale();
        let p = state.get_screen_pos(0, 0);
        originScreen = [p[0], p[1]];

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => cancelAnimationFrame(loop));

    $: {
        state.set_show_collidable($editorSettings.showCollidable);
        state.set_hide_triggers($editorSettings.hideTriggers);
        state.set_hide_grid($editorSettings.hideGrid);
        state.set_hide_ground($editorSettings.hideGround);
        state.set_hide_outline($editorSettings.hideOutline);
    }

    $: {
        state.set_bg_color($bgColor.r, $bgColor.g, $bgColor.b);
    }
    $: {
        state.set_ground1_color(
            $ground1Color.r,
            $ground1Color.g,
            $ground1Color.b
        );
    }
    $: {
        state.set_ground2_color(
            $ground2Color.r,
            $ground2Color.g,
            $ground2Color.b
        );
    }
    $: {
        state.set_event_elapsed($eventElapsed);
    }

    $: {
        console.log($bgColor, $ground1Color, $ground2Color);
    }
</script>

<!-- `pointer...` for mobile + desktop, `mouse...` for desktop -->
<svelte:window
    on:pointerup={e => {
        setMousePos(e);
        // if (e.pointerType === "touch") return;

        handleMouseUp();
    }}
    on:pointermove={e => {
        if (pinchZooming == null) {
            setMousePos(e);
            handleDrag($mouseX, $mouseY);
            if (dragging == null) {
                checkHover();
            } else {
                placedByHover.set(null);
                checkHover.cancel();
            }
        }
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

        if (e.ctrlKey || e.metaKey) {
            if (e.key === "=") {
                e.preventDefault();
                zoomCentral($zoomGoal + 4, canvas);
            } else if (e.key === "-") {
                e.preventDefault();
                zoomCentral($zoomGoal - 4, canvas);
            } else {
                return;
            }

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
    on:pointerdown={e => {
        setMousePos(e);
        if (e.button == 0) {
            startDrag(
                e.clientX * window.devicePixelRatio,
                e.clientY * window.devicePixelRatio
            );
        }
    }}
    on:wheel={e => {
        setMousePos(e);
        e.preventDefault();
        $zoomGoal = $zoomGoal - e.deltaY / Math.max(Math.abs(e.deltaY), 1);
    }}
    use:pinch
    on:pinch={e => {
        dragging = null;
        $mouseX = e.detail.center.x * window.devicePixelRatio;
        $mouseY = e.detail.center.y * window.devicePixelRatio;
        if (pinchZooming == null) {
            pinchZooming = $zoomTween;
        } else {
            console.log(pinchZooming, e.detail.scale);
            $zoomGoal =
                pinchZooming + (Math.log(e.detail.scale) / Math.log(2)) * 6;
        }
    }}
    on:pinchup={() => {
        pinchZooming = null;
    }}
/>

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
        <Widget
            position={[
                originScreen[0] / window.devicePixelRatio,
                originScreen[1] / window.devicePixelRatio,
            ]}
            scale={textZoomScale}
        >
            <DeleteTexts />
        </Widget>
    {/if}
    <Widget
        position={[
            originScreen[0] / window.devicePixelRatio,
            originScreen[1] / window.devicePixelRatio,
        ]}
        scale={textZoomScale}
    >
        <TriggerRuns />
    </Widget>
    {#if $loginData.currentUserData != null}
        <!-- <Widget position={[30, -30]} scale={1.0} screenCenter={false}>
            <ObjectInfo bind:state />
        </Widget> -->
    {/if}
    {#if $placedByHover != null && !$editorSettings.hidePlacedTooltip}
        <Widget
            position={getScreenPosZoomCorrected(
                $placedByHover.x,
                $placedByHover.y
            )}
            scale={1.0}
        >
            <PlacedByText username={$placedByHover.username} />
        </Widget>
    {/if}
</div>
