import { GDObjectOpt } from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

import { placeObject, deleteObject, getPlaceCooldown } from "./cloud_functions";
import { encodeString } from "shared-lib/base_util";

export const addObject = (
    obj: GDObjectOpt,
    cb: (key: string | null, cooldown: number | null) => void
) => {
    // localStorage.setItem("dontShowGuidePopup", "true");
    // let v = obj.bytes();
    // let s = encodeString(v, 126);
    // placeObject({ object: s })
    //     .then(v => {
    //         cb(v.data.key, v.data.cooldown);
    //     })
    //     .catch(e => {
    //         if (e.details.code === 600) {
    //             Toast.showInfoToast(
    //                 "There are too many objects in this chunk! Try deleting a few!"
    //             );
    //         } else if (e.details.code === 212) {
    //             Toast.showWarningToast("You can't place objects at this time!");
    //         } else {
    //             console.error("Failed to place object", e.details.message);
    //             Toast.showErrorToast(
    //                 `Failed to place object. (${e.details.code})`
    //             );
    //         }
    //         cb(null, null);
    //     });
};
export const removeObject = (
    key: string,
    chunk: [number, number],
    cb: (cooldown: number | null) => void
) => {
    // localStorage.setItem("dontShowGuidePopup", "true");
    // deleteObject({ chunkId: `${chunk[0]},${chunk[1]}`, objId: key })
    //     .then(v => {
    //         cb(v.data.cooldown);
    //     })
    //     .catch(e => {
    //         console.error("Failed to delete object", e.details.message);
    //         Toast.showErrorToast(
    //             `Failed to delete object. (${e.details.code})`
    //         );
    //         cb(null);
    //     });
};
//MEOLW!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
let userPlacedCache: Record<string, string> = {};

export const getPlacedUsername = (key: string, cb: (name: string) => void) => {
    // if (userPlacedCache[key] != null) {
    //     cb(userPlacedCache[key]);
    // } else {
    //     db.ref(`userPlaced/${key}`)
    //         .get()
    //         .then(s => {
    //             let username = s.val();
    //             if (username != undefined) {
    //                 userPlacedCache[key] = username;
    //                 cb(username);
    //             }
    //         });
    // }
};
