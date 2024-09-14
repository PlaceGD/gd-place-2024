let channels: Record<string, HTMLAudioElement> = {};

export const playSound = (
    url: string,
    volume: number,
    exclusive_channel?: string,
    extra_cb?: (audio: HTMLAudioElement) => void
) => {
    console.log("Bro", exclusive_channel);
    var audio = new Audio(url);
    audio.volume = volume;
    if (extra_cb != undefined) {
        extra_cb(audio);
    }

    if (exclusive_channel != undefined) {
        // console.log("boing1");
        if (channels[exclusive_channel] != undefined) {
            // console.log("boing2");
            channels[exclusive_channel].pause();
        }
        channels[exclusive_channel] = audio;
    }

    audio.play();
};
