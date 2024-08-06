import { GDObjectOpt } from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

import { placeObject, deleteObject } from "./cloud_functions";
import { encodeString } from "shared-lib/base_util";
import { ref } from "shared-lib/db_util";

export const addObject = (obj: GDObjectOpt) => {
    let v = obj.bytes();
    console.log(v);
    let s = encodeString(v, 126);
    placeObject({ object: s }).catch(e => {
        Toast.showErrorToast(e);
    });
};
export const removeObject = (key: string, chunk: [number, number]) => {
    deleteObject({ chunkId: `${chunk[0]},${chunk[1]}`, objId: key }).catch(
        e => {
            Toast.showErrorToast(e);
        }
    );
};
//MEOLW!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
let userPlacedCache: Record<string, string> = {};

export const getPlacedUsername = (key: string, cb: (name: string) => void) => {
    if (userPlacedCache[key] != null) {
        cb(userPlacedCache[key]);
    } else {
        ref(db, `userPlaced/${key}`)
            .get()
            .then(username => {
                let val = username.val();
                if (val != undefined) {
                    userPlacedCache[key] = val;
                    cb(val);
                }
            });
    }
};
