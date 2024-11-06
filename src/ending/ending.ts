import { get, writable } from "svelte/store";
import { db } from "../firebase/firebase";
import { isGuideActive } from "../guide/guide";
import { toast } from "@zerodevx/svelte-toast";
import { DEFAULT_SETTINGS, editorSettings } from "../stores";

export const LEVEL_NAME_DELAY = 26;

export const DEBUG_ENDING_VISIBILITY = writable(false);

export const CROSSFADE_DURATION = 2000;

export const resetStoresForEnding = () => {
    Howler.stop();
    isGuideActive.set(false);
    toast.pop();
    toast.pop({ target: "announcement" });
    editorSettings.set({
        ...DEFAULT_SETTINGS,
        quality: get(editorSettings).quality,
    });
};
