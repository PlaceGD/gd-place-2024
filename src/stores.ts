import { writable } from "svelte/store";
import LocalSettings from "./utils/LocalSettings";
import { EditTab } from "./place_menu/edit_tab";
import { ZLayer } from "../wasm-lib/pkg/wasm_lib";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

export const menuSettings = writable(
    new LocalSettings("menuSettings", {
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
    })
);
