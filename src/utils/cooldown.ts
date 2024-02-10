import { get, ref } from "firebase/database";
import { writable, type Writable } from "svelte/store";
import { db } from "../firebase/firebase";
import Toast from "./toast";

export class SyncedCooldown {
    finished: Writable<boolean> = writable(false);
    display: Writable<string> = writable("--:--");
    private interval: NodeJS.Timeout | null = null;
    private value: number;

    constructor(
        dbPath: string,
        private refS: string,
        private duration: number
    ) {
        this.value = duration;

        const epoch = localStorage.getItem(refS);
        if (epoch != null) {
            const numEpoch = parseInt(epoch);

            if (Date.now() > numEpoch) {
                this.display.update(() => "00:00");
                this.finished.update(() => true);
            } else {
                this.value = (numEpoch - Date.now()) / 1000;
                this.duration = this.value;
                this.start();
                localStorage.setItem(this.refS, `${numEpoch}`);
            }
        } else {
            get(ref(db, `${dbPath}/${refS}`))
                .then(v => {
                    const epoch = v.val() ?? 0;

                    if (Date.now() > epoch) {
                        this.display.update(() => "00:00");
                        this.finished.update(() => true);
                    }

                    localStorage.setItem(refS, epoch);
                })
                .catch(e => {
                    Toast.showErrorToast(
                        `Failed to get ${ref} epoch from database! (${e})`
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

        this.finished.update(() => false);

        this.interval = setInterval(() => {
            if (--this.duration < 0) {
                this.finished.update(() => true);
                clearInterval(this.interval!);
                this.duration = this.value;
            } else {
                this.updateDisplay();
            }
        }, 1000);
    }
}
