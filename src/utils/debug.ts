import devtoolsFPS from "devtools-fps";
import StateMouse from "devtools-fps/src/state-mouse";

import { writable } from "svelte/store";

let isDebug = __DEBUG ?? false; // in development mode it is set to true by default

document.addEventListener("keydown", e => {
    if (e.key == "F" && e.ctrlKey && e.shiftKey) {
        isDebug = !isDebug;
        DEBUG.set(isDebug);
    }
});

export let DEBUG = writable(isDebug);

// const fpsCanvasWidth = 220;
// const fpsCanvasHeight = 64;
// devtoolsFPS.config({
//     width: fpsCanvasWidth,
//     height: fpsCanvasHeight,
//     bufferSize: 200,
//     style: { top: "90px", left: "0px" },
// });
// // https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio#correcting_resolution_in_a_canvas
// const fpsCanvas = devtoolsFPS.canvas;
// fpsCanvas.style.width = `${fpsCanvasWidth}px`;
// fpsCanvas.style.height = `${fpsCanvasHeight}px`;
// const scale = window.devicePixelRatio;
// fpsCanvas.width = Math.floor(fpsCanvasWidth * scale);
// fpsCanvas.height = Math.floor(fpsCanvasHeight * scale);
// fpsCanvas.getContext("2d")?.scale(scale, scale);

// let oldTop: string;
// // let oldLeft;

// DEBUG.subscribe(debug => {
//     if (debug) {
//         fpsCanvas.style.top = oldTop;
//         fpsCanvas.style.display = "block";
//         devtoolsFPS.start();
//     } else {
//         oldTop = fpsCanvas.style.top;
//         // oldLeft = fpsCanvas.style.left;
//         fpsCanvas.style.top = "-1000px";
//         fpsCanvas.style.display = "none";
//         devtoolsFPS.mouseState = new StateMouse(fpsCanvas); // bruh
//         devtoolsFPS.update();
//         devtoolsFPS.stop();
//     }
// });
