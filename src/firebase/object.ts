import { push, ref, remove } from "firebase/database";
import { GDObject, GDColor } from "wasm-lib";
import * as wasm from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

import { placeObject, deleteObject } from "./cloud_functions";
import { decode, decodeString, encode, encodeString } from "shared-lib";

export const addObject = (obj: GDObject) => {
    console.log(`${obj.bytes()}`);
    let s = encodeString(obj.bytes(), 126);
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
