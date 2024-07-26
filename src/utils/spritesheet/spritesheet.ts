import {
    createIndexedDBStorage,
    persist,
} from "@macfja/svelte-persistent-store";
import { PNG } from "pngjs/browser";
import { writable } from "svelte/store";
import { spritesheet, type SpriteData } from "shared-lib/gd";
import Toast from "../toast";
import { clamp } from "shared-lib/util";

// export const sprites = persist(
//     writable<Record<number, Blob>>({}),
//     createIndexedDBStorage(),
//     "sprites"
// );
// let _sprites: Record<number, Blob> = {};
// sprites.subscribe(v => {
//     _sprites = v;
// });

// let spriteUrls: Record<number, string> = {};

export let BUTTON_SPRITESHEET: Spritesheet | null = null;
export const loadButtonSpritesheet = (b: ArrayBuffer) => {
    BUTTON_SPRITESHEET = new Spritesheet(b);
};

export class Spritesheet {
    sPng: PNG | null = null;
    worker: Worker = new Worker(new URL("./worker.ts", import.meta.url), {
        type: "module",
    });

    constructor(spritesheetData: ArrayBuffer) {
        new PNG().parse(spritesheetData as Buffer, (error, data) => {
            if (error) {
                Toast.showErrorToast(
                    "There was an error parsing the spritesheet."
                );
                console.error("PNG parse error:", error);
            }
            this.sPng = data;
        });
    }

    async spriteImageStringFromId(id: number): Promise<string | null> {
        return new Promise(res => {
            if (!this.sPng) {
                res(null);
            } else {
                this.worker.addEventListener("message", e => {
                    // res(e.data.blobUrl
                    res(e.data.blobUrl);
                });

                this.worker.postMessage({
                    type: "load_sprite",
                    spritesheetData: {
                        buffer: this.sPng.data,
                        width: this.sPng.width,
                    },
                    spriteId: id,
                });
            }
        });
    }
}
