import { initializeApp } from "firebase-admin/app";
import { Database } from "firebase-admin/database";

export { deleteObject, placeObject } from "./object";
export { initUserWithUsername, reportUser } from "./user";

interface IDatabase {
    reportedUsers: Record<string, { count: number; user: string }>;
    userCount: number;
    dumbass: {
        pussy: number;
        dick: Record<string, BigInt>;
    };
    userData: Record<
        string,
        { lastDeleted: number; lastPlaced: number; username: string }
    >;
}

type PathWithNext<T> = T extends `${infer First}/${infer Next}`
    ? [First, Next]
    : undefined;

type RefInner<T extends string, CurrType> =
    PathWithNext<T> extends [infer Key, infer Next extends string]
        ? Key extends keyof CurrType
            ? RefInner<Next, CurrType[Key]>
            : never
        : T extends keyof CurrType
          ? CurrType[T]
          : never;

type Ref<T extends string> = RefInner<T, IDatabase>;

const getFromRef = async <T extends string>(
    value: T,
    db: Database
): Promise<Ref<T>> => {
    return (await db.ref(value).get()).val() as Ref<T>;
};

/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
export const CHUNK_SIZE_BLOCKS = 20;
export const CHUNK_SIZE_UNITS = CHUNK_SIZE_BLOCKS * 30;

/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED (THERE ARE ANOTHER PLACEIN WASM)
export const LEVEL_WIDTH_BLOCKS = 400;
export const LEVEL_HEIGHT_BLOCKS = 80;
export const LEVEL_WIDTH_UNITS = LEVEL_WIDTH_BLOCKS * 30;
export const LEVEL_HEIGHT_UNITS = LEVEL_HEIGHT_BLOCKS * 30;

initializeApp();
