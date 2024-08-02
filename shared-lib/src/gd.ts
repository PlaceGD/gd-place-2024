import _objects from "./gd/objects.json";
import _objectOrder from "./gd/object_order.json";
import _spritesheet from "./gd/spritesheet.json";
import _colors from "./gd/colors.json";
import { remEuclid } from "./util";

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

export interface GDColor {
    r: number;
    g: number;
    b: number;
    opacity: number;
    blending: boolean;
}

export interface GDObjectOpt {
    id: number;
    x: number;
    y: number;
    x_scale_exp: number;
    x_angle: number;
    y_scale_exp: number;
    y_angle: number;
    z_layer: number;
    z_order: number;
    main_color: GDColor;
    detail_color: GDColor;
}
export const GD_OBJECT_OPT_BYTE_SIZE = 26;

export const isValidObject = (obj: GDObjectOpt) => {
    if (obj.z_layer < 0 || obj.z_layer > 8) return false;

    if (obj.z_order < -50 || obj.z_order > 50) return false;

    if (remEuclid(obj.x_angle - obj.y_angle, 36) == 0) {
        return false;
    }

    if (
        objects[obj.id].hitboxType == "Solid" &&
        (obj.x_angle % 18 != 0 || obj.y_angle % 18 != 0)
    ) {
        return false;
    }

    return true;
};
