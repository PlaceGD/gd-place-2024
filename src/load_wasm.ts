import { writable } from "svelte/store";
import initWasmInner from "wasm-lib";
import * as wasm from "wasm-lib";
import Toast from "./utils/toast";
import { HAS_OPT_WASM } from "./main";
import { loadButtonSpritesheet, Spritesheet } from "./utils/spritesheet";

export const wasmProgress = writable({
    progress: -1,
    max: 0,
    hasLoaded: false,
});

export const initWasm = () => {
    const WASM_URL = `../wasm-lib/pkg/wasm_lib_bg.wasm${
        HAS_OPT_WASM ? "-opt.wasm" : ""
    }`;

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
                wasmProgress.update(v => ({ ...v, hasLoaded: true }));
            })
            .catch((e: any) => {
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

    wasmReq.open("GET", WASM_URL);
    wasmReq.send();
};

export const spritesheetProgress = writable<{
    progress: number;
    max: number;
    arrayBuffer: Uint8Array | null;
    blobURL: string | null;
}>({
    progress: -1,
    max: 0,
    arrayBuffer: null,
    blobURL: null,
});

export const loadSpritesheet = () => {
    const SPRITESHEET_URL = "textures/spritesheet.png";

    const spritesheetReq = new XMLHttpRequest();

    spritesheetReq.responseType = "blob";
    spritesheetReq.addEventListener("progress", p => {
        console.info(`downloading spritesheet: ${p.loaded}/${p.total}`);
        spritesheetProgress.set({
            max: p.total,
            progress: p.loaded,
            arrayBuffer: null,
            blobURL: null,
        });
    });
    spritesheetReq.addEventListener("load", () => {
        const blob = spritesheetReq.response;

        blob.arrayBuffer()
            .then((ab: ArrayBuffer) => {
                loadButtonSpritesheet(ab);

                spritesheetProgress.update(v => ({
                    ...v,
                    arrayBuffer: new Uint8Array(ab),
                    blobURL: URL.createObjectURL(blob),
                }));
            })
            .catch((e: any) => {
                console.error(e, "(failed in `blob.arrayBuffer`)");
                Toast.showErrorToast(`Failed to get spritesheet. (${e})`);
            });
    });
    spritesheetReq.addEventListener("error", () => {
        Toast.showErrorToast(
            "Failed to download spritesheet. This is usually a network related issue. Please refresh and try again."
        );
    });
    spritesheetReq.addEventListener("timeout", () => {
        Toast.showErrorToast(
            "Spritesheet download timed out. This is usually a network related issue. Please refresh and try again."
        );
    });

    spritesheetReq.open("GET", SPRITESHEET_URL);
    spritesheetReq.send();
};
