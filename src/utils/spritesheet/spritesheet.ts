import {
    createIndexedDBStorage,
    persist,
} from "@macfja/svelte-persistent-store";
import { PNG } from "pngjs/browser";
import { writable } from "svelte/store";
import { spritesheet, type SpriteData } from "shared-lib/gd";
import Toast from "../toast";
import { clamp } from "shared-lib/util";

export type SpritesheetData = {
    buffer: Uint8Array;
    width: number;
};

export type Message =
    | {
          type: "loaded_sheet";
      }
    | {
          type: "loaded_sprite";
          blobUrl: string;
          spriteId: number;
      }
    | {
          type: "load_sheet";
          spritesheetData: SpritesheetData;
      }
    | {
          type: "load_sprite";
          spriteId: number;
      }
    | {
          type: "worker_loaded";
      };

export class Spritesheet {
    static worker: Worker;
    static promiseResList: Record<number, ((v: string) => void)[]> = {};
    static sheetLoadRes: () => void;

    // constructor(public worker: Worker)

    static async waitForWorkerLoad(): Promise<void> {
        return new Promise(workerLoadRes => {
            Spritesheet.worker = new Worker(
                new URL("./worker.ts", import.meta.url),
                {
                    type: "module",
                }
            );
            Spritesheet.worker.onmessage = (e: MessageEvent<Message>) => {
                if (e.data.type == "loaded_sprite") {
                    Spritesheet.promiseResList[e.data.spriteId].pop()!(
                        e.data.blobUrl
                    );
                } else if (e.data.type == "loaded_sheet") {
                    Spritesheet.sheetLoadRes();
                } else if (e.data.type == "worker_loaded") {
                    workerLoadRes();
                }
            };
        });
    }

    static async loadSheet(spritesheetData: ArrayBuffer): Promise<void> {
        return new Promise(loadRes => {
            Spritesheet.sheetLoadRes = loadRes;

            new PNG().parse(spritesheetData as Buffer, (error, png) => {
                if (error) {
                    Toast.showErrorToast(
                        "There was an error parsing the spritesheet."
                    );
                    console.error("PNG parse error:", error);
                }

                Spritesheet.worker.postMessage({
                    type: "load_sheet",
                    spritesheetData: {
                        buffer: png.data,
                        width: png.width,
                    },
                } as Message);
            });
        });
    }

    static async spriteImageStringFromId(id: number): Promise<string | null> {
        return new Promise(res => {
            if (Spritesheet.promiseResList[id] == undefined) {
                Spritesheet.promiseResList[id] = [];
            }
            Spritesheet.promiseResList[id].push(res);

            Spritesheet.worker.postMessage({
                type: "load_sprite",
                spriteId: id,
            } as Message);
        });
    }
}
