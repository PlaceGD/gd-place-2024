import { KofiTxId } from "./kofi";

export type PlaceReq = { object: string };
export type DeleteReq = { chunkId: string; objId: string };
export type InitWithUsernameReq = {
    username: string;
    uid: string;
    turnstileResp: string;
};
export type ReportUserReq = {
    username: string;
    turnstileResp: string;
    x: number;
    y: number;
};
export type ReportedUserOperationReq = {
    operation: "ignore" | "ban";
    reportedUserUid: string;
};
export type BanReq = {
    username: string;
};

export type KofiReq = {
    txId: KofiTxId;
};
