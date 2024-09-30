import spritesheetWorkerUrl from "./worker?worker&url";

export type SpritesheetData = {
    buffer: Buffer;
    width: number;
};

export type RawSpritesheetData = {
    data: Uint8Array;
    width: number;
    height: number;
};

export type Message =
    | {
          type: "loaded_sheet";
          data: RawSpritesheetData;
      }
    | {
          type: "loaded_sprite";
          blobUrl: string;
          spriteId: number;
      }
    | {
          type: "load_sheet";
          spritesheetData: ArrayBuffer;
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
    static sheetLoadRes: (data: RawSpritesheetData) => void;

    // constructor(public worker: Worker)

    static async waitForWorkerLoad(): Promise<void> {
        return new Promise(workerLoadRes => {
            Spritesheet.worker = new Worker(
                new URL(spritesheetWorkerUrl, import.meta.url),
                { type: "module" }
            );

            Spritesheet.worker.onmessage = (e: MessageEvent<Message>) => {
                if (e.data.type == "loaded_sprite") {
                    Spritesheet.promiseResList[e.data.spriteId].pop()!(
                        e.data.blobUrl
                    );
                } else if (e.data.type == "loaded_sheet") {
                    Spritesheet.sheetLoadRes(e.data.data);
                } else if (e.data.type == "worker_loaded") {
                    workerLoadRes();
                }
            };
        });
    }

    static async loadSheet(
        spritesheetData: ArrayBuffer
    ): Promise<RawSpritesheetData> {
        return new Promise(loadRes => {
            Spritesheet.sheetLoadRes = loadRes;

            Spritesheet.worker.postMessage({
                type: "load_sheet",
                spritesheetData,
            } as Message);
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
