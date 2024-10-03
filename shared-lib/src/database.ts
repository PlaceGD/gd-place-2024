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

export type HistoryItem =
    | {
          // PLACEMENT
          objKey: ObjKey;
          object: string;
          time: number;
          username: string;
      }
    | {
          // DELETION
          objKey: ObjKey;
          chunk: ChunkID;
          username: string;
          time: number;
      };

// export type ActiveDonation = {
//     txId: string;
//     userClaimed: string | null; // user id
// };

export interface DatabaseSchema {
    announcement: {
        text: string;
        time: number;
    };

    objects: Record<ChunkID, Record<ObjKey, string>>;
    objectCount: Record<ChunkID, number>;

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

    /**
     * user id -> tx id
     */
    claimedDonations: Record<string, KofiTxId>;
    /**
     * tx id -> user id
     */
    activeDonations: Record<KofiTxId, string>;

    metaVariables: {
        placeCooldown: number;
        deleteCooldown: number;
        chunkObjectLimit: number;

        /**
         * unix timesamp
         */
        eventStartTime: number;

        /**
         * unix timesamp
         */
        eventEndTime: number;
    };

    /**
     * random uid -> data
     */
    history: Record<string, HistoryItem>;
}
