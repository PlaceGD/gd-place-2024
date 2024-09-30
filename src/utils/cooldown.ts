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

    private constructor(path: string, constExtra: number = 0) {
        let interval: NodeJS.Timeout | null = null;
        let onValue = db.ref(path).on("value", v => {
            this.epoch = v.val() as number;

            if (interval != null) {
                clearInterval(interval);
            }
            const setRemaining = () => {
                let remaining = this.epoch - Date.now();
                this.remaining.set(remaining / 1000 + constExtra);
            };
            setRemaining();
            interval = setInterval(() => {
                setRemaining();
            }, 1000);
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
        constExtra: number = 0
    ) {
        return new this(path, constExtra);
    }
}
