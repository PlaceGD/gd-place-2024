import { Howl } from "howler";

let howlCache: Record<string, Howl> = {};

// number is howl play id
let channels: Record<string, [Howl, number]> = {};

type SoundPlayOptions = {
    url: string;
    volume?: number;
    speed?: number;
    exclusiveChannel?: string;
    endCb?: () => void;
};
export const playSound = (options: SoundPlayOptions) => {
    if (typeof window === "undefined") return;

    let howl =
        howlCache[options.url] ??
        new Howl({
            src: options.url,
            preload: true,
            onloaderror: e => console.error(e),
        });

    howl.once("load", () => {
        howlCache[options.url] = howl;

        if (options.exclusiveChannel != undefined) {
            let old = channels[options.exclusiveChannel];
            if (old != undefined) {
                old[0].stop(old[1]);
            }
        }
        let id = howl.play();
        howl.volume((options.volume ?? 1) / 2, id);
        howl.rate(options.speed ?? 1, id);

        if (options.endCb != undefined) {
            howl.once("end", options.endCb, id);
        }

        if (options.exclusiveChannel != undefined) {
            channels[options.exclusiveChannel] = [howl, id];
        }
    });

    howl.load();

    // if (options.endCb != undefined) {
    //     audio.onended = options.endCb;
    // }
    // (async () => {
    //     try {
    //         await audio.play();
    //     } catch {}
    // })();
};

export const stopSound = (channel: string) => {
    let v = channels[channel];
    if (v != undefined) {
        v[0].stop(v[1]);
    }
};

export const transferSoundChannel = (from: string, to: string) => {
    let v = channels[from];
    if (v != undefined) {
        v[0].stop(v[1]);
        stopSound(to);
        channels[to] = v;
        delete channels[from];
    }
};
