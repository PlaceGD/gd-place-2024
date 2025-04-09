import {
    derived,
    readable,
    writable,
    type Readable,
    type Unsubscriber,
    type Writable,
} from "svelte/store";
import { EditTab, WidgetType } from "./place_menu/edit/edit_tab";
import { ZLayer, GDColor, State } from "wasm-lib";
import type { UserData } from "./firebase/auth";
import {
    createLocalStorage,
    createNoopStorage,
    persist,
    type PersistentStore,
} from "@macfja/svelte-persistent-store";
import { colors, type ObjectCategory } from "shared-lib/gd";
import { tweened } from "svelte/motion";
import { linear } from "svelte/easing";
import { db } from "./firebase/firebase";
import type { RawSpritesheetData } from "./utils/spritesheet/spritesheet";
import {
    BG_TRIGGER,
    GROUND_2_TRIGGER,
    GROUND_TRIGGER,
} from "shared-lib/nexusgen";
import { isMobile } from "./utils/document";
import { getExactServerTime } from "./firebase/cloud_functions";
import { LEVEL_NAME_DELAY } from "shared-lib/ending";

const STORAGE_VERSION = 5;

if (typeof window != "undefined") {
    if (
        localStorage["STORAGE_VERSION"] == null ||
        localStorage["STORAGE_VERSION"] != STORAGE_VERSION
    ) {
        localStorage.clear();
        indexedDB.deleteDatabase("PlaceDB");

        localStorage["STORAGE_VERSION"] = STORAGE_VERSION;
    }
}

export enum TabGroup {
    Build,
    Edit,
    Delete,
}

const persistLocalWritable = <T>(v: T, key: string): PersistentStore<T> =>
    persist(
        writable(v),
        // avoids errors in sveltekit
        typeof window === "undefined"
            ? createNoopStorage()
            : createLocalStorage(),
        key
    );
// const persistLocalTweened = <T>(
//     v: T,
//     key: string,
//     options: TweenedOptions<T>
// ) => persist(tweened(v, options), createLocalStorage(), key);

// MARK: User Stuff

export const savePosition = { value: true };

export const bannedUsers = writable<Record<string, boolean>>({});

export const analytics = persistLocalWritable<boolean | null>(
    null,
    "analytics"
);
export const newReports = persistLocalWritable(false, "newReports");

export type LoginData = {
    currentUserData: UserData | null;
};
export const loginData = writable<LoginData>({
    currentUserData: null,
});

export const currentNameGradient = persistLocalWritable(
    {
        positions: null,
        colors: null,
        ids: null,
    },
    "nameGradient"
);

// MARK: Editor Stuff

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
    { hue: 5, x: 0, y: 0, opacity: 1, blending: false },
    "menuMainColor"
);
export const menuDetailColor = persistLocalWritable(
    {
        hue: 5,
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
export const menuSelectedSong = persistLocalWritable(0, "menuSelectedSong");
export const menuSpeed = persistLocalWritable(0, "menuSpeed");

export const mainColorRGB = derived(
    menuMainColor,
    c => colors.list[c.hue].palette[c.y][c.x]
);
export const detailColorRGB = derived(
    menuDetailColor,
    c => colors.list[c.hue].palette[c.y][c.x]
);

export const selectedObject = writable<{
    id: number;
    mainColor: GDColor;
    detailColor: GDColor;
    namePlaced: string | null;
    signupDate: number | null;
    zLayer: ZLayer;
    zOrder: number;
    posX: number;
    posY: number;
} | null>(null);

export const editorData = persistLocalWritable(
    {
        x: 450 + 30 * 20.5,
        y: 360,
        zoom: 0,
    },
    "editorData"
);

export const DEFAULT_SETTINGS = {
    showCollidable: false,
    selectDangerous: false,
    hideTriggers: false,
    noRotatingObjects: false,
    hideGrid: false,
    hideGround: false,
    hideOutline: false,
    showDeleteText: true,
    showPlacedText: true,
    quality: "high",
} as const;
export const editorSettings = persistLocalWritable<{
    showCollidable: boolean;
    selectDangerous: boolean;
    hideTriggers: boolean;
    noRotatingObjects: boolean;
    hideGrid: boolean;
    hideGround: boolean;
    hideOutline: boolean;
    showDeleteText: boolean;
    showPlacedText: boolean;
    quality: "low" | "medium" | "high";
}>(DEFAULT_SETTINGS, "editorSettings");

export const canPlacePreview = writable(true);
export const canPlaceEditDelete = writable(true);

// MARK: Menu Stuff

export const menuOpenWidget = persistLocalWritable(
    WidgetType.None,
    "menuOpenWidget"
);

export enum ExclusiveMenus {
    Moderator,
    Login,
    Settings,
    Kofi,
    Meta,

    Editor,
}
export const openMenu: Writable<ExclusiveMenus | null> = writable(null);

export const lastClosedAnnouncement = persistLocalWritable<number>(
    0,
    "lastClosedAnnouncement"
);

// MARK: Color Stuff

export const DEFAULT_BG_COLOR = { r: 4 * 1.5, g: 24 * 1.5, b: 46 * 1.5 };
export const DEFAULT_GROUND_1_COLOR = { r: 5, g: 40, b: 77 };
export const DEFAULT_GROUND_2_COLOR = { r: 0, g: 120, b: 255 };

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

let nonPreviewColors = {
    bg: DEFAULT_BG_COLOR,
    ground1: DEFAULT_GROUND_1_COLOR,
    ground2: DEFAULT_GROUND_2_COLOR,
};

let previewColor: {
    obj_id: number;
    rgb: [number, number, number];
} | null = null;

export const setLevelColor = (
    state: State,
    obj_id: number,
    rgb: [number, number, number]
) => {
    __setLevelColor(state, obj_id, rgb);

    if (obj_id == BG_TRIGGER) {
        nonPreviewColors.bg = { r: rgb[0], g: rgb[1], b: rgb[2] };
    }

    if (obj_id == GROUND_TRIGGER) {
        nonPreviewColors.ground1 = { r: rgb[0], g: rgb[1], b: rgb[2] };
    }

    if (obj_id == GROUND_2_TRIGGER) {
        nonPreviewColors.ground2 = { r: rgb[0], g: rgb[1], b: rgb[2] };
    }
};

export const setPreviewColor = (
    state: State,
    obj_id: number,
    rgb: [number, number, number]
) => {
    __setLevelColor(state, obj_id, rgb);
    previewColor = { obj_id, rgb };
};

export const resetPreviewColor = (state: State, new_id: number) => {
    if (previewColor != null && new_id == previewColor.obj_id) {
        previewColor = null;
    } else if (previewColor != null) {
        let color = { r: 0, g: 0, b: 0 };
        if (previewColor.obj_id == BG_TRIGGER) {
            color = nonPreviewColors.bg;
        } else if (previewColor.obj_id == GROUND_TRIGGER) {
            color = nonPreviewColors.ground1;
        } else if (previewColor.obj_id == GROUND_2_TRIGGER) {
            color = nonPreviewColors.ground2;
        }

        __setLevelColor(state, previewColor.obj_id, [
            color.r,
            color.g,
            color.b,
        ]);

        previewColor = null;
    }
};

export const applyPreviewColor = () => {
    if (previewColor != null) {
        if (previewColor.obj_id == BG_TRIGGER) {
            nonPreviewColors.bg = {
                r: previewColor.rgb[0],
                g: previewColor.rgb[1],
                b: previewColor.rgb[2],
            };
        } else if (previewColor.obj_id == GROUND_TRIGGER) {
            nonPreviewColors.ground1 = {
                r: previewColor.rgb[0],
                g: previewColor.rgb[1],
                b: previewColor.rgb[2],
            };
        } else if (previewColor.obj_id == GROUND_2_TRIGGER) {
            nonPreviewColors.ground2 = {
                r: previewColor.rgb[0],
                g: previewColor.rgb[1],
                b: previewColor.rgb[2],
            };
        }

        previewColor = null;
    }
};

export const chooseRandomTriggerColor = (state: State, obj_id: number) => {
    const random_hue = Math.floor(Math.random() * colors.hues);
    const random_x = Math.floor(Math.random() * (colors.columns - 1)) + 1;
    const y = obj_id == BG_TRIGGER ? 3 : obj_id == GROUND_TRIGGER ? 2 : 1;
    menuMainColor.set({
        hue: random_hue,
        x: random_x,
        y,
        opacity: 1,
        blending: false,
    });
    menuDetailColor.set({
        hue: random_hue,
        x: random_x,
        y,
        opacity: 1,
        blending: false,
    });

    setPreviewColor(
        state,
        obj_id,
        colors.list[random_hue].palette[y][random_x]
    );
};

export const chooseDefaultColor = () => {
    menuMainColor.set({
        hue: 0,
        x: 0,
        y: 0,
        opacity: 1,
        blending: false,
    });
    menuDetailColor.set({
        hue: 0,
        x: 0,
        y: 0,
        opacity: 1,
        blending: false,
    });
};

const __setLevelColor = (
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
// loginData.subscribe(v => {
//     currentColorUnsub();
//     // if (v.currentUserData != null && v.currentUserData.userDetails != null) {
//     //     currentColorUnsub = db
//     //         .ref(
//     //             `userName/${v.currentUserData.userDetails.username.toLowerCase()}/displayColor`
//     //         )
//     //         .on("value", v => {
//     //             currentUserColor.set(v.val() ?? "white");
//     //         });
//     // }
// });

// MARK: DOM editor indicator stuff

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

// MARK: spritesheet

export const rawSpritesheetData = writable<RawSpritesheetData | null>(null);

// MARK: Event Times

// import { browser }

let debugOffset = 0;

export const setDebugTimeOffset = (offset: number) => {
    debugOffset = offset * 1000;
};

export const addDebugTimeOffset = (offset: number) => {
    debugOffset += offset * 1000;
};

let serverNow = 0;
if (typeof window !== "undefined") {
    const draw = (time: number) => {
        serverNow = Date.now() - debugOffset;
        requestAnimationFrame(draw);
    };
    requestAnimationFrame(draw);
}
export const getServerNow = () => serverNow;

export const nowStore = writable(0);
setTimeout(
    () => {
        setInterval(() => {
            nowStore.set(serverNow);
        }, 1000);
    },
    1500 - (Date.now() % 1000)
);

export const scheduleFor = (
    f: () => void,
    timeUnix: number | Readable<number>,
    { runIfNegative, delay }: { runIfNegative?: boolean; delay?: number } = {
        runIfNegative: false,
        delay: 0,
    },
    mesag: string = ""
) => {
    let timeout: NodeJS.Timeout;
    let time: number;
    let unsub: Unsubscriber;

    const s = () => {
        if (isNaN(time) || time == Infinity || time > 2147483647) {
            return;
        }

        if (time >= 0 || runIfNegative) {
            timeout = setTimeout(
                () => {
                    unsub?.();
                    f();
                },
                time + (delay ?? 0)
            );
        }
    };

    if (typeof timeUnix === "number") {
        time = timeUnix - getServerNow();
        s();
    } else {
        unsub = timeUnix.subscribe(t => {
            clearTimeout(timeout);
            time = t - getServerNow();
            s();
        });
    }
};

export const eventStartTime = writable(Date.now());
export const eventEndTime = writable(Number.POSITIVE_INFINITY);
export const setNameSeconds = writable(0);
// db.ref("metaVariables/eventStartTime").on("value", v => {
//     eventStartTime.set(v.val());
// });
// db.ref("metaVariables/eventEndTime").on("value", v => {
//     eventEndTime.set(Number(v.val()));
// });
// db.ref("metaVariables/setNameSeconds").on("value", v => {
//     setNameSeconds.set(v.val() + LEVEL_NAME_DELAY);
// });

export const eventElapsed = derived(
    [eventStartTime, nowStore],
    ([start, now]) => (now - start) / 1000
);
export const timeLeft = derived(
    [eventEndTime, nowStore],
    ([end, now]) => (end - now) / 1000
);

export type EventStatus =
    | "loading"
    | "before"
    | "during"
    | "name set"
    | "fully done";
export const eventStatus: Readable<EventStatus> = readable("during");

export const eventElapsedContinuous = tweened(0, {
    duration: 1000,
    easing: linear,
});
eventElapsed.subscribe(v => eventElapsedContinuous.set(v));

// MARK: Other Shit Idk Sputnix

export const countdownCreatorNames = writable<string[]>(["", "", "", ""]);

export const userCount = writable(19524);

for (let i = 0; i < 482; i++) {
    setTimeout(() => userCount.update(a => a + 1), Math.random() * 40 * 1000);
}

export const viewingLevelAfterEvent = writable(false);

// db.ref("userCount").on("value", v => {
//     userCount.set(v.val());
// });

export const songPlaying = writable(false);
export let songPlayingIsPreview = writable(false);

export const hasLoggedInBefore = writable(false);
