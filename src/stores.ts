import { writable, type Writable } from "svelte/store";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor } from "wasm-lib";
import type { UserData } from "./firebase/auth";
import {
    createIndexedDBStorage,
    createLocalStorage,
    persist,
    type PersistentStore,
} from "@macfja/svelte-persistent-store";
import type { ObjectCategory } from "shared-lib/gd";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

const persistLocalWritable = <T>(v: T, key: string) =>
    persist(writable(v), createLocalStorage(), key);

export const menuMinimized = persistLocalWritable(false, "menuMinimized");
export const menuTabGroup = persistLocalWritable(
    TabGroup.Build,
    "menuTabGroup"
);
export const menuEditTab = persistLocalWritable(
    EditTab.Transform,
    "menuEditTab"
);
export const menuBuildTab: PersistentStore<ObjectCategory> =
    persistLocalWritable("Blocks", "menuBuildTab");
export const menuSelectedObject = persistLocalWritable(1, "menuSelectedObject");
export const menuMainColor = persistLocalWritable(
    { hue: 0, x: 0, y: 0, opacity: 1, blending: false },
    "menuMainColor"
);
export const menuDetailColor = persistLocalWritable(
    {
        hue: 0,
        x: 0,
        y: 0,
        opacity: 1,
        blending: false,
    },
    "menuDetailColor"
);
export const menuZLayer = persistLocalWritable(ZLayer.B1, "menuZLayer");
export const menuZOrder = persistLocalWritable(0, "menuZOrder");
export const menuOpenWidget = persistLocalWritable(
    WidgetType.None,
    "menuOpenWidget"
);

// export const menuSettings = persist(
//     writable({
//         isMinimized: false,

//         selectedGroup: TabGroup.Build,
//         selectedEditTab: EditTab.Transform,
//         selectedBuildTab: "Blocks",
//         selectedObject: 1,
//         selectedMainColor: { hue: 0, x: 0, y: 0, opacity: 1, blending: false },
//         selectedDetailColor: {
//             hue: 0,
//             x: 0,
//             y: 0,
//             opacity: 1,
//             blending: false,
//         },
//         zLayer: ZLayer.B1,
//         zOrder: 0,
//         selectedWidget: WidgetType.None,
//     }),
//     createLocalStorage(),
//     "menuSettings"
// );

export const peepo = writable<{
    a: number;
    b: number;
}>({
    a: 0,
    b: 69,
});

export const editorData = persist(
    writable({
        x: 0,
        y: 0,
        zoom: 0,
    }),
    createLocalStorage(),
    "editorData"
);

export const editorSettings = persist(
    writable({
        showCollidable: false,
        hideTriggers: true,
    }),
    createLocalStorage(),
    "editorSettings"
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

export const colors = persist(
    writable({
        bg: { r: 40, g: 125, b: 255 },
        ground1: { r: 40, g: 125, b: 255 },
        ground2: { r: 127, g: 178, b: 255 },
    }),
    createLocalStorage(),
    "colors"
);
