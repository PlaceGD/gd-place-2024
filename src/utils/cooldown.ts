import {
    derived,
    get,
    writable,
    type Unsubscriber,
    type Writable,
} from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";
import type { PathType, SmartDatabase } from "@smart-firebase/client";
import type { UserDetails } from "shared-lib/database";
import { timerDisplay } from "shared-lib/util";
import { getServerNow, nowStore, type LoginData } from "../stores";
import type { TypedHttpsCallable } from "../firebase/cloud_functions";

export class Cooldown {
    private future: Writable<number> = writable(Number.NEGATIVE_INFINITY);
    private isStarted = false;

    private unsubCooldown: Unsubscriber | null = null;
    private loginDataUnsub: Unsubscriber | null = null;

    constructor(
        private cooldownGetter: TypedHttpsCallable<never, number>,
        userDetails: Writable<LoginData>,
        userDetailsKey: keyof UserDetails
    ) {
        this.loginDataUnsub = userDetails.subscribe(d => {
            if (d.currentUserData != null) {
                this.loginDataUnsub?.();
                this.unsubCooldown?.();

                this.unsubCooldown = db
                    .ref(
                        `/userDetails/${d.currentUserData.user.uid}/${userDetailsKey}`
                    )
                    .on("value", async () => {
                        if (this.isStarted) return;

                        this.updateCooldown();
                    });
            }
        });
    }

    remaining = derived(
        [this.future, nowStore],
        ([f, now]) => (f - now) / 1000
    );
    finished = derived(this.remaining, v => {
        const f = v <= 0;
        this.isStarted = !f;
        return f;
    });
    display = derived(this.remaining, v => timerDisplay(v));

    public cleanup() {
        this.loginDataUnsub?.();
        this.unsubCooldown?.();
    }

    public async updateCooldown() {
        const cooldown = (await this.cooldownGetter()).data;

        this.future.set(getServerNow() + cooldown);
    }

    public start(cooldown: number) {
        if (this.isStarted) return;

        this.future.set(getServerNow() + cooldown);
    }
}
