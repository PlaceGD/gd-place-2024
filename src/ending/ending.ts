import { writable } from "svelte/store";
import { db } from "../firebase/firebase";

export const LEVEL_NAME_DELAY = 26;

export const DEBUG_ENDING_VISIBILITY = writable(false);

export const CROSSFADE_DURATION = 2000;

export const STATS = Promise.all([
    db
        .ref("userCount")
        .get()
        .then(snapshot => snapshot.val() ?? 0),
    db
        .ref("totalObjectsPlaced")
        .get()
        .then(snapshot => snapshot.val() ?? 0),
    db
        .ref("totalObjectsDeleted")
        .get()
        .then(snapshot => snapshot.val() ?? 0),
]);
