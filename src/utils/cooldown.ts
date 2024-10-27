import { derived, get, writable, type Writable } from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";
import type { PathType, SmartDatabase } from "@smart-firebase/client";
import type { DatabaseSchema } from "shared-lib/database";
import { timerDisplay } from "shared-lib/util";

export class SyncedCooldown {
    private lastTimestamp: number = 0;
    readonly unsub: () => void;

    remaining: Writable<number> = writable(0);
    finished = derived(this.remaining, v => v <= 0);
    display = derived(this.remaining, v => timerDisplay(v));

    private constructor(
        path: string,
        currentCooldown: Writable<number> | number
    ) {
        let interval: NodeJS.Timeout | null = null;

        let cooldownValue: number;
        let cooldownUsub = null;
        if (typeof currentCooldown === "number") {
            cooldownValue = currentCooldown;
        } else {
            cooldownUsub = currentCooldown.subscribe(
                cooldown => (cooldownValue = cooldown)
            );
        }

        let onValue = db.ref(path).on("value", v => {
            this.lastTimestamp = v.val() as number;

            if (interval != null) {
                clearInterval(interval);
            }
            const setRemaining = () => {
                let remaining =
                    this.lastTimestamp + cooldownValue * 1000 - Date.now();
                this.remaining.set(remaining / 1000);
            };
            setRemaining();
            interval = setInterval(() => {
                setRemaining();
            }, 1000);
        });
        this.unsub = () => {
            onValue();

            cooldownUsub?.();

            if (interval != null) {
                clearInterval(interval);
            }
        };
    }

    static new<P extends string>(
        path: number extends PathType<P, DatabaseSchema> ? P : never,
        currentCooldown: Writable<number> | number
    ) {
        return new this(path, currentCooldown);
    }
}
