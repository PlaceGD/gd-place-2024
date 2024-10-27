import {
    PathType,
    SmartDatabase,
    SmartDataSnapshot,
    SmartReference,
} from "@smart-firebase/admin";
import { DatabaseSchema, UserDetails } from "shared-lib/database";
import { HttpsError } from "firebase-functions/v2/https";
import Error from "./errors";

export const getCheckedUserDetails = async (
    db: SmartDatabase<DatabaseSchema>,
    uid: string
): Promise<{
    val: UserDetails;
    ref: SmartReference<UserDetails>;
}> => {
    let userDetailsRef = db.ref(`userDetails/${uid}`);
    let userDetails = (await userDetailsRef.get()).val();
    if (userDetails == undefined) {
        throw Error.code(404, "invalid-argument");
    }

    let banned = (await db.ref(`bannedUsers/${uid}`).get()).exists();
    if (banned) {
        throw Error.code(201, "permission-denied");
    }

    return {
        val: userDetails,
        ref: userDetailsRef as SmartReference<UserDetails>,
    };
};

// copied this from smart-firebase code cause it wasnt exported lol 😛😛😛
export type MapPathsSnapshot<
    T extends string[],
    D,
    Out extends any[] = [],
> = T extends []
    ? Out
    : T extends [infer A, ...infer B]
      ? A extends string
          ? B extends string[]
              ? MapPathsSnapshot<
                    B,
                    D,
                    [...Out, SmartDataSnapshot<PathType<A, D>>]
                >
              : never
          : never
      : never;

export const refAllGet = async <T extends string[], D = DatabaseSchema>(
    db: SmartDatabase<D>,
    ...paths: T
): Promise<MapPathsSnapshot<T, D>> => {
    return Promise.all(paths.map(p => db.ref(p).get())) as any;
};

// const goggler = async (db: SmartDatabase<DatabaseSchema>) => {
//     let [lol] = await Promise.all(["userCount"].map(p => db.ref(p).get()));
// }

export const checkedTransaction = async <T, E>(
    ref: SmartReference<T>,
    is_valid: (value: T) => boolean,
    err: () => E,
    update: (v: T | null) => T
) => {
    let bad = false;
    let result = await ref.transaction(v => {
        if (v != null && !is_valid(v)) {
            bad = true;
            return v;
        }
        return update(v);
    });
    if (bad) {
        throw err();
    }
    if (!result.committed) {
        throw Error.code(501, "aborted");
    }
};
