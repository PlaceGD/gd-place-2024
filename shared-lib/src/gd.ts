import _objects from "./gd/objects.json";
import _objectOrder from "./gd/object_order.json";
import _spritesheet from "./gd/spritesheet.json";
import _colors from "./gd/colors.json";

export interface SpriteData {
    pos: [number, number];
    size: [number, number];
    offset: [number, number];
    rotated: boolean;
}
export interface ObjectData {
    placeOffsetX: number;
    placeOffsetY: number;
    tintable: boolean;
    solid: boolean;
    category: string; // CATEGORY_ICONS key
    builtinScale: number;
}

export const objects: Record<string, ObjectData> = _objects;
export const spritesheet: {
    mainSprites: Record<string, SpriteData>;
    detailSprites: Record<string, SpriteData>;
} = _spritesheet as any;
export const objectOrder: number[] = _objectOrder;
export const colors: {
    columns: number;
    hues: number;
    rows: number;
    list: { hue: number; palette: [number, number, number][][] }[];
} = _colors as any;
