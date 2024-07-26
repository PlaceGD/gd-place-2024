import { writable } from "svelte/store";
import initWasmInner from "wasm-lib";
import * as wasm from "wasm-lib";
import Toast from "./utils/toast";
import { HAS_OPT_WASM } from "./main";
import { Spritesheet } from "./utils/spritesheet/spritesheet";
import { downloadWithProgress } from "./utils/download";
import { PlaceDB } from "./utils/indexdb";

let db: PlaceDB | null = null;
try {
    db = await PlaceDB.open();
} catch (e) {
    Toast.showWarningToast("Failed to open database, falling back to no cache");
}

export const wasmProgress = writable({
    progress: -1,
    max: 0,
    hasLoaded: false,
});

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

const startWasm = (data: ArrayBuffer) => {
    initWasmInner(data)
        .then(() => {
            wasmProgress.update(v => ({ ...v, hasLoaded: true }));
        })
        .catch((e: any) => {
            console.error(e, "(failed in initWasmInner)");
            Toast.showErrorToast(`Failed to initialize WASM. (${e})`);
        });
};

export const initWasm = async () => {
    const WASM_URL = `../wasm-lib/pkg/wasm_lib_bg.wasm${
        HAS_OPT_WASM ? "-opt.wasm" : ""
    }`;

    try {
        if (db != null) {
            const wasm = await db.getWasmCache();

            if (wasm != undefined) {
                startWasm(wasm);
                return;
            }
        }
    } catch {
        // console.war
    }

    downloadWithProgress(WASM_URL, "arraybuffer", p => {
        console.info(`downloading wasm: ${p.loaded}/${p.total}`);
        wasmProgress.set({
            max: p.total,
            progress: p.loaded,
            hasLoaded: false,
        });
    })
        .then(result => {
            db?.putWasmCache(result);
            startWasm(result);
        })
        .catch(() => {
            Toast.showErrorToast(
                "Failed to download WASM. This is usually a network related issue. Please refresh and try again."
            );
        });
};

export const loadSpritesheet = () => {
    const SPRITESHEET_URL = "textures/spritesheet.png";

    downloadWithProgress(SPRITESHEET_URL, "blob", progress => {
        console.info(
            `downloading spritesheet: ${progress.loaded}/${progress.total}`
        );
        spritesheetProgress.set({
            max: progress.total,
            progress: progress.loaded,
            arrayBuffer: null,
            blobURL: null,
        });
    })
        .then(result => {
            result
                .arrayBuffer()
                .then((ab: ArrayBuffer) => {
                    Spritesheet.waitForWorkerLoad().then(() => {
                        Spritesheet.loadSheet(ab).then(() => {
                            spritesheetProgress.update(v => ({
                                ...v,
                                arrayBuffer: new Uint8Array(ab),
                                blobURL: URL.createObjectURL(result),
                            }));
                        });
                    });
                })
                .catch((e: any) => {
                    console.error(e, "(failed in `blob.arrayBuffer`)");
                    Toast.showErrorToast(`Failed to get spritesheet. (${e})`);
                });
        })
        .catch(() => {
            Toast.showErrorToast(
                "Failed to download spritesheet. This is usually a network related issue. Please refresh and try again."
            );
        });
};
