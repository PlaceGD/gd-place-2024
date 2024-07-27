import { writable, type Writable } from "svelte/store";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor } from "wasm-lib";
import type { UserData } from "./firebase/auth";
import {
    createIndexedDBStorage,
    createLocalStorage,
    persist,
} from "@macfja/svelte-persistent-store";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

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
    createLocalStorage(),
    "menuSettings"
);

export const editorData = persist(
    writable({
        x: 0,
        y: 0,
        zoom: 0,
    }),
    createLocalStorage(),
    "editorData"
);

export const bannedUsers = writable<string[]>([]);

export enum ExclusiveMenus {
    Moderator,
    Login,
    Settings,
    Kofi,
}

export const openMenu: Writable<ExclusiveMenus | null> = writable(null);

export const analytics = persist(
    writable<boolean | null>(null),
    createLocalStorage(),
    "analytics"
);
export const newReports = persist(
    writable(false),
    createLocalStorage(),
    "newReports"
);

export const loginData = writable<{
    isLoggedIn: boolean;
    currentUserData: UserData | null;
}>({
    isLoggedIn: false,
    currentUserData: null,
});

export const currentNameGradient = persist(
    writable<{
        positions: number[] | null;
        colors: string[] | null;
    }>({
        positions: null,
        colors: null,
    }),
    createLocalStorage(),
    "nameGradient"
);

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
