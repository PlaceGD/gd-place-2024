export type ChunkID = `${number},${number}`;
export type ObjKey = string;

export interface DatabaseSchema {
    objects: Record<ChunkID, Record<ObjKey, string>>;

    reportedUsers: Record<string, { count: number; username: string }>;
    userCount: number;
    userData: Record<
        string,
        {
            lastDeleted: number;
            lastPlaced: number;
            username: string;
            nameColor?: string;
            banned: boolean;
            moderator: boolean;
        }
    >;

    userName: Record<string, { uid: string }>;

    userPlaced: Record<ObjKey, string>;
}
