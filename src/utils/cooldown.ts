import { writable, type Writable } from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";
import type { PathType, SmartDatabase } from "@smart-firebase/client";
import type { DatabaseSchema } from "shared-lib/database";

// type Lol = <P>(p: P) => void;

export class CoolSyncedCooldown {
    private epoch: number = 0;
    readonly unsub: () => void;

    private constructor(path: string, cb: (remaining: number) => void) {
        let interval: NodeJS.Timeout | null = null;
        let onValue = db.ref(path).on("value", v => {
            console.log("ballsack");
            this.epoch = v.val() as number;

            if (interval != null) {
                clearInterval(interval);
            }
            interval = setInterval(
                () => cb((this.epoch - Date.now()) / 1000),
                1000
            );
        });
        this.unsub = () => {
            onValue();

            if (interval != null) {
                clearInterval(interval);
            }
        };
    }

    static new<P extends string>(
        path: number extends PathType<P, DatabaseSchema> ? P : never,
        cb: (remaining: number) => void
    ) {
        return new this(path, cb);
    }
}

export class SyncedCooldown<P extends string, RefS extends string> {
    finished: Writable<boolean> = writable(true);
    display: Writable<string> = writable("--:--");
    private interval: NodeJS.Timeout | null = null;
    private value: number;

    constructor(
        dbPath: P,
        private refS: RefS,
        private duration: number
    ) {
        this.value = duration;

        const epoch = localStorage.getItem(refS);
        if (epoch != null) {
            const numEpoch = parseInt(epoch);

            if (Date.now() > numEpoch) {
                this.display.set("00:00");
                this.finished.set(true);
            } else {
                this.value = (numEpoch - Date.now()) / 1000;
                this.duration = this.value;
                this.start();
                localStorage.setItem(this.refS, `${numEpoch}`);
            }
        } else {
            db.ref(`${dbPath}/${refS}`)
                .get()
                .then(v => {
                    const epoch = v.val() ?? 0;

                    if (Date.now() > epoch) {
                        this.display.set("00:00");
                        this.finished.set(true);
                    }

                    localStorage.setItem(refS, epoch);
                })
                .catch(e => {
                    Toast.showErrorToast(
                        `Failed to get ${refS} epoch from database! (${e})`
                    );
                });
        }
    }

    private updateDisplay() {
        this.display.update(() => {
            const mins = Math.floor(this.duration / 60);
            const secs = Math.floor(this.duration - mins * 60);
            return `${mins >= 10 ? "" : "0"}${mins}:${secs >= 10 ? "" : "0"}${secs}`;
        });
    }

    start() {
        this.updateDisplay();

        localStorage.setItem(this.refS, `${Date.now() + this.duration * 1000}`);
        // updating the database is handled in the cloud function

        this.finished.set(false);

        this.interval = setInterval(() => {
            if (--this.duration < 0) {
                this.finished.set(true);
                clearInterval(this.interval!);
                this.duration = this.value;
            } else {
                this.updateDisplay();
            }
        }, 1000);
    }
}
