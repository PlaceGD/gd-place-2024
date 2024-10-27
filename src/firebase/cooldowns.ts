import { writable } from "svelte/store";
import { db } from "./firebase";

export let currentPlaceCooldown = writable<number>(0);
export let currentDeleteCooldown = writable<number>(0);
let currentPlaceListener = db
    .ref("metaVariables/placeCooldown")
    .on("value", v => currentPlaceCooldown.set(v.val()));
let currentDeleteListener = db
    .ref("metaVariables/deleteCooldown")
    .on("value", v => currentDeleteCooldown.set(v.val()));
