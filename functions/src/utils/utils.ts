import { SmartDatabase, SmartReference } from "@smart-firebase/admin";
import { DatabaseSchema, UserDetails } from "shared-lib/database";
import { HttpsError } from "firebase-functions/v2/https";

export const getCheckedUserDetails = async (
    db: SmartDatabase<DatabaseSchema>,
    uid: string
): Promise<{
    userDetails: UserDetails;
    userDetailsRef: SmartReference<UserDetails>;
}> => {
    let userDetailsRef = db.ref(`userDetails/${uid}`);
    let userDetails = (await userDetailsRef.get()).val();
    if (userDetails == undefined) {
        throw new HttpsError("invalid-argument", "Missing user data");
    }

    let banned = (await db.ref(`bannedUsers/${uid}`).get()).exists();
    if (banned) {
        throw new HttpsError("permission-denied", "Banned");
    }

    return {
        userDetails,
        userDetailsRef: userDetailsRef as SmartReference<UserDetails>,
    };
};
