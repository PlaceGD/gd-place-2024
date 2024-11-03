import Stats from "stats.js";
import { writable } from "svelte/store";

export let DEBUG = writable(__DEBUG ?? false); // in development mode it is set to true by default

export let stats: Stats;
if (typeof window !== "undefined") {
    stats = new Stats();

    DEBUG.subscribe(is => {
        if (is) {
            stats = new Stats();
            document.body.appendChild(stats.dom);
            stats.dom.style.inset = "unset";
            stats.dom.style.right = "0";
            stats.dom.style.bottom = "0";
        } else {
            try {
                document.body.removeChild(stats.dom);
            } catch {}

            stats = { begin: () => {}, end: () => 0 } as Stats;
        }
    });
}
