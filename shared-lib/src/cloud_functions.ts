export type PlaceReq = { object: string };
export type DeleteReq = { chunkId: string; objId: string };
export type InitWithUsernameReq = {
    username: string;
    uid: string;
};
export type ReportUserReq = {
    username: string;
};
export type ReportedUserOperationReq = {
    operation: "ignore" | "ban";
    reportedUserUid: string;
};
export type BanReq = {
    username: string;
};
