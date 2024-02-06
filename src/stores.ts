import { writable, type Writable } from "svelte/store";
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

export const bannedUsers = writable<string[]>([]);

export const showModeratorOptions = writable({
    show: false,
    newReports: localStorage.getItem("newReports") == "1",
});

// test
class Timer {
    private value: number;
    finished: Writable<boolean> = writable(false);
    private interval: NodeJS.Timeout | null = null;
    display: Writable<string> = writable("00:00");

    constructor() {
        this.value = 0;
    }

    private updateDisplay() {
        this.display.update(() => {
            const mins = Math.floor(this.value / 60);
            const secs = Math.floor(this.value - mins * 60);
            return `${mins > 10 ? "" : "0"}${mins}:${secs > 10 ? "" : "0"}${secs}`;
        });
    }

    start(atMins: number) {
        this.finished.update(() => false);
        this.value = 60 * atMins;

        this.interval = setInterval(() => {
            if (this.value-- <= 0) {
                this.updateDisplay();
                this.finished.update(() => true);
                clearInterval(this.interval!);
            } else {
                this.updateDisplay();
            }
        }, 1000);
    }
}

export const reportTimer = new Timer();

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
