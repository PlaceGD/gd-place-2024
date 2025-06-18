<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import debounce from "lodash.debounce";
    import { onDestroy, onMount } from "svelte";

    import * as wasm from "wasm-lib";

    import player_start_help from "./assets/player_start_help.png?url";
    import player_goal_help from "./assets/player_goal_help.png?url";
    import player_goal from "./assets/player_goal.png?url";

    import { clamp, hexToRgb, lerp, semitonesToFactor } from "shared-lib/util";
    import {
        BG_TRIGGER,
        GROUND_TRIGGER,
        GROUND_2_TRIGGER,
        SFX_TRIGGER,
        SFX_TRIGGER_SOUNDS,
        SONG_TRIGGER,
        SONG_TRIGGER_SONGS,
        LEVEL_WIDTH_UNITS,
        LEVEL_HEIGHT_UNITS,
        END_POS_X,
        END_POS_Y,
        END_RADIUS,
        COLOR_TRIGGERS,
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
        canPlaceEditDelete,
        menuSpeed,
        lastRunColorTrigger,
        setLevelColor,
        menuSelectedSong,
        songPlaying,
        eventStartTime,
        canPlacePreview,
        setPreviewColor,
        resetPreviewColor,
        chooseRandomTriggerColor,
        chooseDefaultColor,
        songPlayingIsPreview,
        eventEndTime,
        timeLeft,
        eventStatus,
        nowStore,
        openMenu,
        ExclusiveMenus,
        viewingLevelAfterEvent,
        setNameSeconds,
        DEFAULT_SETTINGS,
        getServerNow,
        addDebugTimeOffset,
    } from "../stores";
    import {
        getTransformedPlaceOffset,
        MISC_KEYBINDS,
        MOVE_KEYBINDS,
        TRANSFORM_KEYBINDS,
        WidgetType,
    } from "../place_menu/edit/edit_tab";

    import Toast from "../utils/toast";
    import { isMobile } from "../utils/document";
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
        zoomCenterX,
        zoomCenterY,
        zoomCentral,
        zoomGoal,
        zoomTween,
    } from "./view_controls";
    import { pinch, pan, tap } from "svelte-gestures";
    import { colors, isValidObject, objects } from "shared-lib/gd";
    import TriggerRuns from "../widgets/TriggerRuns.svelte";
    import { setCheckedPreviewObject } from "../utils/misc";
    import { playSound, stopSound } from "../utils/audio";
    import PlacedByText from "../widgets/PlacedByText.svelte";
    import { scale } from "svelte/transition";
    import { SFX_SOUNDS, SONG_SOUNDS } from "../place_menu/edit/sfx_tab";

    import Image from "../components/Image.svelte";
    import ClosableWindow from "../components/ClosableWindow.svelte";
    import { isGuideActive, walmart } from "../guide/guide";
    import LevelWidget from "../widgets/LevelWidget.svelte";
    import { set } from "firebase/database";
    import { skipForward, togglePause } from "../timelapse";
    import { toast } from "@zerodevx/svelte-toast";
    import { getSignupTimestamp } from "../firebase/moderation";

    export let state: wasm.State;
    export let canvas: HTMLCanvasElement;
    export let isFocused: boolean = false;

    let cinematic = false;
    let zoomVel = 0;

    const GESTURE_TARGET_ID: string = "gesture-target";
    const isGestureTarget = <T,>(
        e: Event & {
            currentTarget: EventTarget & T;
        }
    ) => {
        return (e.target as unknown as HTMLElement).id === GESTURE_TARGET_ID;
    };

    let dragging: null | {
        prevMouseX: number;
        prevMouseY: number;
        prevCameraX: number;
        prevCameraY: number;
        thresholdReached: boolean;
    } = null;

    let panzooming: null | {
        prevTouches: { x: number; y: number }[];
        prevFrameTouches: { x: number; y: number }[];
        prevCameraX: number;
        prevCameraY: number;
        prevZoom: number;
        prevZoomScale: number;
        thresholdReached: boolean;
    } = null;

    //let pinchZooming: null | number = null;

    $zoomGoal = state.get_zoom();
    $: {
        changeZoom($zoomTween);
    }

    const screenToWorld = (x: number, y: number) => {
        return state.get_world_pos(
            x - (canvas.offsetWidth * window.devicePixelRatio) / 2,
            -(y - (canvas.offsetHeight * window.devicePixelRatio) / 2)
        );
    };

    const getWorldMousePos = () => screenToWorld($mouseX, $mouseY);
    const getWorldZoomCenter = () => screenToWorld($zoomCenterX, $zoomCenterY);

    const setMousePos = (e: { clientX: number; clientY: number }) => {
        $mouseX = e.clientX * window.devicePixelRatio;
        $mouseY = e.clientY * window.devicePixelRatio;
    };

    const setZoomCenter = (x: number, y: number) => {
        $zoomCenterX = x * window.devicePixelRatio;
        $zoomCenterY = y * window.devicePixelRatio;
    };

    const changeZoom = (z: number) => {
        let [mx, my] = getWorldZoomCenter();
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

    $: {
        $nowStore;
        handleSub(state);
    }

    let interval = setInterval(() => {
        if ($timeLeft < 0) {
            handleSub(state);
        }

        state.get_chunks_to_sub(); // this just updates time of visible chiunks, doesnt subscriber
        handleUnsub(state);
    }, 50);

    const placePreview = (mx: number, my: number) => {
        if ($eventStatus != "during") {
            return;
        }

        let obj = state.get_preview_object();

        let [gagaX, gagaY] = getTransformedPlaceOffset(obj);

        obj.x = Math.floor(mx / 30) * 30 + 15 - gagaX;
        obj.y = Math.floor(my / 30) * 30 + 15 - gagaY;

        // obj.x_scale_exp = 0;
        // obj.x_angle = 0;
        // obj.y_scale_exp = 0;
        // obj.y_angle = 18;

        resetPreviewColor(state, obj.id);
        if (COLOR_TRIGGERS.includes(obj.id)) {
            chooseRandomTriggerColor(state, obj.id);
        } else {
            // chooseDefaultColor();
        }
        // $menuZLayer = wasm.ZLayer.B2;
        // $menuZOrder = 0;
        // $menuSelectedSFX = 0;
        // $menuSelectedSong = 0;
        // $menuSpeed = 0;

        if (obj.id == SFX_TRIGGER) {
            $menuSelectedSFX = Math.floor(
                Math.random() * SFX_TRIGGER_SOUNDS.length
            );
            playSound({
                url: SFX_SOUNDS[SFX_TRIGGER_SOUNDS[$menuSelectedSFX]],
                exclusiveChannel: "preview sfx",
                speed: semitonesToFactor($menuSpeed),
            });
        }

        if (obj.id == SONG_TRIGGER) {
            $menuSelectedSong = Math.floor(
                Math.random() * SONG_TRIGGER_SONGS.length
            );
            stopSound("song");
            playSound({
                url: SONG_SOUNDS[SONG_TRIGGER_SONGS[$menuSelectedSong]],
                exclusiveChannel: "preview song",
                speed: semitonesToFactor($menuSpeed),
                endCb: () => {
                    songPlaying.set(false);
                },
                loadCb: () => {
                    songPlaying.set(true);
                    songPlayingIsPreview.set(true);
                },
            });
        } else {
            stopSound("preview song");
            if ($songPlayingIsPreview) songPlaying.set(false);
        }

        if (setCheckedPreviewObject(state, obj)) {
            state.set_preview_visibility(true);
        }
    };

    let selectDepth = 0;
    const trySelectAt = (mx: number, my: number, hit: wasm.HitObjectInfo[]) => {
        if ($eventStatus != "during") {
            return;
        }

        if (hit.length == 0) {
            $selectedObject = null;
            state.deselect_object();
            return;
        }
        if (selectDepth >= hit.length) {
            selectDepth = 0;
        }

        let selected = hit[selectDepth];
        selectDepth += 1;

        state.set_selected_object(selected.key());

        $selectedObject = {
            id: selected.obj.id,
            mainColor: selected.obj.main_color,
            detailColor: selected.obj.detail_color,
            namePlaced: null,
            signupDate: null,
            zLayer: selected.obj.z_layer,
            zOrder: selected.obj.z_order,
            posX: selected.obj.x,
            posY: selected.obj.y,
        };
        getPlacedUsername(selected.key(), v => {
            if ($selectedObject != null) {
                $selectedObject.namePlaced = v;

                if ($loginData?.currentUserData?.userDetails?.moderator) {
                    getSignupTimestamp(v.toLowerCase(), d => {
                        if (d != undefined && $selectedObject != null) {
                            $selectedObject.signupDate = d;
                        }
                    });
                }
            }
        });
    };

    const tryRunTriggers = (hit: wasm.HitObjectInfo[]): boolean => {
        let triggersRun = false;
        let audio_hits_count = hit.filter(
            i => i.obj.id == SFX_TRIGGER || i.obj.id == SONG_TRIGGER
        ).length;
        let audio_hit_idx = 0;
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
                    // let rand =
                    //     Math.sin(Math.sin(audio_hit_idx * 6.97) * 6.97) / 2 + 1;

                    playSound({
                        url: SFX_SOUNDS[SFX_TRIGGER_SOUNDS[i.obj.main_color.r]],
                        volume: 1.0 / Math.sqrt(audio_hits_count),
                        speed: semitonesToFactor(i.obj.main_color.g - 12),
                    });
                    audio_hit_idx += 1;
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
                case SONG_TRIGGER: {
                    // let rand =
                    //     Math.sin(Math.sin(audio_hit_idx * 6.97) * 6.97) / 2 + 1;

                    stopSound("preview song");
                    playSound({
                        url: SONG_SOUNDS[
                            SONG_TRIGGER_SONGS[i.obj.main_color.r]
                        ],
                        volume: 1.0 / Math.sqrt(audio_hits_count),
                        speed: semitonesToFactor(i.obj.main_color.g - 12),
                        exclusiveChannel: "song", // because honestly 2 songs should never play on top of eachother
                        endCb: () => {
                            songPlaying.set(false);
                        },
                        loadCb: () => {
                            songPlaying.set(true);
                            songPlayingIsPreview.set(false);
                        },
                    });

                    audio_hit_idx += 1;
                    triggersRun = true;
                    addTriggerRun(i.obj.x, i.obj.y);
                    break;
                }
            }
        }
        return triggersRun;
    };

    onMount(() => {
        if ($eventStatus != "fully done") {
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
        } else {
            $editorData.x = LEVEL_WIDTH_UNITS / 2;
            $editorData.y = LEVEL_HEIGHT_UNITS / 2;
            $editorData.zoom = -2;
        }

        $zoomGoal = $editorData.zoom;
        state.set_zoom($editorData.zoom);
        state.set_camera_pos($editorData.x, $editorData.y);

        handleSub(state);
    });

    const handleMouseUp = (isTouch: boolean) => {
        if (!dragging && !panzooming) return;

        // if (
        //     !(dragging?.thresholdReached ?? true) ||
        //     !(panzooming?.thresholdReached ?? true)
        // ) {
        //     let [mx, my] = getWorldMousePos();
        //     let hit = state.objects_hit_at(mx, my, 0.0);

        //     if ($menuTabGroup == TabGroup.Delete && $canPlaceEditDelete) {
        //         trySelectAt(mx, my, hit);

        //         if ($isGuideActive) {
        //             $walmart.hasDeleteSelection =
        //                 state.get_selected_object_key() != undefined &&
        //                 $isGuideActive;
        //         }
        //     } else {
        //         if (isTouch) {
        //             checkHover();
        //         }
        //         if (
        //             !tryRunTriggers(hit) &&
        //             !(isTouch && hit.length > 0) &&
        //             $canPlaceEditDelete
        //         ) {
        //             placePreview(mx, my);

        //             if ($isGuideActive) {
        //                 $walmart.hasPlacedObject = state.is_preview_visible();
        //             }
        //         }
        //     }
        // }

        dragging = null;
        panzooming = null;
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

    const makeTouchList = (l: TouchList): { x: number; y: number }[] => {
        let out = [];
        for (let i = 0; i < l.length; i++) {
            out.push({
                x: l[i].clientX * window.devicePixelRatio,
                y: l[i].clientY * window.devicePixelRatio,
            });
        }
        return out;
    };

    const startPanzoom = (
        touches: { x: number; y: number }[],
        thresholdReached: boolean = false
    ) => {
        let [prevX, prevY] = state.get_camera_pos();

        panzooming = {
            prevCameraX: prevX,
            prevCameraY: prevY,
            prevTouches: touches,
            prevFrameTouches: touches,

            thresholdReached,
            prevZoom: state.get_zoom(),
            prevZoomScale: state.get_zoom_scale(),
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

    const handlePanzoom = (touches: { x: number; y: number }[]) => {
        let numPrevTouch = panzooming?.prevTouches.length ?? 0;
        let numTouch = touches.length;

        if (numPrevTouch != numTouch) {
            startPanzoom(touches);
            return;
        }

        switch (numTouch) {
            case 1: {
                let [camX, camY] = state.get_camera_pos();
                let touch = touches[0];

                if (panzooming!.thresholdReached) {
                    let z = state.get_zoom_scale();
                    let prevFrameTouch = panzooming!.prevFrameTouches[0];

                    moveCamera(
                        state,
                        (prevFrameTouch.x - touch.x) / z + camX,
                        (-prevFrameTouch.y + touch.y) / z + camY
                    );
                } else {
                    let prevTouch = panzooming!.prevTouches[0];
                    if (
                        Math.hypot(
                            touch.x - prevTouch.x,
                            touch.y - prevTouch.y
                        ) > 30.0
                    ) {
                        panzooming!.thresholdReached = true;
                        panzooming!.prevTouches[0] = touch;
                    }
                }

                break;
            }
            case 2: {
                let touch1 = touches[0];
                let touch2 = touches[1];

                let [camX, camY] = state.get_camera_pos();

                // PAN
                let prevFrameTouch1 = panzooming!.prevFrameTouches[0];
                let prevFrameTouch2 = panzooming!.prevFrameTouches[1];

                let midX = (touch1.x + touch2.x) / 2;
                let midY = (touch1.y + touch2.y) / 2;

                let prevMidX = (prevFrameTouch1.x + prevFrameTouch2.x) / 2;
                let prevMidY = (prevFrameTouch1.y + prevFrameTouch2.y) / 2;

                let z = state.get_zoom_scale();
                let newCamX = camX - (midX - prevMidX) / z;
                let newCamY = camY - (-midY + prevMidY) / z;

                moveCamera(state, newCamX, newCamY);

                // ZOOM
                let prevTouch1 = panzooming!.prevTouches[0];
                let prevTouch2 = panzooming!.prevTouches[1];
                let dist = Math.hypot(touch1.x - touch2.x, touch1.y - touch2.y);

                let prevdist = Math.hypot(
                    prevTouch1.x - prevTouch2.x,
                    prevTouch1.y - prevTouch2.y
                );

                let zoomFactor = dist / prevdist;
                let newZoom =
                    panzooming!.prevZoom +
                    (Math.log(zoomFactor) / Math.log(2)) * 12;

                $zoomCenterX = midX;
                $zoomCenterY = midY;

                zoomGoal.set(newZoom);
                zoomTween.set(newZoom, { duration: 0 });

                break;
            }
        }

        panzooming!.prevFrameTouches = touches;
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
    }, 50);
    //

    //

    let previewObjectPos: [number, number] = [0, 0];
    let editWidgetScale = 1;
    let placedNameScale = 1;
    let editWidgetVisible = false;

    const loopFn = () => {
        let obj = state.get_preview_object();

        previewObjectPos = [obj.x, obj.y];

        editWidgetScale = 1 + state.get_zoom() / 80;
        editWidgetVisible = state.is_preview_visible();

        placedNameScale = Math.pow(2.0, state.get_zoom() / 30.0);

        state.set_now(getServerNow());

        loop = requestAnimationFrame(loopFn);
    };

    let loop = requestAnimationFrame(loopFn);

    onDestroy(() => {
        clearInterval(interval);
        cancelAnimationFrame(loop);
    });

    $: {
        state.set_show_collidable($editorSettings.showCollidable);
        state.set_hide_triggers($editorSettings.hideTriggers);
        state.set_no_rotating_objects($editorSettings.noRotatingObjects);
        state.set_hide_grid($editorSettings.hideGrid);
        state.set_hide_ground($editorSettings.hideGround);
        state.set_hide_outline($editorSettings.hideOutline);
        state.set_select_hazards($editorSettings.selectDangerous);
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
        state.set_event_start($eventStartTime);
    }
    $: {
        state.set_event_end($eventEndTime);
    }
    $: {
        state.set_ending_fully_done($eventEndTime + $setNameSeconds * 1000);
    }
    $: {
        if ($eventStatus == "name set" || $eventStatus == "fully done") {
            dragging = null;
            panzooming = null;
            resetPreviewColor(state, 1);
            $selectedObject = null;
            state.deselect_object();
            stopSound("preview song");
            stopSound("song");
            isGuideActive.set(false);
            toast.pop();
            toast.pop({ target: "announcement" });
            editorSettings.set({
                ...DEFAULT_SETTINGS,
                quality: $editorSettings.quality,
            });
            state.set_preview_visibility(false);
        }
    }

    let prevTime = 0;
    const cinematicLoop = (time: number) => {
        const delta = time - prevTime;
        if (cinematic) {
            //set zoom center to screen center
            const w = canvas.offsetWidth * window.devicePixelRatio;
            const h = canvas.offsetHeight * window.devicePixelRatio;
            $zoomCenterX = w / 2;
            $zoomCenterY = h / 2;

            $zoomTween = $zoomTween + zoomVel * delta;
            let z = state.get_zoom_scale();
            let cameraVel = [
                (($mouseX - w / 2) * 15.0) / w / z,
                (($mouseY - h / 2) * 15.0) / h / z,
            ];
            const camPos = state.get_camera_pos();

            moveCamera(
                state,
                camPos[0] + cameraVel[0],
                camPos[1] - cameraVel[1]
            );
        }
        requestAnimationFrame(cinematicLoop);
    };

    requestAnimationFrame(cinematicLoop);

    const key_positions = [
        // { x: 12136, y: 12543, zoom: 0.07 },
        // { x: 12136, y: 12543, zoom: 0.07 },
        // { x: 771, y: 432, zoom: 1.6970562934875488 },
        // { x: 612, y: 23548, zoom: 1.7979686260223389 },
        // { x: 22786, y: 23068, zoom: 1.0090757608413696 },
        // {
        //     x: 22514,
        //     y: 22191,
        //     zoom: 2.8540971279144287,
        // },
        // {
        //     x: 22073,
        //     y: 22795,
        //     zoom: 3.023810625076294,
        // },
        // {
        //     x: 22759,
        //     y: 23180,
        //     zoom: 3.203615665435791,
        // },
        // {
        //     x: 22222,
        //     y: 20395,
        //     zoom: 1.0690785646438599,
        // },
        // { x: 22972, y: 610, zoom: 1.3469544649124146 },

        {
            x: 1615,
            y: 17053,
            zoom: 0.7,
        },
        {
            x: 1615,
            y: 17053,
            zoom: 2.5427117347717285,
        },
        {
            x: 22614,
            y: 17844,
            zoom: 1.3469544649124146,
        },
        {
            x: 13080,
            y: 1437,
            zoom: 2.4000000953674316,
        },
        {
            x: 12365,
            y: 23517,
            zoom: 1.511905312538147,
        },
    ];

    const transitionTime = 3000;

    let last_key_time = 0;
    let key_index = 0;

    function scale_to_zoom(s: number): number {
        let size_zoom = Math.max(
            state.width() / 1600.0,
            state.height() / 900.0
        );
        // scale = 2.0f32.powf(zoom / 12.0)
        return Math.log2(s / size_zoom) * 12.0;
    }

    function get_camera_rect(
        x: number,
        y: number,
        scale: number
    ): { ax: number; ay: number; bx: number; by: number } {
        // a: top left corner, b: bottom right corner
        // scale 1 = 1600x900 units

        return {
            ax: x - state.width() / 2 / scale,
            ay: y - state.height() / 2 / scale,

            bx: x + state.width() / 2 / scale,
            by: y + state.height() / 2 / scale,
        };
    }

    function rect_to_cam(
        ax: number,
        ay: number,
        bx: number,
        by: number
    ): { x: number; y: number; scale: number } {
        // a: top left corner, b: bottom right corner
        // scale 1 = 1600x900 units

        return {
            x: (ax + bx) / 2,
            y: (ay + by) / 2,
            scale: state.width() / Math.abs(bx - ax),
        };
    }

    function easeInOutExpo(x: number): number {
        return x === 0
            ? 0
            : x === 1
              ? 1
              : x < 0.5
                ? Math.pow(2, 20 * x - 10) / 2
                : (2 - Math.pow(2, -20 * x + 10)) / 2;
    }

    function easeOutExpo(x: number): number {
        return x === 1 ? 1 : 1 - Math.pow(2, -10 * x);
    }
    function easeInExpo(x: number): number {
        return x === 0 ? 0 : Math.pow(2, 10 * x - 10);
    }
    function easeInOutCubic(x: number): number {
        return x < 0.5 ? 4 * x * x * x : 1 - Math.pow(-2 * x + 2, 3) / 2;
    }

    const keyLoop = (delta: number) => {
        const progress = Math.min(
            (Date.now() - last_key_time) / transitionTime,
            1
        );
        const easedProgress = easeInOutCubic(progress);
        if (progress < 1) {
            const a = key_positions[key_index - 1];
            const b = key_positions[key_index];
            const a_rect = get_camera_rect(a.x, a.y, a.zoom);
            const b_rect = get_camera_rect(b.x, b.y, b.zoom);

            const point_a = {
                x: lerp(a_rect.ax, b_rect.ax, easedProgress),
                y: lerp(a_rect.ay, b_rect.ay, easedProgress),
            };

            const point_b = {
                x: lerp(a_rect.bx, b_rect.bx, easedProgress),
                y: lerp(a_rect.by, b_rect.by, easedProgress),
            };

            const cam = rect_to_cam(point_a.x, point_a.y, point_b.x, point_b.y);

            state.set_camera_pos(cam.x, cam.y);

            const jump = 1 - Math.pow(2 * easedProgress - 1, 2);
            const scale_fac = 1 + jump * 0.7;
            //console.log(cam.scale);
            state.set_zoom(scale_to_zoom(cam.scale));
        }

        requestAnimationFrame(keyLoop);
    };

    requestAnimationFrame(keyLoop);
</script>

<!-- `pointer...` for mobile + desktop, `mouse...` for desktop -->
<svelte:window
    on:pointerup={e => {
        setMousePos(e);
        const isTouch = e.pointerType == "touch";
        handleMouseUp(isTouch);
    }}
    on:touchend={e => {
        isFocused = false;
        panzooming = null;
    }}
    on:pointermove={e => {
        if (cinematic) {
            setMousePos(e);
            return;
        }
        if (dragging == null && !isGestureTarget(e)) {
            return;
        }

        const isTouch = e.pointerType == "touch";
        if (!isTouch) {
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
    on:touchmove={e => {
        if (!isGestureTarget(e)) {
            return;
        }

        const touches = makeTouchList(e.touches);

        handlePanzoom(touches);

        if (panzooming == null) {
            checkHover();
        } else {
            placedByHover.set(null);
            checkHover.cancel();
        }
    }}
    on:resize={() => {
        handleSub(state);
    }}
    on:keydown={e => {
        console.log(e.code);

        if (e.code == "Space") {
            e.preventDefault();
            togglePause();
            return;
        }

        if (e.shiftKey && e.code == "ArrowLeft") {
            e.preventDefault();
            skipForward(-100);
            return;
        }

        if (e.shiftKey && e.code == "ArrowRight") {
            e.preventDefault();
            skipForward(100);
            return;
        }

        if (e.code == "ArrowUp") {
            e.preventDefault();
            zoomVel += 0.00000045;
            return;
        }

        if (e.code == "ArrowDown") {
            e.preventDefault();
            zoomVel -= 0.00000045;
            return;
        }

        if (e.code == "ArrowLeft") {
            e.preventDefault();
            skipForward(-1000);
            return;
        }

        if (e.code == "ArrowRight") {
            e.preventDefault();
            skipForward(1000);
            return;
        }

        if (e.key == "c") {
            e.preventDefault();
            cinematic = !cinematic;
            if (!cinematic) {
                zoomVel = 0;
            }
            return;
        }

        if (e.key == "g") {
            e.preventDefault();
            addDebugTimeOffset(-1200);
            return;
        }

        if (e.key == "y") {
            e.preventDefault();
            addDebugTimeOffset(-1200);
            let total_offset = 0;
            for (let i = 0; i < 100; i++) {
                total_offset += Math.pow(0.7, i) * 500 + 50;
                setTimeout(() => {
                    addDebugTimeOffset(-1200);
                }, total_offset);
            }
            return;
        }

        if (e.key == "h") {
            e.preventDefault();
            addDebugTimeOffset(1200);
            return;
        }

        if (e.key == "f") {
            e.preventDefault();
            addDebugTimeOffset(-3600 * 10);
            return;
        }

        if (e.key == "j") {
            e.preventDefault();
            addDebugTimeOffset(3600 * 10);
            return;
        }
        if (e.key == ",") {
            e.preventDefault();
            addDebugTimeOffset(-60);
            return;
        }

        if (e.key == ".") {
            e.preventDefault();
            addDebugTimeOffset(60);
            return;
        }

        if (e.key == "q") {
            e.preventDefault();
            // capture
            console.log({
                x: $editorData.x,
                y: $editorData.y,
                zoom: state.get_zoom_scale(),
            });
            return;
        }

        if (e.key == "k") {
            e.preventDefault();
            key_index += 1;
            last_key_time = Date.now();
            return;
        }
        if (e.key == "s") {
            e.preventDefault();
            state.flip_smiley();
            return;
        }
    }}
/>

{#if $eventStatus == "before" || $eventStatus == "during" || $viewingLevelAfterEvent}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
    <div
        class="absolute w-full h-full touch-none"
        id={GESTURE_TARGET_ID}
        tabindex="0"
        on:focus={() => (isFocused = true)}
        on:blur={() => (isFocused = false)}
        on:touchstart={e => {
            isFocused = true;

            setMousePos(e.touches[0]);
            const touches = makeTouchList(e.touches);
            startPanzoom(touches);
        }}
        on:pointerdown={e => {
            setMousePos(e);
            const isTouch = e.pointerType == "touch";

            if (e.button == 0 && !isTouch) {
                startDrag(
                    e.clientX * window.devicePixelRatio,
                    e.clientY * window.devicePixelRatio
                );
            }
        }}
        on:wheel={e => {
            setZoomCenter(e.clientX, e.clientY);
            e.preventDefault();
            $zoomGoal = $zoomGoal - e.deltaY / Math.max(Math.abs(e.deltaY), 1);
        }}
    />

    <!-- use:pinch
    on:pinch={e => {
        dragging = null;
        $mouseX = e.detail.center.x * window.devicePixelRatio;
        $mouseY = e.detail.center.y * window.devicePixelRatio;

        if (pinchZooming == null) {
            pinchZooming = $zoomTween;
        } else {
            $zoomGoal =
                pinchZooming + (Math.log(e.detail.scale) / Math.log(2)) * 6;
        }
    }}
    on:pinchup={() => {
        pinchZooming = null;
    }} -->

    <div class="absolute w-full h-full overflow-visible pointer-events-none">
        <!-- {#if editWidgetVisible}
            <LevelWidget
                {state}
                x={previewObjectPos[0]}
                y={previewObjectPos[1]}
                scale={editWidgetScale}
                scaleWithZoom={false}
            >
                {#if $menuOpenWidget == WidgetType.Rotate}
                    <Rotate bind:state />
                {:else if $menuOpenWidget == WidgetType.Scale}
                    <Scale bind:state />
                {:else if $menuOpenWidget == WidgetType.Warp}
                    <Warp bind:state widgetScale={editWidgetScale} />
                {/if}
            </LevelWidget>
        {/if}
        {#if $editorSettings.showDeleteText}
            <DeleteTexts {state} />
        {/if}

        <TriggerRuns {state} /> -->

        {#if $placedByHover != null && $editorSettings.showPlacedText && $menuOpenWidget == WidgetType.None}
            <LevelWidget
                {state}
                x={$placedByHover.x}
                y={$placedByHover.y + 15}
                scaleWithZoom={false}
                scale={placedNameScale}
            >
                <PlacedByText username={$placedByHover.username} />
            </LevelWidget>
        {/if}

        <LevelWidget {state} x={-55} y={40} scale={0.15}>
            <ClosableWindow
                name="playerStartHelp"
                open={$eventStatus == "during" || $eventStatus == "before"}
            >
                <Image src={player_start_help} />
            </ClosableWindow>
        </LevelWidget>
        <LevelWidget {state} x={-90} y={200} scale={0.2}>
            <ClosableWindow
                name="playerGoalHelp"
                open={$eventStatus == "during" || $eventStatus == "before"}
            >
                <Image src={player_goal_help} />
            </ClosableWindow>
        </LevelWidget>

        <LevelWidget {state} x={END_POS_X} y={END_POS_Y - 1} scale={0.12}>
            <Image src={player_goal} />
        </LevelWidget>

        <!-- <LevelWidget {state} x={60} y={60}>
        <button class="p-4 bg-red">Gaga</button>
    </LevelWidget> -->
    </div>
{/if}
