import { push, ref, remove } from "firebase/database";
import type { GDObject } from "wasm-lib";
import { db } from "./firebase";
import Toast from "../utils/toast";

export const addObject = (obj: GDObject) => {
    let chunkCoord = obj.get_chunk_coord();
    const objectsRef = ref(db, `/objects/${chunkCoord.x},${chunkCoord.y}`);

    try {
        let str = obj.serialize();
        push(objectsRef, str);
    } catch (e: any) {
        Toast.showErrorToast(e.display());
    }

    // const newPostRef = push(postListRef);
};
export const deleteObject = (key: string, chunk: [number, number]) => {
    const objectsRef = ref(db, `/objects/${chunk[0]},${chunk[1]}/${key}`);
    remove(objectsRef);
    // try {
    //     let str = obj.serialize();
    //     push(objectsRef, str);
    // } catch (e: any) {
    //     Toast.showErrorToast(e.display());
    // }

    // const newPostRef = push(postListRef);
};
