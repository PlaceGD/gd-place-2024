import { ref } from "shared-lib/db_util";
import { db } from "./firebase";

let userColorCache: Record<string, string> = {};

export const getUsernameColor = async (username: string): Promise<string> => {
    if (userColorCache[username]) {
        return userColorCache[username];
    }

    let colorRef = await ref(
        db,
        `userName/${username?.toLowerCase()}/displayColor`
    ).get();

    let color = colorRef.val() ?? "white";

    userColorCache[username] = color;

    return color;
};
