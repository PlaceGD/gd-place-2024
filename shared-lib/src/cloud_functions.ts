import { ChunkID } from "./database.js";
import { type KofiTxId } from "./kofi.js";

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
    [108]: {
        code: 108,
        message: "Invalid level name letter",
    },
    [109]: {
        code: 109,
        message: "Invalid level name index",
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
        message: "Cannot place at current time",
    },
    [209]: {
        code: 209,
        message: "Cannot delete at current time",
    },
    [210]: {
        code: 210,
        message: "User is not authenticated",
    },
    [211]: {
        code: 211,
        message: "Cannot change letter at current time",
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
        message: "Missing object ID",
    },
    [402]: {
        code: 402,
        message: "Missing object key",
    },
    [403]: {
        code: 403,
        message: "Missing chunk ID",
    },
    [404]: {
        code: 403,
        message: "Missing user data",
    },
    // 500 - other
    [500]: {
        code: 500,
        message: "Unknown operation",
    },
    [501]: {
        code: 501,
        message: "Transaction not commited",
    },
    // 600 - exhausted
    [600]: {
        code: 600,
        message: "Too many objects in chunk",
    },
} as const;

export type FirebaseError =
    (typeof FIREBASE_ERRORS)[keyof typeof FIREBASE_ERRORS];

// TODO: add our uids
export const DEV_UIDS: string[] = [
    "mPjKp1BVO2T5sIZa0sesbP9izde2",
    "ePRWGbc6NtW5opp0T3n8xQVr8TW2",
    "LtbULkrh8sRas6y52oieUlApCLo1",
];

export type PlaceReq = { object: string };
export type PlaceRes = { key: string; cooldown: number };

export type DeleteReq = { chunkId: ChunkID; objId: string };
export type DeleteRes = { cooldown: number };

export type InitWithUsernameReq = {
    username: string;
    uid: string;
    turnstileResp: string;
};

export type ReportUserReq = {
    username: string;
    // turnstileResp: string;
    x: number;
    y: number;
};
export type ReportUserRes = { cooldown: number };

export type ReportedUserOperationReq =
    | {
          operation: "ban";
          reason: string;
          userUid: string;
          reportKeys: string[];
      }
    | {
          operation: "ignore";
          reportKeys: string[];
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
export type GradientRes = { cooldown: number };

export type MetaOperation =
    | {
          type: "place_timer";
          to: number;
      }
    | {
          type: "delete_timer";
          to: number;
      }
    | {
          type: "event_start";
          to: number;
      }
    | {
          type: "event_end";
          to: number;
      }
    | {
          type: "name_duration";
          duration: number;
      }
    | {
          type: "postpone_start";
          secs: number;
      }
    | {
          type: "postpone_end";
          secs: number;
      }
    | {
          type: "change_mod_status";
          user: string;
          to: "mod" | "unmod";
      }
    | {
          type: "unban";
          user: string;
      }
    | {
          type: "announcement";
          text: string;
      }
    | {
          type: "clear_announcement";
      }
    | {
          type: "to_username";
          uid: string;
      }
    | {
          type: "log_donation";
          uid: string;
      };

export type MetaReq = {
    op: MetaOperation;
};

export type LevelNameReq = {
    letter: string;
    index: number;
};
export type LevelNameRes = {
    cooldown: number;
};
