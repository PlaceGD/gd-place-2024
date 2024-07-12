import { writable, type Writable } from "svelte/store";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor } from "wasm-lib";
import type { UserData } from "./firebase/auth";
import { createLocalStorage, persist } from "@macfja/svelte-persistent-store";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

const LS = createLocalStorage();

export const menuSettings = persist(
    writable({
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
    }),
    LS,
    "menuSettings"
);

export const editorData = persist(
    writable({
        x: 0,
        y: 0,
        zoom: 0,
    }),
    LS,
    "editorData"
);

export const bannedUsers = writable<string[]>([]);

export enum ExclusiveMenus {
    Moderator,
    Login,
}

export const openMenu: Writable<ExclusiveMenus | null> = writable(null);

export const newReports = persist(writable(false), LS, "newReports");

export const loginData = writable<{
    isLoggedIn: boolean;
    currentUserData: UserData | null;
}>({
    isLoggedIn: false,
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
