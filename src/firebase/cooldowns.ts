import { writable } from "svelte/store";
import { db } from "./firebase";

export const placeCooldown = writable<number>(0);
export const deleteCooldown = writable<number>(0);
let placeCooldownListener = db
    .ref("metaVariables/placeCooldown")
    .on("value", v => placeCooldown.set(v.val()));
let deleteCooldownListener = db
    .ref("metaVariables/deleteCooldown")
    .on("value", v => deleteCooldown.set(v.val()));
