import {
    createIndexedDBStorage,
    persist,
} from "@macfja/svelte-persistent-store";
import { PNG } from "pngjs/browser";
import { writable } from "svelte/store";
import { spritesheet, type SpriteData } from "shared-lib/gd";
import Toast from "./toast";
import { clamp } from "shared-lib/util";

export const sprites = persist(
    writable<Record<number, Blob>>({}),
    createIndexedDBStorage(),
    "sprites"
);
let _sprites: Record<number, Blob> = {};
sprites.subscribe(v => {
    _sprites = v;
});

let spriteUrls: Record<number, string> = {};

const copyOverlay = (
    src: PNG,
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

    for (let i = 0; i < +width; i++) {
        for (let j = 0; j < +height; j++) {
            let sx = i + srcX;
            let sy = j + srcY;
            let dx = i + dstX;
            let dy = j + dstY;

            let srcIdx = to1D(sx, sy, src.width) * 4;
            let dstIdx = to1D(dx, dy, dst.width) * 4;

            let srcA = src.data[srcIdx + 3] / 255;
            let dstA = dst.data[dstIdx + 3] / 255;

            let finalA = clamp(dstA * (1 - srcA) + srcA, 0, 1);
            for (let i = 0; i < 3; i++) {
                dst.data[dstIdx + i] =
                    dst.data[dstIdx + i] * dstA * (1 - srcA) +
                    ((src.data[srcIdx + i] * colorMod[i]) / 255) * srcA;
                if (finalA > 0) {
                    dst.data[dstIdx + i] /= finalA;
                }
            }
            dst.data[dstIdx + 3] = finalA * 255;
        }
    }
};

export let BUTTON_SPRITESHEET: Spritesheet | null = null;
export const loadButtonSpritesheet = (b: ArrayBuffer) => {
    BUTTON_SPRITESHEET = new Spritesheet(b);
};

export class Spritesheet {
    sPng: PNG | null = null;

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

    spriteImageStringFromId(id: number): string | null {
        if (!this.sPng) return null;

        // if (spriteUrls[id] != undefined) {
        //     return spriteUrls[id];
        // }

        // if (_sprites[main] != undefined) {
        //     const url = URL.createObjectURL(_sprites[main]);
        //     spriteUrls[main] = url;

        //     return url;
        // }

        let mSprite = spritesheet.mainSprites[id] as SpriteData | null;
        let dSprite = spritesheet.detailSprites[id] as SpriteData | null;

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
            const mTLY =
                (dest.height - mSprite.size[1]) / 2 + mSprite.offset[1];

            PNG.bitblt(
                this.sPng!,
                dest,
                spritesheet.mainSprites[id].pos[0],
                spritesheet.mainSprites[id].pos[1],
                mSprite.size[0],
                mSprite.size[1],
                mTLX,
                mTLY
            );
        }
        if (dSprite != null) {
            const dTLX = (dest.width - dSprite.size[0]) / 2 + dSprite.offset[0];
            const dTLY =
                (dest.height - dSprite.size[1]) / 2 + dSprite.offset[1];
            copyOverlay(
                this.sPng!,
                dest,
                spritesheet.detailSprites[id].pos[0],
                spritesheet.detailSprites[id].pos[1],
                dSprite.size[0],
                dSprite.size[1],
                dTLX,
                dTLY,
                [178, 178, 255]
            );
        }

        let newPngBuffer = PNG.sync.write(dest);
        let blob = new Blob([newPngBuffer], { type: "image/png" });

        // sprites.update(s => {
        //     s[id] = blob;
        //     return s;
        // });

        const url = URL.createObjectURL(blob);
        spriteUrls[id] = url;

        return url;
    }
}

/*


import { PNG } from "pngjs/browser";


    

    $: {
        if ($spritesheetProgress.arrayBuffer != null) {
            const mSprite = spritesheet.mainSprites[939];
            const dSprite = spritesheet.detailSprites[939];

            const dest = new PNG({
                width:
                    Math.max(
                        mSprite.size[0] + Math.abs(mSprite.offset[0]) * 2,
                        dSprite.size[0] + Math.abs(dSprite.offset[0]) * 2
                    ) + 2,
                height:
                    Math.max(
                        mSprite.size[1] + Math.abs(mSprite.offset[1]) * 2,
                        dSprite.size[1] + Math.abs(dSprite.offset[1]) * 2
                    ) + 2,
            });

            const mTLX = (dest.width - mSprite.size[0]) / 2 + mSprite.offset[0];
            const mTLY =
                (dest.height - mSprite.size[1]) / 2 + mSprite.offset[1];
            const dTLX = (dest.width - dSprite.size[0]) / 2 + dSprite.offset[0];
            const dTLY =
                (dest.height - dSprite.size[1]) / 2 + dSprite.offset[1];

            const detailSprite = new PNG({
                width: dSprite.size[0],
                height: dSprite.size[1],
            });

            const spriteSheet = new PNG().parse(
                $spritesheetProgress.arrayBuffer as Buffer,
                (error, data) => {
                    // copy main sprite to dest png
                    PNG.bitblt(
                        data,
                        dest,
                        spritesheet.mainSprites[939].pos[0],
                        spritesheet.mainSprites[939].pos[1],
                        mSprite.size[0],
                        mSprite.size[1],
                        mTLX,
                        mTLY
                    );

                    // copy detail sprite to secondary image
                    PNG.bitblt(
                        data,
                        detailSprite,
                        spritesheet.detailSprites[939].pos[0],
                        spritesheet.detailSprites[939].pos[1],
                        dSprite.size[0],
                        dSprite.size[1],
                        0,
                        0
                    );

                    // recolour detail pixels
                    for (let i = 0; i < detailSprite.data.length; i += 4) {
                        detailSprite.data[i] *= 178 / 255;
                        detailSprite.data[i + 1] *= 178 / 255;
                        detailSprite.data[i + 2] *= 255 / 255;
                    }

                    // copy secondary image to dest png ignoring fully transparent pixels
                    copyIgnoreTransparent(
                        detailSprite,
                        dest,
                        0,
                        0,
                        dSprite.size[0],
                        dSprite.size[1],
                        dTLX,
                        dTLY
                    );

                    let newPngBuffer = PNG.sync.write(dest);
                    let blob = new Blob([newPngBuffer], { type: "image/png" });

                    let url = URL.createObjectURL(blob);
                    const img = new Image();
                    img.src = url;

                    document.body.appendChild(img);
                }
            );
        }
    }


*/
