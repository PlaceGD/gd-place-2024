import { HttpsError as fHttpsError } from "firebase-functions/v2/https";
import { FIREBASE_ERRORS } from "shared-lib/cloud_functions";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export type HttpsError<_T> = fHttpsError;

type FirebaseErrorKindMap = {
    "1": "invalid-argument";
    "2": "permission-denied";
    "3": "already-exists";
    "4": "invalid-argument";
    "5": "aborted";
    "6": "resource-exhausted";
};

type FirebaseErrorKind<T extends keyof typeof FIREBASE_ERRORS> =
    `${T}` extends `${infer K}${number}${number}`
        ? FirebaseErrorKindMap[K & keyof FirebaseErrorKindMap]
        : never;

export default class Error {
    static code<T extends keyof typeof FIREBASE_ERRORS>(
        code: T,
        kind: FirebaseErrorKind<T>
    ): HttpsError<(typeof FIREBASE_ERRORS)[T]["message"]> {
        return new fHttpsError(kind, FIREBASE_ERRORS[code].message, {
            code: code as unknown,
            message: FIREBASE_ERRORS[code].message,
        });
    }
}
