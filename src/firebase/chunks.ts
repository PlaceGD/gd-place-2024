import { db } from "./firebase";
import { type SmartDataSnapshot } from "@smart-firebase/client";

let unsubMap: { [key: string]: { add: () => void; remove: () => void } } = {};

export const subChunk = (
    chunk: [number, number],
    onAdd: (data: SmartDataSnapshot<string>) => void,
    onRemove: (data: SmartDataSnapshot<string>) => void
) => {
    let add = db
        .ref(`objects/${chunk[0]},${chunk[1]}`)
        .on("child_added", onAdd);
    let remove = db
        .ref(`objects/${chunk[0]},${chunk[1]}`)
        .on("child_removed", onRemove);
    unsubMap[`${chunk[0]},${chunk[1]}`] = { add, remove };
};
export const unsubChunk = (chunk: [number, number]) => {
    let unsub = unsubMap[chunk.join(",")];
    if (unsub != null) {
        unsub.add();
        unsub.remove();
    }
};
