import { writable } from "svelte/store";
import initWasmInner from "wasm-lib";
import Toast from "./utils/toast";
import { Spritesheet } from "./utils/spritesheet/spritesheet";
import { downloadWithProgress } from "./utils/download";
import { PlaceDB } from "./utils/indexdb";

import wasmUrl from "../wasm-lib/pkg/wasm_lib_bg.wasm?url";
import wasmVersionUrl from "../public/wasm.txt?url";
import spritesheetUrl from "../public/assets/spritesheet.png?url";

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
        .catch((e: unknown) => {
            console.error(e, "(failed in initWasmInner)");
            Toast.showErrorToast(`Failed to initialize WASM. (${e})`);
        });
};

export const initWasm = async () => {
    console.debug(wasmUrl, wasmVersionUrl);

    try {
        const newVersion = (await (await fetch(wasmVersionUrl)).text()).trim();
        const currentVersion = localStorage.getItem("wasmVersion");

        if (db != null && newVersion === currentVersion) {
            const wasm = await db.getWasmCache();

            if (wasm != undefined) {
                wasmProgress.set({
                    max: 100,
                    progress: 100,
                    hasLoaded: false,
                });

                startWasm(wasm);
                return;
            }
        } else if (newVersion !== currentVersion) {
            localStorage.setItem("wasmVersion", newVersion);
        }
    } catch {
        Toast.showWarningToast("Database is null, cache will not be used");
    }

    downloadWithProgress(wasmUrl, "arraybuffer", p => {
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

const startSpritesheet = (data: Blob) => {
    data.arrayBuffer()
        .then((ab: ArrayBuffer) => {
            Spritesheet.waitForWorkerLoad().then(() => {
                Spritesheet.loadSheet(ab).then(() => {
                    spritesheetProgress.update(v => ({
                        ...v,
                        arrayBuffer: new Uint8Array(ab),
                        blobURL: URL.createObjectURL(data),
                    }));
                });
            });
        })
        .catch((e: unknown) => {
            console.error(e, "(failed in `blob.arrayBuffer`)");
            Toast.showErrorToast(`Failed to get spritesheet. (${e})`);
        });
};

export const loadSpritesheet = async () => {
    console.debug(spritesheetUrl);

    try {
        if (db != null) {
            const spritesheet = await db.getSpritesheetCache();

            if (spritesheet != undefined) {
                spritesheetProgress.set({
                    max: 100,
                    progress: 100,
                    arrayBuffer: null,
                    blobURL: null,
                });

                startSpritesheet(spritesheet);
                return;
            }
        }
    } catch {
        Toast.showWarningToast("Database is null, cache will not be used");
    }

    downloadWithProgress(spritesheetUrl, "blob", progress => {
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
            db?.putSpritesheetCache(result);

            startSpritesheet(result);
        })
        .catch(() => {
            Toast.showErrorToast(
                "Failed to download spritesheet. This is usually a network related issue. Please refresh and try again."
            );
        });
};
