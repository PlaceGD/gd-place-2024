import {
    getFunctions,
    httpsCallable as fHttpsCallable,
    type HttpsCallable,
    type FunctionsErrorCode,
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

type TypedHttpsCallable<
    RequestData,
    ResponseData = unknown,
> = _TypedHttpsCallable<HttpsCallable<RequestData, ResponseData>>;

const httpsCallable = <Req, Res = unknown>(
    ...args: Parameters<typeof fHttpsCallable>
): TypedHttpsCallable<Req, Res> => {
    return fHttpsCallable(...args) as TypedHttpsCallable<Req, Res>;
};

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
export const changeNameGradient = httpsCallable<GradientReq>(
    functions,
    "changeNameGradient"
);
export const setMeta = httpsCallable<MetaReq>(functions, "setMeta");

let randomKey = obj => {
    let keys = Object.keys(obj);
    return keys[Math.floor(keys.length * Math.random())];
};
let randInt = (min: number, max: number) => {
    let range = max - min + 1;
    return Math.floor(Math.random() * range) + min;
};
console.log("GUH", randomKey(objects));

window.honkyPonky = () => {
    let total = 0;
    while (total < 4000) {
        let obj = new GDObjectOpt(
            parseInt(randomKey(objects)),
            Math.random() * 40 * 30,
            Math.random() * 40 * 30,
            0,
            randInt(0, 72),
            0,
            randInt(0, 72),
            randInt(0, 8),
            randInt(-50, 50),
            new GDColor(255, 0, 0, 255, Math.random() >= 0.5),
            new GDColor(255, 0, 0, 255, Math.random() >= 0.5)
        );
        if (isValidObject(obj)) {
            total += 1;
            let v = obj.bytes();

            let s = encodeString(v, 126);

            placeObject({ object: s });
        }
        // console.log();
    }
};
