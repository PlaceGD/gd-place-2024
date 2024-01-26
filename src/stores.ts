import { writable } from "svelte/store";
import LocalSettingsFactory from "./utils/local_settings";
import { EditTab } from "./place_menu/edit/edit_tab";
import { ZLayer } from "wasm-lib";
import type { UserData } from "./firebase/auth";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

export const menuSettings = writable(
    LocalSettingsFactory("menuSettings", {
        isMinimized: false,

        selectedGroup: TabGroup.Build,
        selectedEditTab: EditTab.Transform,
        selectedBuildTab: "Blocks",
        selectedObject: 1,
        selectedMainColor: { hue: 0, x: 0, y: 0, opacity: 1, blending: false },
        selectedDetailColor: {
            hue: 0,
            x: 0,
            y: 0,
            opacity: 1,
            blending: false,
        },
        zLayer: ZLayer.B1,
        zOrder: 0,
    })
);

export const widgetData = writable({
    maxScaleLen: 4,

    scale: 1,
    prevScale: 1,

    scaleX: 1,
    prevScaleX: 1,
    scaleY: 1,
    prevScaleY: 1,

    angle: 0,
    prevAngle: 0,

    // ix: 1,
    // iy: 0,
    // jx: 0,
    // jy: 1,
});

export const loginData = writable<{
    isLoggedIn: boolean;
    showLoginUI: boolean;
    currentUserData: UserData | null;
}>({
    isLoggedIn: false,
    showLoginUI: false,
    currentUserData: null,
});
