import { writable } from "svelte/store";
import initWasmInner, { type InitOutput } from "wasm-lib";
import * as wasm from "wasm-lib";
import Toast from "./utils/Toast";
import { HAS_OPT_WASM } from "./main";

const WASM_URL = `../wasm-lib/pkg/wasm_lib_bg.wasm${
    //HAS_OPT_WASM ? "-opt.wasm" : "" // TODO: Fix? (an't access lexical declaration 'HAS_OPT_WASM' before initialization)
    ""
}`;

export const wasmProgress = writable({
    progress: -1,
    max: 0,
    hasLoaded: false,
});

export const initWasm = () => {
    const wasmReq = new XMLHttpRequest();

    wasmReq.responseType = "arraybuffer";
    wasmReq.addEventListener("progress", p => {
        console.info(`downloading wasm: ${p.loaded}/${p.total}`);
        wasmProgress.set({
            max: p.total,
            progress: p.loaded,
            hasLoaded: false,
        });
    });
    wasmReq.addEventListener("load", () => {
        initWasmInner(wasmReq.response)
            .then(() => {
                wasmProgress.update(v => (v = { ...v, hasLoaded: true }));
            })
            .catch(e => {
                console.error(e, "(failed in initWasmInner)");
                Toast.showErrorToast(`Failed to initialize WASM. (${e})`);
            });
    });
    wasmReq.addEventListener("error", () => {
        Toast.showErrorToast(
            "Failed to download WASM. This is usually a network related issue. Please refresh and try again."
        );
    });
    wasmReq.addEventListener("timeout", () => {
        Toast.showErrorToast(
            "WASM download timed out. This is usually a network related issue. Please refresh and try again."
        );
    });
    wasmReq.addEventListener("abort", () => {
        Toast.showErrorToast(
            "Failed to download WASM. Please refresh and try again."
        );
    });

    wasmReq.open("GET", WASM_URL);
    wasmReq.send();
};
