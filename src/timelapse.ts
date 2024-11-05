import { writable } from "svelte/store";
import betatestdb from "./assets/betatestdb.json";
import * as wasm from "wasm-lib";
import { addObjString } from "./level_view/view_controls";

export const HIDE_UI = true;

type HistoryObject =
    | {
          time: number;
          object: string;
          objKey: string;
          username: string;
      }
    | {
          time: number;
          objKey: string;
          username: string;
      };

const HISTORY: HistoryObject[] = Object.values(betatestdb.history).sort(
    (a, b) => a.time - b.time
);

let obj_data_map: Map<string, string> = new Map();

HISTORY.forEach(h => {
    if ("object" in h) obj_data_map.set(h.objKey, h.object);
});

let prevTime = 0;

let timelapsetime = HISTORY[0].time;

let historyIndex = 0;

let paused = true;
let TIMELAPSE_SPEED = 200;
export const togglePause = () => {
    paused = !paused;
};

export const skipForward = (a: number) => {
    timelapsetime += a * TIMELAPSE_SPEED;
};

export const runtTimelapse = (time: number, state: wasm.State | null) => {
    if (!state) return;
    let delta = time - prevTime;

    if (HISTORY[historyIndex - 1]?.time > timelapsetime) {
        while (
            historyIndex > 0 &&
            HISTORY[historyIndex - 1]?.time > timelapsetime
        ) {
            historyIndex--;
            const h = HISTORY[historyIndex];
            if ("object" in h) {
                // DELETE OBJECT
                state.delete_object(h.objKey);
            } else {
                // PLACE OBJECT
                addObjString(state, h.objKey, obj_data_map.get(h.objKey)!);
            }
        }
    } else {
        while (
            historyIndex < HISTORY.length &&
            HISTORY[historyIndex].time < timelapsetime
        ) {
            const h = HISTORY[historyIndex];
            if ("object" in h) {
                // PLACE OBJECT
                addObjString(state, h.objKey, h.object);
            } else {
                // DELETE OBJECT
                state.delete_object(h.objKey);
            }
            historyIndex++;
        }
    }

    if (!paused) {
        timelapsetime += delta * TIMELAPSE_SPEED;
        //TIMELAPSE_SPEED += 0.01 * delta;
    }

    prevTime = time;
    requestAnimationFrame(t => runtTimelapse(t, state));
};

const unescape = (s: string) => {
    return JSON.parse(
        '"' + s.replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"'
    );
};
