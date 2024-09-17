import { HttpsError } from "firebase-functions/v2/https";
import { type FirebaseError as FirebaseErrors } from "shared-lib/cloud_functions";

export const FIREBASE_ERRORS = {
    [100]: {
        code: 100,
        message: "Invalid username",
    },
    [101]: {
        code: 101,
        message: "Invalid report position",
    },
    [102]: {
        code: 102,
        message: "Invalid user ID",
    },
    [103]: {
        code: 103,
        message: "Invalid mod ID",
    },
    [104]: {
        code: 104,
        message: "Invalid transaction ID",
    },
    [105]: {
        code: 105,
        message: "Invalid gradient",
    },
    [106]: {
        code: 106,
        message: "Invalid object string",
    },
    [107]: {
        code: 107,
        message: "Invalid transaction ID",
    },
    // 200 - permission denied
    [200]: {
        code: 200,
        message: "User is not permitted to perform this operation",
    },
    [201]: {
        code: 201,
        message: "Banned",
    },
    [202]: {
        code: 202,
        message: "Cannot place before timer expired",
    },
    [203]: {
        code: 203,
        message: "Cannot delete before timer expired",
    },
    [204]: {
        code: 204,
        message: "Cannot report before timer expired",
    },
    [205]: {
        code: 205,
        message: "Cannot change gradient before timer expired",
    },
    [206]: {
        code: 206,
        message:
            "Cannot ban a moderator. Please contact a developer if there is an issue.",
    },
    [207]: {
        code: 207,
        message: "Cannot change gradient of user without donation",
    },
    [208]: {
        code: 208,
        message: "Cannot place before event starts",
    },
    [209]: {
        code: 209,
        message: "Cannot delete before event starts",
    },
    // 300 - already exists
    [300]: {
        code: 300,
        message: "Username already exists",
    },
    [301]: {
        code: 301,
        message: "User data already exists",
    },
    // 400 - missing data
    [400]: {
        code: 400,
        message: "Missing object string",
    },
    [401]: {
        code: 401,
        message: "Missing object id",
    },
    [402]: {
        code: 402,
        message: "Missing object key",
    },
    // 500 - other
    [500]: {
        code: 500,
        message: "Unknown user operation",
    },
    // 600 - exhausted
    [600]: {
        code: 600,
        message: "Too many objects in chunk",
    },
} as const;

export type FirebaseError = {
    kind:
        | "invalid-argument"
        | "permission-denied"
        | "already-exists"
        | "unknown";
    error: (typeof FIREBASE_ERRORS)[keyof typeof FIREBASE_ERRORS];
};

export class Error {
    static code<T extends keyof typeof FIREBASE_ERRORS>(
        code: T,
        kind: FirebaseError["kind"]
    ): HttpsError {
        // return new HttpsError(kind, {
        //     code: code as any,
        //     message: FIREBASE_ERRORS[code].message,
        // });
    }
}

// const x = () => {
//     FirebaseError.code(x);
// };
