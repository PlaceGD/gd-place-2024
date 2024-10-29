import { type KofiTxId } from "./kofi.js";

export type ChunkID = `${number},${number}`;
export type ObjKey = string;

export type UserDetails = {
    username: string;
    lastPlaceTimestamp: number;
    lastDeleteTimestamp: number;
    lastReportTimestamp: number;
    lastGradientTimestamp: number;
    lastCharacterTimestamp: number;
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

        /**
         * seconds (auto added to end time) ![oye](https://preview.redd.it/why-does-this-cat-look-like-that-v0-zin7alw9g8jd1.jpeg?width=1080&format=pjpg&auto=webp&s=7684738cd8233040c7a60ce4a9d4296a03524136)
         */
        setNameTime: number;
    };

    /**
     * random uid -> data
     */
    history: Record<string, HistoryItem>;

    levelName: {
        inputs: Record<string, string>;
        history: Record<string, { letter: string; index: number }>;
    };
}
