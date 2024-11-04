import { derived, get, writable, type Writable } from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";
import type { PathType, SmartDatabase } from "@smart-firebase/client";
import type { DatabaseSchema } from "shared-lib/database";
import { timerDisplay } from "shared-lib/util";
import { nowStore } from "../stores";

export class Cooldown {
    private future: Writable<number> = writable(Number.NEGATIVE_INFINITY);

    remaining = derived(
        [this.future, nowStore],
        ([f, now]) => (f - now) / 1000
    );
    finished = derived(this.remaining, v => v <= 0);
    display = derived(this.remaining, v => timerDisplay(v));

    public setCooldown(to: number) {
        this.setFuture(Date.now() + to);
    }
    public setFuture(to: number) {
        this.future.set(to);
    }
}
