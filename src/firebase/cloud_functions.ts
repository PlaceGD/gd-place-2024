import {
    getFunctions,
    httpsCallable as fHttpsCallable,
    type HttpsCallable,
    type FunctionsErrorCode,
    connectFunctionsEmulator,
} from "firebase/functions";
import { type FirebaseError as FFirebaseError } from "firebase/app";
import type {
    ReportUserReq,
    DeleteReq,
    PlaceReq,
    BanReq,
    ReportedUserOperationReq,
    InitWithUsernameReq,
    KofiReq,
    GradientReq,
    FirebaseError,
    MetaReq,
    PlaceRes,
    LevelNameReq,
    DeleteRes,
    GradientRes,
    ReportUserRes,
    LevelNameRes,
} from "shared-lib/cloud_functions";
import { GDColor, GDObjectOpt } from "wasm-lib";
import { isValidObject, objects } from "shared-lib/gd";
import { encodeString } from "shared-lib/base_util";

interface TypedPromise<ResolveType, RejectType> extends Promise<ResolveType> {
    catch<TResult = never>(
        onrejected?:
            | ((reason: RejectType) => TResult | PromiseLike<TResult>)
            | undefined
            | null
    ): TypedPromise<ResolveType, TResult>;
    then<TResult1 = ResolveType, TResult2 = never>(
        onfulfilled?:
            | ((value: ResolveType) => TResult1 | PromiseLike<TResult1>)
            | undefined
            | null,
        onrejected?:
            | ((reason: RejectType) => TResult2 | PromiseLike<TResult2>)
            | undefined
            | null
    ): TypedPromise<TResult1, RejectType>;
}

interface FunctionsError extends FFirebaseError {
    readonly code: FunctionsErrorCode;
    readonly details: FirebaseError;
}

type _TypedHttpsCallable<FHttpCallable extends HttpsCallable<any, any>> = (
    ...args: Parameters<FHttpCallable>
) => ReturnType<FHttpCallable> extends Promise<infer Res>
    ? TypedPromise<Res, FunctionsError>
    : never;

export type TypedHttpsCallable<
    RequestData,
    ResponseData = unknown,
> = _TypedHttpsCallable<HttpsCallable<RequestData, ResponseData>>;

const httpsCallable = <Req, Res = unknown>(
    ...args: Parameters<typeof fHttpsCallable>
): TypedHttpsCallable<Req, Res> => {
    return fHttpsCallable(...args) as TypedHttpsCallable<Req, Res>;
};

const functions = getFunctions();

if (typeof window !== "undefined" && __USE_DB === "local") {
    connectFunctionsEmulator(functions, "127.0.0.1", 5001);
}

export const placeObject = httpsCallable<PlaceReq, PlaceRes>(
    functions,
    "placeObject"
);
export const deleteObject = httpsCallable<DeleteReq, DeleteRes>(
    functions,
    "deleteObject"
);
export const reportUser = httpsCallable<ReportUserReq, ReportUserRes>(
    functions,
    "reportUser"
);
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
export const changeNameGradient = httpsCallable<GradientReq, GradientRes>(
    functions,
    "changeNameGradient"
);
export const setMeta = httpsCallable<MetaReq>(functions, "setMeta");
export const setLevelNameLetter = httpsCallable<LevelNameReq, LevelNameRes>(
    functions,
    "setLevelNameLetter"
);

export const getPlaceCooldown = httpsCallable<{}, number>(
    functions,
    "getPlaceCooldown"
);
export const getDeleteCooldown = httpsCallable<{}, number>(
    functions,
    "getDeleteCooldown"
);
export const getReportCooldown = httpsCallable<{}, number>(
    functions,
    "getReportCooldown"
);
export const getGradientCooldown = httpsCallable<{}, number>(
    functions,
    "getGradientCooldown"
);
export const getCharacterCooldown = httpsCallable<{}, number>(
    functions,
    "getCharacterCooldown"
);
const getServerTime = httpsCallable<{}, number>(functions, "getServerTime");

export const getExactServerTime = async () => {
    const startTime = Date.now();
    const serverTime = (await getServerTime()).data;
    const endTime = Date.now();
    const roundTripTime = endTime - startTime;
    return serverTime + roundTripTime / 2; // time when it returns, not when it was sent
};
