import { getFunctions, httpsCallable } from "firebase/functions";
import type {
    ReportUserReq,
    DeleteReq,
    PlaceReq,
    BanReq,
    ReportedUserOperationReq,
    InitWithUsernameReq,
    KofiReq,
} from "shared-lib/cloud_functions";

const functions = getFunctions();
export const placeObject = httpsCallable<PlaceReq>(functions, "placeObject");
export const deleteObject = httpsCallable<DeleteReq>(functions, "deleteObject");
export const reportUser = httpsCallable<ReportUserReq>(functions, "reportUser");
export const banUser = httpsCallable<BanReq>(functions, "banUser");
export const reportedUserOperation = httpsCallable<ReportedUserOperationReq>(
    functions,
    "reportedUserOperation"
);
export const initUserWithUsername = httpsCallable<InitWithUsernameReq>(
    functions,
    "initUserWithUsername"
);
export const submitKofiTxId = httpsCallable<KofiReq>(
    functions,
    "submitKofiTxId"
);
