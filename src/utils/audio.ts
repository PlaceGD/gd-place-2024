let channels: Record<string, HTMLAudioElement> = {};

type SoundPlayOptions = {
    url: string;
    volume?: number;
    speed?: number;
    exclusive_channel?: string;
    end_cb?: () => void;
};
export const playSound = (
    options: SoundPlayOptions
    // extra_cb?: (audio: HTMLAudioElement) => void
) => {
    let audio = new Audio(options.url);
    audio.volume = (options.volume ?? 1) / 2;
    audio.preservesPitch = false;
    audio.playbackRate = options.speed ?? 1;

    // if (extra_cb != undefined) {
    //     extra_cb(audio);
    // }

    if (options.exclusive_channel != undefined) {
        // console.log("boing1");
        if (channels[options.exclusive_channel] != undefined) {
            // console.log("boing2");
            channels[options.exclusive_channel].pause();
        }
        channels[options.exclusive_channel] = audio;
    }
    if (options.end_cb != undefined) {
        audio.onended = options.end_cb;
    }
    audio.play();
};

export const stopSound = (channel: string) => {
    if (channels[channel] != undefined) {
        channels[channel].pause();
    }
};
