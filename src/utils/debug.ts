import devtoolsFPS from "devtools-fps";

import { writable } from "svelte/store";

let isDebug = __DEBUG ?? false; // in development mode it is set to true by default

document.addEventListener("keydown", e => {
    if (e.key == "F" && e.ctrlKey && e.shiftKey) {
        isDebug = !isDebug;
        DEBUG.set(isDebug);
    }
});

export let DEBUG = writable(isDebug);

const fpsCanvasWidth = 220;
const fpsCanvasHeight = 64;
devtoolsFPS.config({
    width: fpsCanvasWidth,
    height: fpsCanvasHeight,
    bufferSize: 200,
    style: { top: "90px", left: "0px" },
});
// https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio#correcting_resolution_in_a_canvas
const fpsCanvas = devtoolsFPS.canvas;
fpsCanvas.style.width = `${fpsCanvasWidth}px`;
fpsCanvas.style.height = `${fpsCanvasHeight}px`;
const scale = window.devicePixelRatio;
fpsCanvas.width = Math.floor(fpsCanvasWidth * scale);
fpsCanvas.height = Math.floor(fpsCanvasHeight * scale);
fpsCanvas.getContext("2d")?.scale(scale, scale);

DEBUG.subscribe(debug => {
    if (debug) {
        fpsCanvas.style.visibility = "visible";
        devtoolsFPS.start();
    } else {
        fpsCanvas.style.visibility = "hidden";
        devtoolsFPS.stop();
    }
});
