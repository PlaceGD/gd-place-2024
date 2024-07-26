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

export class Spritesheet {
    static worker: Worker = new Worker(
        new URL("./worker.ts", import.meta.url),
        {
            type: "module",
        }
    );
    static promise_res_list: Record<number, ((v: string) => void)[]> = {};

    static async loadSheet(spritesheetData: ArrayBuffer): Promise<void> {
        return new Promise(loadRes => {
            this.worker.onmessage = e => {
                if (e.data.type == "loaded_sprite") {
                    this.promise_res_list[e.data.spriteId].pop()!(
                        e.data.blobUrl
                    );
                } else if (e.data.type == "loaded_sheet") {
                    loadRes();
                }
            };

            new PNG().parse(spritesheetData as Buffer, (error, png) => {
                if (error) {
                    Toast.showErrorToast(
                        "There was an error parsing the spritesheet."
                    );
                    console.error("PNG parse error:", error);
                }
                this.worker.postMessage({
                    type: "load_sheet",
                    spritesheetData: {
                        buffer: png.data,
                        width: png.width,
                    },
                });
            });
        });
    }

    static async spriteImageStringFromId(id: number): Promise<string | null> {
        return new Promise(res => {
            if (this.promise_res_list[id] == undefined) {
                this.promise_res_list[id] = [];
            }
            this.promise_res_list[id].push(res);

            this.worker.postMessage({
                type: "load_sprite",
                spriteId: id,
            });
        });
    }
}
