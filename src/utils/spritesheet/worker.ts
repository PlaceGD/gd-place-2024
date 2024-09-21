import { PNG } from "pngjs/browser";
import { spritesheet, type SpriteData } from "shared-lib/gd";
import { clamp } from "shared-lib/util";
import type { Message, SpritesheetData } from "./spritesheet";
import { PlaceDB } from "../indexdb";

// const spriteCache = indexedDB.open("spriteCache");

// spriteCache.onupgradeneeded = () => {
//     const db = spriteCache.result;

//     const store = db.createObjectStore("sprite", { keyPath: "id" });
// };

let db: PlaceDB | null = null;
try {
    db = await PlaceDB.open();
} catch (e) {
    console.warn(
        `Worker failed to open database, falling back to no cache (${e})`
    );
}

let spriteUrls: Record<number, string> = {};

const copyOverlay = (
    src: SpritesheetData,
    dst: PNG,
    srcX: number,
    srcY: number,
    width: number,
    height: number,
    dstX: number,
    dstY: number,
    colorMod: [number, number, number]
) => {
    const to1D = (x: number, y: number, width: number) => x + y * width;

    const srcData = src.buffer;

    for (let i = 0; i < +width; i++) {
        for (let j = 0; j < +height; j++) {
            let sx = i + srcX;
            let sy = j + srcY;
            let dx = i + dstX;
            let dy = j + dstY;

            let srcIdx = to1D(sx, sy, src.width) * 4;
            let dstIdx = to1D(dx, dy, dst.width) * 4;

            let srcA = srcData[srcIdx + 3] / 255;
            let dstA = dst.data[dstIdx + 3] / 255;

            let finalA = clamp(dstA * (1 - srcA) + srcA, 0, 1);
            for (let i = 0; i < 3; i++) {
                dst.data[dstIdx + i] =
                    dst.data[dstIdx + i] * dstA * (1 - srcA) +
                    ((srcData[srcIdx + i] * colorMod[i]) / 255) * srcA;
                if (finalA > 0) {
                    dst.data[dstIdx + i] /= finalA;
                }
            }
            dst.data[dstIdx + 3] = finalA * 255;
        }
    }
};

const bitBlt = (
    src: SpritesheetData,
    dst: PNG,
    srcX: number,
    srcY: number,
    width: number,
    height: number,
    deltaX: number,
    deltaY: number
) => {
    for (let y = 0; y < height; y++) {
        dst.data.set(
            src.buffer.slice(
                ((srcY + y) * src.width + srcX) << 2,
                ((srcY + y) * src.width + srcX + width) << 2
            ),
            ((deltaY + y) * dst.width + deltaX) << 2
        );
    }
};

const loadSprite = async (
    spritesheetData: SpritesheetData,
    id: number
): Promise<string | null> => {
    if (spriteUrls[id] != undefined) {
        return spriteUrls[id];
    }

    try {
        if (db != null) {
            let blob = await db.getSpriteCache(id);
            if (blob != null) {
                const url = URL.createObjectURL(blob);
                spriteUrls[id] = url;

                return url;
            }
        }
    } catch {}

    let mSprite = spritesheet.main_sprites[id] as SpriteData | null;
    let dSprite = spritesheet.detail_sprites[id] as SpriteData | null;

    const dest = new PNG({
        width:
            Math.max(
                (mSprite == null ? 0 : mSprite.size[0]) +
                    Math.abs(mSprite == null ? 0 : mSprite.offset[0]) * 2,
                (dSprite == null ? 0 : dSprite.size[0]) +
                    Math.abs(dSprite == null ? 0 : dSprite.offset[0]) * 2
            ) + 2,
        height:
            Math.max(
                (mSprite == null ? 0 : mSprite.size[1]) +
                    Math.abs(mSprite == null ? 0 : mSprite.offset[1]) * 2,
                (dSprite == null ? 0 : dSprite.size[1]) +
                    Math.abs(dSprite == null ? 0 : dSprite.offset[1]) * 2
            ) + 2,
    });

    if (mSprite != null) {
        const mTLX = (dest.width - mSprite.size[0]) / 2 + mSprite.offset[0];
        const mTLY = (dest.height - mSprite.size[1]) / 2 + mSprite.offset[1];

        bitBlt(
            spritesheetData,
            dest,
            spritesheet.main_sprites[id].pos[0],
            spritesheet.main_sprites[id].pos[1],
            mSprite.size[0],
            mSprite.size[1],
            mTLX,
            mTLY
        );
    }
    if (dSprite != null) {
        const dTLX = (dest.width - dSprite.size[0]) / 2 + dSprite.offset[0];
        const dTLY = (dest.height - dSprite.size[1]) / 2 + dSprite.offset[1];
        copyOverlay(
            spritesheetData,
            dest,
            spritesheet.detail_sprites[id].pos[0],
            spritesheet.detail_sprites[id].pos[1],
            dSprite.size[0],
            dSprite.size[1],
            dTLX,
            dTLY,
            [178, 178, 255]
        );
    }

    let newPngBuffer = PNG.sync.write(dest);
    let blob = new Blob([newPngBuffer], { type: "image/png" });

    try {
        if (db != null) {
            await db.putSpriteCache(id, blob);
        }
    } catch {}

    const url = URL.createObjectURL(blob);
    spriteUrls[id] = url;

    return url;
};

let spritesheetData: SpritesheetData;

postMessage({ type: "worker_loaded" } as Message);

onmessage = (ev: MessageEvent<Message>) => {
    if (ev.data.type === "load_sprite") {
        let id = ev.data.spriteId;
        loadSprite(spritesheetData, id).then(blobUrl => {
            postMessage({
                type: "loaded_sprite",
                blobUrl,
                spriteId: id,
            } as Message);
        });
    } else if (ev.data.type === "load_sheet") {
        new PNG().parse(ev.data.spritesheetData as Buffer, (error, png) => {
            if (error) {
                console.error("PNG parse error:", error);
            } else {
                spritesheetData = {
                    buffer: png.data,
                    width: png.width,
                };

                postMessage({
                    type: "loaded_sheet",
                    data: {
                        width: png.width,
                        height: png.height,
                        data: new Uint8Array(png.data.buffer),
                    },
                } as Message);
            }
        });
    }
};
