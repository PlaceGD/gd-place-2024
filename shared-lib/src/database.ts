export type ChunkID = `${number},${number}`;
export type ObjKey = string;

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
    userData: Record<
        string,
        {
            epochNextReport: number;
            lastDeleted: number;
            lastPlaced: number;
            username: string;
            nameColor?: string;
            moderator: boolean;
        }
    >;

    userName: Record<string, { uid: string }>;

    userPlaced: Record<ObjKey, string>;
}
