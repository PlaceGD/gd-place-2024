import { get, writable } from "svelte/store";
import { db } from "../firebase/firebase";
import { isGuideActive } from "../guide/guide";
import { toast } from "@zerodevx/svelte-toast";
import { DEFAULT_SETTINGS, editorSettings } from "../stores";
import * as wasm from "wasm-lib";

export const DEBUG_ENDING_VISIBILITY = writable(false);

export const CROSSFADE_DURATION = 2000;
