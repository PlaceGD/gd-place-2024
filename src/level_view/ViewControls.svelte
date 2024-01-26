<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import debounce from "lodash.debounce";
    import { onMount } from "svelte";
    import TinyGesture from "tinygesture";

    import * as wasm from "wasm-lib";

    import { clamp, hexToRgb, lerp } from "shared-lib";
    import { subChunk, unsubChunk } from "../firebase/chunks";
    import { TabGroup, menuSettings } from "../stores";
    import { KEYBINDS } from "../place_menu/edit/edit_tab";

    import Toast from "../utils/toast";
    import LocalSettingsFactory from "../utils/local_settings";
    import { isMobile } from "../utils/document";
    import { decodeString } from "shared-lib";
    import Widget from "../widgets/Widget.svelte";

    export let state: wasm.StateWrapper;
    export let canvas: HTMLCanvasElement;

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

    const getWorldMousePos = () =>
        state.get_world_pos(
            mouseX - canvas.offsetWidth / 2,
            -(mouseY - canvas.offsetHeight / 2)
        );

    const changeZoom = (z: number) => {
        let [mx, my] = getWorldMousePos();
        let [cx, cy] = state.get_camera_pos();
        let prev_zoom_scale = state.get_zoom_scale();

        state.set_zoom(z);

        let zoom_scale_change = state.get_zoom_scale() / prev_zoom_scale;

        state.set_camera_pos(
            lerp(mx, cx, 1 / zoom_scale_change),
            lerp(my, cy, 1 / zoom_scale_change)
        );
        handleSub();
    };

    $: {
        changeZoom($zoomTween);
    }

    $: {
        if ($menuSettings.selectedGroup == TabGroup.Delete) {
            state.set_preview_visibility(false);
        } else {
            state.deselect_object();
        }
    }

    const changeBgColor = (event: any) => {
        let c = hexToRgb(event.target.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_bg_color(r, g, b);
        }
    };
    const changeGround1Color = (event: any) => {
        let c = hexToRgb(event.target.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_ground1_color(r, g, b);
        }
    };
    const changeGround2Color = (event: any) => {
        let c = hexToRgb(event.target.value);
        if (c != null) {
            let { r, g, b } = c;
            state.set_ground2_color(r, g, b);
        }
    };

    const handleSub = () => {
        for (let chunk of state.get_chunks_to_sub()) {
            subChunk(
                [chunk.x, chunk.y],
                data => {
                    let key = data.key;
                    if (key != null) {
                        try {
                            let obj = wasm.GDObjectOpt.from_bytes(
                                decodeString(data.val(), 126)
                            );

                            state.add_object(key, obj);
                        } catch (e: any) {
                            console.error(
                                "(Failed in `GDObjectOpt.from_bytes`)"
                            );
                            Toast.showErrorToast(e.display());
                        }
                    }
                },
                data => {
                    let key = data.key;
                    if (key != null) {
                        state.delete_object(key);
                    }
                }
            );
        }
    };
    const handleUnsub = () => {
        for (let chunk of state.get_chunks_to_unsub()) {
            unsubChunk([chunk.x, chunk.y]);
        }
    };

    setInterval(() => {
        state.get_chunks_to_sub(); // this just updates time of visible chiunks, doesnt subscriber
        handleUnsub();
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

    const savePos = debounce(() => {
        // dreaming insanity is so cool and he is so cool
        // and i wish him nothing but the best
        // he is my favorite skrunkle and such a good guy
        // so cool
        let zoom = Math.round(state.get_zoom());
        let [x, y] = state.get_camera_pos().map(Math.round);

        history.replaceState({}, "", `?x=${x}&y=${y}&zoom=${zoom}`);

        editorData.x = x;
        editorData.y = y;
        editorData.zoom = zoom;
    }, 200);

    const editorData = LocalSettingsFactory("editorData", {
        x: 0,
        y: 0,
        zoom: 0,
    });

    onMount(() => {
        let data = new URLSearchParams(window.location.search);

        if (data.get("x")) {
            editorData.x = parseFloat(data.get("x")!);
        }
        if (data.get("y")) {
            editorData.y = parseFloat(data.get("y")!);
        }
        if (data.get("zoom")) {
            editorData.zoom = parseFloat(data.get("zoom")!);
        }

        zoomGoal = editorData.zoom;
        zoomTween.set(zoomGoal);
        state.set_zoom(editorData.zoom);
        state.set_camera_pos(editorData.x, editorData.y);

        handleSub();
    });

    const handleShowObjPreview = () => {
        if (!dragging) return;

        if (!dragging.thresholdReached) {
            if ($menuSettings.selectedGroup == TabGroup.Delete) {
                let [mx, my] = getWorldMousePos();
                state.try_select_at(mx, my);
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

                state.set_camera_pos(
                    (1 / z) * (dragging.prevMouseX - x) + dragging.prevCameraX,
                    (1 / z) * (-dragging.prevMouseY + y) + dragging.prevCameraY
                );

                savePos();
                handleSub();
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
                document.getElementById("gesture-target")!
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
</script>

<!-- `pointer...` for mobile + desktop, `mouse...` for desktop -->
<svelte:window
    on:mouseup={() => {
        handleShowObjPreview();
    }}
    on:mousemove={e => {
        handleDrag(e.pageX, e.pageY);
    }}
    on:resize={() => {
        handleSub();
    }}
/>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    class="absolute w-full h-full touch-none"
    id="gesture-target"
    tabindex="-1"
    on:keydown={e => {
        for (let v of Object.values(KEYBINDS)) {
            if (
                e.key.toLowerCase() == v.shortcut.key.toLowerCase() &&
                e.shiftKey == v.shortcut.shift &&
                e.altKey == v.shortcut.alt
            ) {
                e.preventDefault();
                let obj = state.get_preview_object();
                v.cb(obj);
                state.set_preview_object(obj);
                // console.log(state.get_preview_object().debug());
            }
        }
    }}
    on:mousemove={e => {
        mouseX = e.pageX;
        mouseY = e.pageY;
    }}
    on:mousedown={e => {
        if (e.button == 0) {
            startDrag(e.pageX, e.pageY);
        }
    }}
    on:wheel={e => {
        zoomGoal = clamp(zoomGoal - (e.deltaY / 100) * 2, -36, 36);
        zoomTween.set(zoomGoal);
        savePos();
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

<div class="absolute overflow-visible w-full h-full pointer-events-none">
    <Widget />
</div>
