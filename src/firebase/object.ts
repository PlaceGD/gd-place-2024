import { get, push, ref, remove } from "firebase/database";
import { GDObjectOpt, GDColor } from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

import { placeObject, deleteObject } from "./cloud_functions";
import { decode, decodeString, encode, encodeString } from "shared-lib";

export const addObject = (obj: GDObjectOpt) => {
    let s = encodeString(obj.bytes(), 126);
    console.log(s.length);
    // console.log("bbobo", decodeString(s, 126).byteLength);
    placeObject({ object: s }).catch(e => {
        Toast.showErrorToast(e);
    });
};
export const removeObject = (key: string, chunk: [number, number]) => {
    // console.log("removing object", key, chunk);
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
        get(ref(db, `userPlaced/${key}`)).then(username => {
            userPlacedCache[key] = username.val();
            cb(username.val());
        });
    }
};
