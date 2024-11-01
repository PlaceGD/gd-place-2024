import { GDObjectOpt } from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

import { placeObject, deleteObject } from "./cloud_functions";
import { encodeString } from "shared-lib/base_util";

export const addObject = (
    obj: GDObjectOpt,
    cb: (key: string | null) => void
) => {
    localStorage.setItem("dontShowGuidePopup", "true");
    let v = obj.bytes();

    let s = encodeString(v, 126);

    placeObject({ object: s, timestamp: Date.now() })
        .then(v => {
            cb(v.data);
        })
        .catch(e => {
            if (e.details.code === 600) {
                Toast.showInfoToast(
                    "There are too many objects in this chunk! Try deleting a few!"
                );
            } else {
                console.error("Failed to place object", e.details.message);
                Toast.showErrorToast(
                    `Failed to place object. (${e.details.code})`
                );
            }
            cb(null);
        });
};
export const removeObject = (key: string, chunk: [number, number]) => {
    localStorage.setItem("dontShowGuidePopup", "true");
    deleteObject({ chunkId: `${chunk[0]},${chunk[1]}`, objId: key }).catch(
        e => {
            console.error("Failed to delete object", e.details.message);
            Toast.showErrorToast(
                `Failed to delete object. (${e.details.code})`
            );
        }
    );
};
//MEOLW!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
let userPlacedCache: Record<string, string> = {};

export const getPlacedUsername = (key: string, cb: (name: string) => void) => {
    if (userPlacedCache[key] != null) {
        cb(userPlacedCache[key]);
    } else {
        db.ref(`userPlaced/${key}`)
            .get()
            .then(s => {
                let username = s.val();
                if (username != undefined) {
                    userPlacedCache[key] = username;
                    cb(username);
                }
            });
    }
};
