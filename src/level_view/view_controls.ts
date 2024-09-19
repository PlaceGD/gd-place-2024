import * as wasm from "wasm-lib";
import { subChunk, unsubChunk } from "../firebase/chunks";
import { decodeString } from "shared-lib/base_util";
import Toast from "../utils/toast";
import {
    addDeleteText,
    bgColor,
    DEFAULT_BG_COLOR,
    DEFAULT_GROUND_1_COLOR,
    DEFAULT_GROUND_2_COLOR,
    editorData,
    ground1Color,
    ground2Color,
    lastRunColorTrigger,
} from "../stores";
import debounce from "lodash.debounce";
import { tweened } from "svelte/motion";
import { cubicOut } from "svelte/easing";
import { writable } from "svelte/store";
import { clamp } from "shared-lib/util";

export const zoomGoal = writable(0);
export const zoomTween = tweened(0, {
    duration: 100,
    easing: cubicOut,
});
zoomGoal.subscribe(v => zoomTween.set(v));
zoomGoal.subscribe(v => {
    if (v < -4 || v > 36) {
        zoomGoal.set(clamp(v, -4, 36));
    }
});
export const [mouseX, mouseY] = [writable(0), writable(0)];
export const zoomCentral = (to: number, canvas: HTMLCanvasElement) => {
    mouseX.set((canvas.offsetWidth / 2) * window.devicePixelRatio);
    mouseY.set((canvas.offsetHeight / 2) * window.devicePixelRatio);
    zoomGoal.set(to);
    zoomTween.set(to, { duration: 0 });
};

export const handleSub = (state: wasm.State) => {
    for (let chunk of state.get_chunks_to_sub()) {
        subChunk(
            [chunk.x, chunk.y],
            data => {
                let key = data.key;
                let val = data.val();
                if (key != null) {
                    try {
                        let obj = wasm.GDObjectOpt.from_bytes(
                            decodeString(val, 126)
                        );

                        state.add_object(key, obj);
                    } catch (e: any) {
                        if (val.slice(0, 2) != "%%") {
                            console.error(
                                "(Failed in `GDObjectOpt.from_bytes`)"
                            );
                            Toast.showErrorToast(e.display());
                        }
                    }
                }
            },
            data => {
                let key = data.key;
                let val = data.val();
                if (key != null) {
                    let coords = state.delete_object(key);
                    if (coords != null) {
                        addDeleteText(val.slice(2), coords[0], coords[1]);
                    }
                }
            }
        );
    }
};

export const handleUnsub = (state: wasm.State) => {
    for (let chunk of state.get_chunks_to_unsub()) {
        unsubChunk([chunk.x, chunk.y]);
    }
};

const savePos = debounce((state: wasm.State) => {
    let zoom = Math.round(state.get_zoom());
    let [x, y] = state.get_camera_pos().map(Math.round);

    history.replaceState({}, "", `?x=${x}&y=${y}&zoom=${zoom}`);

    editorData.update(data => {
        data.x = x;
        data.y = y;
        data.zoom = zoom;
        return data;
    });
}, 200);

const RESET_COLOR_TRIGGER_MIN_COOLDOWN_SECS_HAHA = 10;
export const moveCamera = (state: wasm.State, x: number, y: number) => {
    state.set_camera_pos(x, y);

    let reset_milis = RESET_COLOR_TRIGGER_MIN_COOLDOWN_SECS_HAHA * 1000;

    lastRunColorTrigger.update(v => {
        if (
            v.bg != null &&
            Date.now() > v.bg.time + reset_milis &&
            Math.hypot(v.bg.x - x, v.bg.y - y) >= 3000
        ) {
            v.bg = null;
            bgColor.set(structuredClone(DEFAULT_BG_COLOR));
        }
        if (
            v.ground1 != null &&
            Date.now() > v.ground1.time + reset_milis &&
            Math.hypot(v.ground1.x - x, v.ground1.y - y) >= 3000
        ) {
            v.ground1 = null;
            ground1Color.set(structuredClone(DEFAULT_GROUND_1_COLOR));
        }
        if (
            v.ground2 != null &&
            Date.now() > v.ground2.time + reset_milis &&
            Math.hypot(v.ground2.x - x, v.ground2.y - y) >= 3000
        ) {
            v.ground2 = null;
            ground2Color.set(structuredClone(DEFAULT_GROUND_2_COLOR));
        }
        return v;
    });

    savePos(state);
    handleSub(state);
};

export const getCameraPos = (state: wasm.State): [number, number] => {
    const [x, y] = state.get_camera_pos();
    return [x, y];
};
