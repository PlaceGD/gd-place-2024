import _objects from "./gd/objects.json";
import _objectOrder from "./gd/object_order.json";
import _spritesheet from "./gd/spritesheet.json";
import _colors from "./gd/colors.json";
import { remEuclid } from "./util";
import {
    END_RADIUS,
    LEVEL_HEIGHT_UNITS,
    LEVEL_WIDTH_UNITS,
    SFX_TRIGGER,
    SFX_TRIGGER_SOUNDS,
    SONG_TRIGGER,
    SONG_TRIGGER_SONGS,
} from "./nexusgen";

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
export type ObjectSheet =
    | "GJParticleSheet"
    | "PixelSheet01"
    | "GJGameSheet02"
    | "FireSheet01"
    | "GJGameSheet";
export interface ObjectInfo {
    placeOffsetX: number;
    placeOffsetY: number;
    hitboxType: HitboxType;
    builtinScaleX: number;
    builtinScaleY: number;
    category: ObjectCategory; // CATEGORY_ICONS key
    sheet: ObjectSheet;
}

export const objects: Record<string, ObjectInfo> = _objects as any;
export const spritesheet: {
    main_sprites: Record<string, SpriteData>;
    detail_sprites: Record<string, SpriteData>;
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

const paletteContains = (r: number, g: number, b: number): boolean => {
    for (let { palette } of colors.list) {
        for (let i of palette) {
            for (let j of i) {
                if (r == j[0] && g == j[1] && b == j[2]) {
                    return true;
                }
            }
        }
    }
    return false;
};
const isValidColor = (color: GDColor): boolean => {
    if (color.r == 0 && color.g == 0 && color.b == 0 && color.blending)
        return false;

    return paletteContains(color.r, color.g, color.b);
};

export const isValidObject = (obj: GDObjectOpt) => {
    if (obj.z_layer < 0 || obj.z_layer > 8) return false;

    if (obj.z_order < -50 || obj.z_order > 50) return false;

    if (obj.id == SFX_TRIGGER) {
        if (obj.main_color.r >= SFX_TRIGGER_SOUNDS.length) {
            return false;
        }
        if (obj.main_color.g > 12 + 12) {
            return false;
        }
    } else if (obj.id == SONG_TRIGGER) {
        if (obj.main_color.r >= SONG_TRIGGER_SONGS.length) {
            return false;
        }
        if (obj.main_color.g > 12 + 12) {
            return false;
        }
    } else {
        if (!isValidColor(obj.main_color)) return false;
    }

    if (!isValidColor(obj.detail_color)) return false;

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
