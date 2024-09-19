import { derived, get, writable, type Writable } from "svelte/store";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor, State } from "wasm-lib";
import type { UserData } from "./firebase/auth";
import {
    createIndexedDBStorage,
    createLocalStorage,
    persist,
    type PersistentStore,
} from "@macfja/svelte-persistent-store";
import { colors, type ObjectCategory } from "shared-lib/gd";
import { tweened, type TweenedOptions } from "svelte/motion";
import { linear } from "svelte/easing";
import { db } from "./firebase/firebase";
import type { RawSpritesheetData } from "./utils/spritesheet/spritesheet";
import type { SmartReference } from "@smart-firebase/client";
import { runTransaction } from "firebase/database";
import {
    BG_TRIGGER,
    GROUND_2_TRIGGER,
    GROUND_TRIGGER,
} from "shared-lib/nexusgen";

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

const persistLocalWritable = <T>(v: T, key: string) =>
    persist(writable(v), createLocalStorage(), key);
const persistLocalTweened = <T>(
    v: T,
    key: string,
    options: TweenedOptions<T>
) => persist(tweened(v, options), createLocalStorage(), key);

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
export const menuZLayer = persistLocalWritable(ZLayer.B2, "menuZLayer");
export const menuZOrder = persistLocalWritable(0, "menuZOrder");

export const menuSelectedSFX = persistLocalWritable(0, "menuSelectedSFX");
export const menuSpeed = persistLocalWritable(0, "menuSpeed");

export const mainColorRGB = derived(
    menuMainColor,
    c => colors.list[c.hue].palette[c.y][c.x]
);
export const detailColorRGB = derived(
    menuDetailColor,
    c => colors.list[c.hue].palette[c.y][c.x]
);

export const menuOpenWidget = persistLocalWritable(
    WidgetType.None,
    "menuOpenWidget"
);

export const lastClosedAnnouncement = persistLocalWritable<number>(
    0,
    "lastClosedAnnouncement"
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

export const editorSettings = persist(
    writable({
        showCollidable: false,
        hideTriggers: false,
        hideGrid: false,
        hideGround: false,
        hideOutline: false,
        hideDeleteText: false,
        hidePlacedTooltip: false,
    }),
    createLocalStorage(),
    "editorSettings"
);

export const bannedUsers = writable<Record<string, boolean>>({});

export enum ExclusiveMenus {
    Moderator,
    Login,
    Settings,
    Kofi,
    Meta,
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
    currentUserData: UserData | null;
}>({
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

export const canPlaceEditDelete = derived(
    [loginData],
    ([l]) => l.currentUserData?.userDetails != null
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

// export const colors = persist(
//     writable({
//         bg: { r: 40, g: 125, b: 255 },
//         ground1: { r: 40, g: 125, b: 255 },
//         ground2: { r: 127, g: 178, b: 255 },
//     }),
//     createLocalStorage(),
//     "colors"
// );

export const DEFAULT_BG_COLOR = { r: 0, g: 15, b: 31 };
export const DEFAULT_GROUND_1_COLOR = { r: 0, g: 15, b: 31 };
export const DEFAULT_GROUND_2_COLOR = { r: 0, g: 82, b: 165 };

export const bgColor = tweened(
    structuredClone(DEFAULT_BG_COLOR),
    // "bgColor",
    { duration: 500, easing: linear }
);
export const ground1Color = tweened(
    structuredClone(DEFAULT_GROUND_1_COLOR),
    // "ground1Color",
    { duration: 500, easing: linear }
);
export const ground2Color = tweened(
    structuredClone(DEFAULT_GROUND_2_COLOR),
    // "ground2Color",
    { duration: 500, easing: linear }
);
export const setLevelColor = (
    state: State,
    obj_id: number,
    rgb: [number, number, number]
) => {
    let obj = state.get_preview_object();
    if (obj_id == BG_TRIGGER) {
        bgColor.set({
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        });
        lastRunColorTrigger.update(v => {
            v.bg = { x: obj.x, y: obj.y, time: Date.now() };
            return v;
        });
    }

    if (obj_id == GROUND_TRIGGER) {
        ground1Color.set({
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        });
        lastRunColorTrigger.update(v => {
            v.ground1 = { x: obj.x, y: obj.y, time: Date.now() };
            return v;
        });
    }

    if (obj_id == GROUND_2_TRIGGER) {
        ground2Color.set({
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        });
        lastRunColorTrigger.update(v => {
            v.ground2 = { x: obj.x, y: obj.y, time: Date.now() };
            return v;
        });
    }
};

export const lastRunColorTrigger = writable<{
    bg: { x: number; y: number; time: number } | null;
    ground1: { x: number; y: number; time: number } | null;
    ground2: { x: number; y: number; time: number } | null;
}>({ bg: null, ground1: null, ground2: null });

export const currentUserColor: Writable<string> = writable("white");

let currentColorUnsub = () => {};
loginData.subscribe(v => {
    currentColorUnsub();
    if (v.currentUserData != null && v.currentUserData.userDetails != null) {
        currentColorUnsub = db
            .ref(
                `userName/${v.currentUserData.userDetails.username.toLowerCase()}/displayColor`
            )
            .on("value", v => {
                currentUserColor.set(v.val() ?? "white");
            });
    }
});

let triggerRunCount = 0;
export const triggerRuns = writable<
    Record<
        number,
        {
            x: number;
            y: number;
        }
    >
>({});
export const addTriggerRun = (x: number, y: number) => {
    let id = triggerRunCount++;

    triggerRuns.update(v => {
        v[id] = { x, y };

        return v;
    });

    setTimeout(() => {
        triggerRuns.update(v => {
            delete v[id];
            return v;
        });
    }, 1500);
};

export const placedByHover = writable<{
    username: string;
    x: number;
    y: number;
} | null>(null);

export const rawSpritesheetData = writable<RawSpritesheetData | null>(null);

export const eventStartTime = writable(Number.POSITIVE_INFINITY);
export const eventEndTime = writable(Number.POSITIVE_INFINITY);

export const eventElapsed = writable(Number.NEGATIVE_INFINITY);

setInterval(() => {
    eventElapsed.set(Date.now() - get(eventStartTime));
}, 500);

db.ref("metaVariables/eventStartTime").on("value", v => {
    console.log(v.val());
    eventStartTime.set(v.val());
});
db.ref("metaVariables/eventEndTime").on("value", v => {
    eventEndTime.set(v.val());
});
