// import { vec } from "../utils/vector"
import { clamp } from "../util";
import objectList from "./objects.json";
import textureSizes from "./texture_sizes.json";

// export class GdColor {
//     constructor(
//         public hex: string,
//         public blending: boolean,
//         public opacity: number
//     ) {}
//     toDatabaseString() {
//         return `${this.hex};${this.blending ? 1 : 0};${this.opacity}`
//     }
//     static fromDatabaseString(str: string) {
//         let [hex, blending, opacity] = str.split(";")

//         return new GdColor(hex, blending == "1", parseFloat(opacity))
//     }
//     clone() {
//         return new GdColor(this.hex, this.blending, this.opacity)
//     }
// }

// export class GDObject {
//     constructor(
//         public id: number,
//         public x: number,
//         public y: number,
//         public rotation: number,
//         public flip: boolean,
//         public scale: number,
//         public zOrder: number,
//         public mainColor: GdColor,
//         public detailColor: GdColor
//     ) {}

//     toDatabaseString() {
//         return `${this.id};${this.x};${this.y};${Math.round(this.rotation)};${
//             this.flip ? 1 : 0
//         };${this.scale};${
//             this.zOrder
//         };${this.mainColor.toDatabaseString()};${this.detailColor.toDatabaseString()}`
//     }

//     clone() {
//         return new GDObject(
//             this.id,
//             this.x,
//             this.y,
//             this.rotation,
//             this.flip,
//             this.scale,
//             this.zOrder,
//             this.mainColor.clone(),
//             this.detailColor.clone()
//         )
//     }

//     static fromDatabaseString(s: string) {
//         let [
//             id,
//             x,
//             y,
//             rotation,
//             flip,
//             scale,
//             zOrder,
//             mainColor,
//             mainBlending,
//             mainOpacity,
//             detailColor,
//             detailBlending,
//             detailOpacity,
//         ] = s.split(";")
//         //console.log(color)
//         return new GDObject(
//             // 🤣
//             parseInt(id),
//             parseFloat(x),
//             parseFloat(y),
//             parseInt(rotation),
//             flip == "1",
//             parseFloat(scale),
//             parseInt(zOrder),
//             new GdColor(
//                 mainColor,
//                 mainBlending == "1",
//                 parseFloat(mainOpacity)
//             ),
//             new GdColor(
//                 detailColor,
//                 detailBlending == "1",
//                 parseFloat(detailOpacity)
//             )
//         )
//     }

//     settings() {
//         return getObjSettings(this.id)
//     }

//     transform(
//         angle: number,
//         flipHoriz: boolean,
//         flipVert: boolean,
//         offset: boolean
//     ) {
//         const settings = this.settings()
//         if (
//             offset &&
//             !(settings.flipWithoutOffset && (flipVert || flipHoriz))
//         ) {
//             let offVec = vec(settings.offset_x, settings.offset_y).rotated(
//                 -(this.rotation * Math.PI) / 180
//             )
//             this.x -= offVec.x
//             this.y -= offVec.y
//             offVec = offVec.rotated(-(angle * Math.PI) / 180)
//             offVec.x *= flipHoriz ? -1 : 1
//             offVec.y *= flipVert ? -1 : 1
//             this.x += offVec.x
//             this.y += offVec.y
//         }
//         this.rotation += angle
//         if (flipHoriz) {
//             this.flip = !this.flip
//             this.rotation *= -1
//         }
//         if (flipVert) {
//             this.flip = !this.flip
//             this.rotation = 180 - this.rotation
//         }
//     }
// }

export const CATEGORY_ICONS = {
    Blocks: "build_tab_icons/blocks.png",
    Outlines: "build_tab_icons/outlines.png",
    Slopes: "build_tab_icons/slopes.png",
    Spikes: "build_tab_icons/spikes.png",
    Utilities: "build_tab_icons/util.png",
    GroundDeco: "build_tab_icons/ground_deco.png",
    Deco: "build_tab_icons/deco.png",
    Pulsing: "build_tab_icons/pulsing.png",
    Saws: "build_tab_icons/saws.png",
};

interface Object {
    id: number;
    offsetX: number;
    offsetY: number;
    tintable: boolean;
    solid: boolean;
    category: string; // CATEGORY_ICONS key
    flipWithoutOffset?: boolean;
    nondeco?: boolean;
    danger?: boolean;
}

export const OBJECT_SETTINGS: Object[] = objectList;

// object id: index
// let idMapping: { [key: number]: number } = {};
// for (let i = 0; i < OBJECT_SETTINGS.length; i++) {
//     idMapping[OBJECT_SETTINGS[i].id] = i;
// }

// export const getObjSettings = (id: number) =>
//     (id && OBJECT_SETTINGS[idMapping[id]]) || {};

const getMainDetailTexRatio = (id: number) => {
    let main_max = Math.max(...(textureSizes as any)[id].main);
    let detail_max = Math.max(...(textureSizes as any)[id].detail);

    let overall_scale = clamp(main_max / 120, 0, 1);

    return {
        main:
            (main_max > detail_max ? 1 : main_max / detail_max) * overall_scale,
        detail:
            (detail_max > main_max ? 1 : detail_max / main_max) * overall_scale,
    };
};

export const MAIN_DETAIL_TEX_RATIOS: Record<
    number,
    { main: number; detail: number }
> = {};

for (let i = 1; i <= 2000; i++) {
    MAIN_DETAIL_TEX_RATIOS[i] = getMainDetailTexRatio(i);
}
