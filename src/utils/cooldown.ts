import { derived, writable, type Writable } from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";
import type { PathType, SmartDatabase } from "@smart-firebase/client";
import type { DatabaseSchema } from "shared-lib/database";
import { timerDisplay } from "shared-lib/util";

// type Lol = <P>(p: P) => void;

export class SyncedCooldown {
    private epoch: number = 0;
    readonly unsub: () => void;

    remaining: Writable<number> = writable(0);
    finished = derived(this.remaining, v => v <= 0);
    display = derived(this.remaining, v => timerDisplay(v));

    private constructor(path: string) {
        let interval: NodeJS.Timeout | null = null;
        let onValue = db.ref(path).on("value", v => {
            // console.log("ballsack");
            this.epoch = v.val() as number;

            if (interval != null) {
                clearInterval(interval);
            }
            const setRemaining = () => {
                let remaining = this.epoch - Date.now();
                this.remaining.set(remaining / 1000);
            };
            setRemaining();
            interval = setInterval(() => {
                setRemaining();
                // console.log(remaining);
                // cb((this.epoch - Date.now()) / 1000);
            }, 200);
        });
        // console.log("ogoge", path);
        this.unsub = () => {
            onValue();

            if (interval != null) {
                clearInterval(interval);
            }
        };
    }

    static new<P extends string>(
        path: number extends PathType<P, DatabaseSchema> ? P : never
        // cb: (remaining: number) => void
    ) {
        return new this(path);
    }
}
