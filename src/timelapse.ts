import { writable } from "svelte/store";
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

// dynamically import the json file

let timelapsetime: number = 825379786; //  + 3600000 * 7.1

// import(/* @vite-ignore */ "./assets/db2.json").then((betatestdb: any) => {
//     HISTORY = Object.values(betatestdb.default.history).sort(
//         (a: any, b: any) => a.time - b.time
//     ) as any;
//     HISTORY.forEach(h => {
//         if ("object" in h) obj_data_map.set(h.objKey, h.object);
//     });
//     timelapsetime = HISTORY[0].time;
// });

let prevTime = 0;

let historyIndex = 0;

let paused = true;
let TIMELAPSE_SPEED = 1000;
export const togglePause = () => {
    paused = !paused;
};

export const skipForward = (a: number) => {
    timelapsetime += a * TIMELAPSE_SPEED;
};

export const runtTimelapse = (time: number, state: wasm.State | null) => {
    if (!state) return;
    let delta = time - prevTime;

    historyIndex = state.run_history(historyIndex, timelapsetime);

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
