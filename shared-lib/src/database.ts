import { type KofiTxId } from "./kofi.js";

export type ChunkID = `${number},${number}`;
export type ObjKey = string;

export type UserDetails = {
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

    /**
     * user uid -> data
     */
    reportedUsers: Record<
        string,
        {
            username: string;
            timestamp: number;
            count: number;
            avg_x: number;
            avg_y: number;
        }
    >;

    /**
     * user uid -> data
     */
    bannedUsers: Record<
        string,
        {
            username: string;
            modName: string;
            reason: string;
        }
    >;
    userCount: number;

    /**
     * user uid -> data
     */
    userDetails: Record<string, UserDetails>;

    /**
     * username -> data
     */
    userName: Record<string, UsernameData>;

    /**
     * obj key -> username
     */
    userPlaced: Record<ObjKey, string>;

    activeDonations: Record<KofiTxId, number>;

    metaVariables: {
        placeCooldown: number;
        deleteCooldown: number;
    };
}
