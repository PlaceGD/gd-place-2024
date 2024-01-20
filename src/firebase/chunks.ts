import {
    onChildAdded,
    type DataSnapshot,
    type Unsubscribe,
    onChildRemoved,
    ref,
} from "firebase/database";
import { db } from "./Firebase";

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
