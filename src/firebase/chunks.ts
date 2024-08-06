import { DataSnapshot, ref } from "shared-lib/db_util";
import { db } from "./firebase";

type Unsubscribe = () => void;
let unsubMap: { [key: string]: { add: Unsubscribe; remove: Unsubscribe } } = {};

export const subChunk = (
    chunk: [number, number],
    onAdd: (data: DataSnapshot<string>) => void,
    onRemove: (data: DataSnapshot<string>) => void
) => {
    let add = ref(db, `objects/${chunk[0]},${chunk[1]}`).onChildAdded(onAdd);
    let remove = ref(db, `objects/${chunk[0]},${chunk[1]}`).onChildRemoved(
        onRemove
    );

    unsubMap[`${chunk[0]},${chunk[1]}`] = { add, remove };
};
export const unsubChunk = (chunk: [number, number]) => {
    let unsub = unsubMap[chunk.join(",")];
    if (unsub != null) {
        unsub.add();
        unsub.remove();
    }
};
