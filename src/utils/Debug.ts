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

devtoolsFPS.config({
    width: 220,
    height: 64,
    bufferSize: 200,
    style: { top: "0" },
});
DEBUG.subscribe(debug => {
    if (debug) {
        devtoolsFPS.canvas.style.visibility = "visible";
        devtoolsFPS.start();
    } else {
        devtoolsFPS.canvas.style.visibility = "hidden";
        devtoolsFPS.stop();
    }
});
