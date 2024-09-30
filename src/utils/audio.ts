let channels: Record<string, HTMLAudioElement> = {};

type SoundPlayOptions = {
    url: string;
    volume?: number;
    speed?: number;
    exclusive_channel?: string;
    end_cb?: () => void;
};
export const playSound = (options: SoundPlayOptions) => {
    let audio: HTMLAudioElement;
    if (options.exclusive_channel != undefined) {
        audio = channels[options.exclusive_channel] ?? new Audio(options.url);
        audio.currentTime = 0;

        if (audio.src != options.url) {
            audio.src = options.url;
        }
    } else {
        audio = new Audio(options.url);
    }

    audio.volume = (options.volume ?? 1) / 2;
    audio.preservesPitch = false;
    audio.playbackRate = options.speed ?? 1;

    if (options.exclusive_channel != undefined) {
        stopSound(options.exclusive_channel);
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
        delete channels[channel];
    }
};
