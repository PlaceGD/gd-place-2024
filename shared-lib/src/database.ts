export type ChunkID = `${number},${number}`;
export type ObjKey = string;

export type UserData = {
    username: string;
    epochNextPlace: number;
    epochNextDelete: number;
    epochNextReport: number;
    moderator: boolean;
};

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

    userName: Record<string, { uid: string }>;

    userPlaced: Record<ObjKey, string>;
}
