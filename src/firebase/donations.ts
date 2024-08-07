import { db } from "./firebase";

let userColorCache: Record<string, string> = {};

export const getUsernameColor = async (username: string): Promise<string> => {
    if (userColorCache[username]) {
        return userColorCache[username];
    }

    let color =
        (
            await db
                .ref(`userName/${username?.toLowerCase()}/displayColor`)
                .get()
        ).val() ?? "white";

    userColorCache[username] = color;

    return color;
};
