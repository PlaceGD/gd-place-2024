import { writable } from "svelte/store";
import initWasmInner from "wasm-lib";
import Toast from "./utils/toast";
import {
    Spritesheet,
    type RawSpritesheetData,
} from "./utils/spritesheet/spritesheet";
import { downloadWithProgress } from "./utils/download";
import { PlaceDB } from "./utils/indexdb";

import wasmUrl from "../rust/wasm-lib/pkg/wasm_lib_bg.wasm?url";
import spritesheetUrl from "./assets/spritesheet.png?url";
import wasmVersionUrl from "./assets/wasm.ver?url";
import spritesheetVersionUrl from "./assets/spritesheet.ver?url";

let db: PlaceDB | null = null;
try {
    if (typeof window !== "undefined") {
        db = await PlaceDB.open();
    }
} catch {
    Toast.showWarningToast("Failed to open database, falling back to no cache");
}

export const wasmProgress = writable({
    progress: 0,
    // max: 0,
    hasLoaded: false,
});

export const spritesheetProgress = writable<{
    progress: number;
    // max: number;
    arrayBuffer: Uint8Array | null;
    blobURL: string | null;
}>({
    progress: 0,
    // max: 0,
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
    const currVer = await (await fetch(wasmVersionUrl)).text();

    try {
        const prevVer = localStorage.getItem("wasmVer");

        if (db != null && prevVer === currVer) {
            const wasm = await db.getWasmCache();

            if (wasm != undefined) {
                wasmProgress.set({
                    // max: 100,
                    progress: 1,
                    hasLoaded: false,
                });

                startWasm(wasm);
                return;
            }
        }
    } catch {
        Toast.showWarningToast("Database is null, cache will not be used");
    }

    downloadWithProgress(wasmUrl, "arraybuffer", p => {
        console.info(`downloading wasm: ${p.loaded}/${p.total}`);
        wasmProgress.set({
            // max: p.total,
            progress: p.loaded / p.total,
            hasLoaded: false,
        });
    })
        .then(result => {
            localStorage.setItem("wasmVer", currVer);

            db?.putWasmCache(result);
            startWasm(result);
        })
        .catch(() => {
            Toast.showErrorToast(
                "Failed to download WASM. This is usually a network related issue. Please refresh and try again."
            );
        });
};

const startSpritesheet = (data: Blob): Promise<RawSpritesheetData> => {
    return new Promise(res => {
        data.arrayBuffer()
            .then((ab: ArrayBuffer) => {
                Spritesheet.waitForWorkerLoad().then(() => {
                    Spritesheet.loadSheet(ab).then(rawData => {
                        spritesheetProgress.update(v => ({
                            ...v,
                            arrayBuffer: new Uint8Array(ab),
                            blobURL: URL.createObjectURL(data),
                        }));

                        res(rawData);
                    });
                });
            })
            .catch((e: unknown) => {
                console.error(e, "(failed in `blob.arrayBuffer`)");
                Toast.showErrorToast(`Failed to get spritesheet. (${e})`);
            });
    });
};

export const fetchAndParseSpritesheet =
    async (): Promise<RawSpritesheetData> => {
        const currVer = await (await fetch(spritesheetVersionUrl)).text();

        return new Promise(async res => {
            try {
                const prevVer = localStorage.getItem("spritesheetVer");

                if (db != null && currVer === prevVer) {
                    const spritesheet = await db.getSpritesheetCache();

                    if (spritesheet != undefined) {
                        spritesheetProgress.set({
                            // max: 100,
                            progress: 1,
                            arrayBuffer: null,
                            blobURL: null,
                        });

                        let data = startSpritesheet(spritesheet);
                        res(data);

                        return;
                    }
                }
            } catch {
                Toast.showWarningToast(
                    "Database is null, cache will not be used"
                );
            }

            downloadWithProgress(spritesheetUrl, "blob", progress => {
                console.info(
                    `downloading spritesheet: ${progress.loaded}/${progress.total}`
                );
                spritesheetProgress.set({
                    // max: progress.total,
                    progress: progress.loaded / progress.total,
                    arrayBuffer: null,
                    blobURL: null,
                });
            })
                .then(result => {
                    localStorage.setItem("spritesheetVer", currVer);

                    db?.putSpritesheetCache(result);

                    let data = startSpritesheet(result);
                    res(data);
                })
                .catch(() => {
                    Toast.showErrorToast(
                        "Failed to download spritesheet. This is usually a network related issue. Please refresh and try again."
                    );
                });
        });
    };
