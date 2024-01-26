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

export const loginData = writable<{
    isLoggedIn: boolean;
    showLoginUI: boolean;
    currentUserData: UserData | null;
}>({
    isLoggedIn: false,
    showLoginUI: false,
    currentUserData: null,
});
