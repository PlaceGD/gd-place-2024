import { type KofiTxId } from "./kofi.js";

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
    reason: string;
    reportedUserUid: string;
};
export type BanReq = {
    reason: string;
    username: string;
};
export type KofiReq = {
    txId: KofiTxId;
};
export type GradientReq = {
    grad: string;
};
