interface Sprite {
    pos: [number, number];
    size: [number, number];
    offset: [number, number];
    rotated: boolean;
}

interface SpriteSheetOptions {
    width: number;
    height: number;
    src: string;

    main_sprites: {
        [id: number]: Sprite;
    };
    detail_sprites: {
        [id: number]: Sprite;
    };
}

export default class SpriteSheet {
    private readonly opts: SpriteSheetOptions;
    private canvas: OffscreenCanvas;

    private ctx: OffscreenCanvasRenderingContext2D;

    constructor(opts: SpriteSheetOptions) {
        this.opts = opts;

        this.canvas = new OffscreenCanvas(opts.width, opts.height);
        this.ctx = this.canvas.getContext("2d", {
            desynchronized: true,
            willReadFrequently: true,
        })!;

        let image = new Image(opts.width, opts.height);
        image.src = opts.src;

        this.ctx.drawImage(image, 0, 0);
    }

    mainSpriteFromId(id: number): string {
        if (!this.opts.main_sprites[id]) {
            return "";
        }
        let { pos, size } = this.opts.main_sprites[id];

        let spr = this.ctx.getImageData(pos[0], pos[1], size[0], size[1]);

        let expr = document.createElement("canvas");
        expr.width = 100;
        expr.height = 100;
        let ctx = expr.getContext("2d")!;
        ctx.putImageData(spr, 0, 0);

        return expr.toDataURL("image/png");
    }
    detailSpriteFromId(id: number): ImageData {
        let { pos, size } = this.opts.detail_sprites[id];

        return this.ctx.getImageData(pos[0], pos[1], size[0], size[1]);
    }

    spriteHasDetail(id: number): boolean {
        return this.opts.detail_sprites[id] != undefined;
    }
}
