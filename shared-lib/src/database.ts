import { KofiTxId } from "./kofi";

export type ChunkID = `${number},${number}`;
export type ObjKey = string;

export type UserData = {
    username: string;
    epochNextPlace: number;
    epochNextDelete: number;
    epochNextReport: number;
    epochNextGradient: number;
    moderator: boolean;
    hasDonated: boolean;
};

export type UsernameData = { uid: string; displayColor: string };

export interface DatabaseSchema {
    objects: Record<ChunkID, Record<ObjKey, string>>;

    reportedUsers: Record<
        string, // randomly generated key by firebase
        {
            uid: string;
            username: string;
            timestamp: number;
            x: number;
            y: number;
        }
    >;
    bannedUsers: Record<string, number>;
    userCount: number;
    userData: Record<string, UserData>;

    userName: Record<string, UsernameData>;

    userPlaced: Record<ObjKey, string>;

    activeDonations: Record<KofiTxId, number>;
}
