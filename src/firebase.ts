import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import {
    getDatabase,
    ref,
    onValue,
    onChildAdded,
    onChildRemoved,
    push,
    DataSnapshot,
    type Unsubscribe,
    set,
    remove,
} from "firebase/database";
import type { GDObject } from "wasm-lib";
import Toast from "./utils/Toast";

const firebaseConfig = {
    apiKey: "AIzaSyB9PSVZzg5WOp26PuCkVrrSTVrWg-XJMgg",
    authDomain: "gd-place-2023.firebaseapp.com",
    databaseURL: "https://gd-place-2023-default-rtdb.firebaseio.com",
    projectId: "gd-place-2023",
    storageBucket: "gd-place-2023.appspot.com",
    messagingSenderId: "358180840785",
    appId: "1:358180840785:web:c0c2c306234f2fe9de5f70",
    measurementId: "G-05Q7TVNRLM",
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
// const analytics = getAnalytics(app);
const db = getDatabase(app);

// set(ref(db, "/blubble"), "\0");

// onValue(ref(db, "/blubble"), data => {
//     console.log(data.val().length);
// });

let unsubMap: { [key: string]: { add: Unsubscribe; remove: Unsubscribe } } = {};

export const subChunk = (
    chunk: [number, number],
    onAdd: (data: DataSnapshot) => void,
    onRemove: (data: DataSnapshot) => void
) => {
    let chunk_name = chunk.join(",");
    let chunk_path = "/objects/" + chunk_name;
    let add = onChildAdded(ref(db, chunk_path), onAdd);
    let remove = onChildRemoved(ref(db, chunk_path), onRemove);
    unsubMap[chunk_name] = { add, remove };
};
export const unsubChunk = (chunk: [number, number]) => {
    let unsub = unsubMap[chunk.join(",")];
    if (unsub != null) {
        unsub.add();
        unsub.remove();
    }
};
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
