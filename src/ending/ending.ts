import { writable } from "svelte/store";
import { db } from "../firebase/firebase";

export const LEVEL_NAME_DELAY = 26;

export const DEBUG_ENDING_VISIBILITY = writable(false);

export const CROSSFADE_DURATION = 2000;
