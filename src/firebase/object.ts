import { push, ref, remove } from "firebase/database";
import type { GDObject } from "wasm-lib";
import { db } from "./Firebase";
import Toast from "../utils/Toast";

import { placeObject, deleteObject } from "./cloudFunctions";

export const addObject = (obj: GDObject) => {
    placeObject({ object: obj.serialize() }).catch(e => {
        Toast.showErrorToast(e);
    });
};
export const removeObject = (key: string, chunk: [number, number]) => {
    console.log("removing object", key, chunk);
    deleteObject({ chunkId: `${chunk[0]},${chunk[1]}`, objId: key }).catch(
        e => {
            Toast.showErrorToast(e);
        }
    );
};
