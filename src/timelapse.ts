import { writable, get } from "svelte/store";
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

// Use a writable store for timelapsetime
export const timelapsetime = writable(825379786 + 1000 * 60 * 60 * 2.6); // 1731697200074

let prevTime = 0;
let historyIndex = 0;
let paused = true;
let TIMELAPSE_SPEED = 25;

export const togglePause = () => {
    paused = !paused;
};

export const skipForward = (a: number) => {
    timelapsetime.update(current => current + a * TIMELAPSE_SPEED);
};

export const runtTimelapse = (time: number, state: wasm.State | null) => {
    if (!state) return;
    let delta = time - prevTime;

    historyIndex = state.run_history(historyIndex, get(timelapsetime));

    if (!paused) {
        timelapsetime.update(current => {
            let newtime = current + delta * TIMELAPSE_SPEED;
            let date = new Date(newtime + 1730871820288);
            console.log(date.toUTCString());
            return newtime;
        });
    }

    prevTime = time;
    requestAnimationFrame(t => runtTimelapse(t, state));
};

const unescape = (s: string) => {
    return JSON.parse(
        '"' + s.replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"'
    );
};
