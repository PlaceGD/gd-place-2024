import { extractFilenames } from "../../utils/misc";

export const SFX_SOUNDS = extractFilenames<string>(
    import.meta.glob("../assets/sfx_tab/sfx/*.mp3", {
        eager: true,
        query: "?url",
        import: "default",
    })
);
export const SONG_SOUNDS = extractFilenames<string>(
    import.meta.glob("../assets/song_tab/songs/*.mp3", {
        eager: true,
        query: "?url",
        import: "default",
    })
);

export const SFX_ICONS = extractFilenames<string>(
    import.meta.glob("../assets/sfx_tab/icons/*.png", {
        eager: true,
        query: "?url",
        import: "default",
    })
);

export const SONG_ICONS = extractFilenames<string>(
    import.meta.glob("../assets/song_tab/icons/*.png", {
        eager: true,
        query: "?url",
        import: "default",
    })
);
