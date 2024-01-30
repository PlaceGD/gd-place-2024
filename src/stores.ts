import { writable } from "svelte/store";
import LocalSettingsFactory from "./utils/local_settings";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor } from "wasm-lib";
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
        selectedWidget: WidgetType.None,
    })
);

export const showModeratorOptions = writable<boolean>(false);

export const loginData = writable<{
    isLoggedIn: boolean;
    showLoginUI: boolean;
    currentUserData: UserData | null;
}>({
    isLoggedIn: false,
    showLoginUI: false,
    currentUserData: null,
});

let deleteTextCounter = 0;
export const deleteTexts = writable<
    Record<
        number,
        {
            name: string;
            x: number;
            y: number;
        }
    >
>({});

export const addDeleteText = (name: string, x: number, y: number) => {
    let id = deleteTextCounter++;

    deleteTexts.update(v => {
        v[id] = { name, x, y };

        return v;
    });

    setTimeout(() => {
        deleteTexts.update(v => {
            delete v[id];
            return v;
        });
    }, 1500);
};

export const selectedObject = writable<{
    id: number;
    mainColor: GDColor;
    detailColor: GDColor;
    namePlaced: string | null;
    zLayer: ZLayer;
    zOrder: number;
} | null>(null);
