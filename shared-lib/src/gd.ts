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

export type ObjectCategory =
    | "Blocks"
    | "Outlines"
    | "Spikes"
    | "OrbsAndGlorbs"
    | "Pixel"
    | "Deco"
    | "Saws"
    | "Triggers";
export type HitboxType = "NoHitbox" | "Solid" | "Hazard" | "Special";
export interface ObjectInfo {
    placeOffsetX: number;
    placeOffsetY: number;
    hitboxType: HitboxType;
    builtinScaleX: number;
    builtinScaleY: number;
    category: ObjectCategory; // CATEGORY_ICONS key
}

export const objects: Record<string, ObjectInfo> = _objects as any;
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
