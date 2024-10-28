import { get } from "svelte/store";
import { db } from "./firebase";
import { loginData } from "../stores";

let userColorCache: Record<string, string> = {};

export const setUsernameColorCache = (username: string, color: string) => {
    userColorCache[username] = color;
};

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
