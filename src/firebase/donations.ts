import { get, ref } from "firebase/database";
import { db } from "./firebase";

let userColorCache: Record<string, string> = {};

export const getUsernameColor = async (username: string): Promise<string> => {
    if (userColorCache[username]) {
        return userColorCache[username];
    }

    let colorRef = await get(
        ref(db, `userName/${username?.toLowerCase()}/displayColor`)
    );

    let color;

    if (!colorRef.exists()) {
        color = "white";
    } else {
        color = colorRef.val();
    }

    userColorCache[username] = color;

    return color;
};
