import { isValidObject, objects } from "shared-lib/gd";
import * as wasm from "wasm-lib";
import { canPlacePreview } from "../stores";
import { get } from "svelte/store";
import Toast from "./toast";

export const KOFI_ID = "Z8Z410GRY2";

export const setCheckedPreviewObject = (
    state: wasm.State,
    obj: wasm.GDObjectOpt
): boolean => {
    if (get(canPlacePreview) == false) {
        return false;
    }

    if (
        objects[obj.id].hitboxType == "Solid" &&
        (obj.x_angle % 18 != 0 || obj.y_angle % 18 != 0)
    ) {
        obj.x_angle = 0;
        obj.y_angle = 18;
    }

    if (isValidObject(obj)) {
        state.set_preview_object(obj);
        return true;
    }

    return false;
};

export const extractFilenames = <T>(
    glob: Record<string, T>
): Record<string, T> => {
    return Object.entries(glob).reduce<Record<string, T>>(
        (prev, [path, data]) => {
            const fname = path.split("/").at(-1)?.split(".")[0];

            if (fname == null) return prev;

            prev[fname] = data;
            return prev;
        },
        {}
    );
};

export const notNaNAnd = (n: string, c: (n: number) => boolean) => {
    let s = parseInt(n);
    return !isNaN(s) && c(s);
};

export const showFpsWarning = () => {
    const helpLink = {
        chrome: "https://help.glorify.com/en/articles/3730301-turn-hardware-acceleration-on-in-google-chrome",
        firefox: "https://support.mozilla.org/en-US/kb/performance-settings",
    };
    let link =
        "https://kb.bigmarker.com/knowledge/enable-disable-hardware-acceleration-in-browser";

    const browser = navigator.userAgent.toLowerCase();
    if (browser.includes("chrome")) {
        link = helpLink.chrome;
    } else if (browser.includes("firefox")) {
        link = helpLink.firefox;
    }

    Toast.showInfoToast(
        `Low FPS detected. Make sure you have <a href="${link}" target="_blank" rel="norefer" class="underline">GPU acceleration enabled in your browser</a>.`
    );
};
