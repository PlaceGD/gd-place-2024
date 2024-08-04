import {
    objectOrder,
    objects,
    type ObjectCategory,
    type ObjectInfo,
} from "shared-lib/gd";

import blocksUrl from "./assets/build_tab_icons/blocks.png?url";
import outlinesUrl from "./assets/build_tab_icons/outlines.png?url";
import spikesUrl from "./assets/build_tab_icons/spikes.png?url";
import orbsUrl from "./assets/build_tab_icons/orbs.png?url";
import pixelUrl from "./assets/build_tab_icons/pixel.png?url";
import decoUrl from "./assets/build_tab_icons/deco.png?url";
import sawsUrl from "./assets/build_tab_icons/saws.png?url";
import triggersUrl from "./assets/build_tab_icons/triggers.png?url";

export const CATEGORY_ICONS: Record<ObjectCategory, string> = {
    Blocks: blocksUrl,
    Outlines: outlinesUrl,
    Spikes: spikesUrl,
    OrbsAndGlorbs: orbsUrl,
    Pixel: pixelUrl,
    Deco: decoUrl,
    Saws: sawsUrl,
    Triggers: triggersUrl,
};

export const getObjsInOrder = (): [number, ObjectInfo][] =>
    objectOrder.map(id => [id, objects[id]]);

// object id: index
// let idMapping: { [key: number]: number } = {};
// for (let i = 0; i < OBJECT_SETTINGS.length; i++) {
//     idMapping[OBJECT_SETTINGS[i].id] = i;
// }

// export const getObjSettings = (id: number) =>
//     (id && OBJECT_SETTINGS[idMapping[id]]) || {};

// const getMainDetailTexRatio = (id: number) => {
//     // let main_max = Math.max(...(textureSizes as any)[id].main);
//     // let detail_max = Math.max(...(textureSizes as any)[id].detail);

//     // let overall_scale = clamp(main_max / 120, 0, 1);

//     // return {
//     //     main:
//     //         (main_max > detail_max ? 1 : main_max / detail_max) * overall_scale,
//     //     detail:
//     //         (detail_max > main_max ? 1 : detail_max / main_max) * overall_scale,
//     // };

//     return {
//         main: 1,
//         detail: 1,
//     };
// };

// export const MAIN_DETAIL_TEX_RATIOS: Record<
//     number,
//     { main: number; detail: number }
// > = {};

// for (let i = 1; i <= 2000; i++) {
//     MAIN_DETAIL_TEX_RATIOS[i] = getMainDetailTexRatio(i);
// }
