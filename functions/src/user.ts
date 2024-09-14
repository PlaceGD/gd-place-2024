import { HttpsError, onCall, Request } from "firebase-functions/v2/https";
import { onMessagePublished } from "firebase-functions/v2/pubsub";
import { REPORT_COOLDOWN_SECONDS, VALID_USERNAME } from "shared-lib/user";
import type {
    InitWithUsernameReq,
    ReportUserReq,
    ReportedUserOperationReq,
    BanReq,
} from "shared-lib/cloud_functions";
import { Level, LogGroup } from "./utils/logger";
import { DEV_UIDS, LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from ".";
import { onCallAuth, onCallAuthLogger } from "./utils/on_call";
import { UserDetails } from "shared-lib/database";
import { smartDatabase } from "src";
import { getCheckedUserDetails } from "./utils/utils";

const validateTurnstile = async (
    key: string | undefined,
    token: string,
    request: Request,
    logger: LogGroup
) => {
    if (key == undefined) {
        throw new HttpsError(
            "permission-denied",
            "Missing turnstile key! Please contact a developer."
        );
    }

    const ip = request.get("CF-Connecting-IP");

    let formData = new FormData();
    formData.append("secret", key);
    formData.append("response", token!);
    formData.append("remoteip", ip!);

    const url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
    const result = await fetch(url, {
        body: formData,
        method: "POST",
    });

    const outcome = await result.json();

    logger.info("Turnstile outcome:", JSON.stringify(outcome));

    if (!outcome.success) {
        throw new HttpsError(
            "permission-denied",
            "Something went wrong. Please try again."
        );
    }
};

export const initUserWithUsername = onCallAuthLogger<
    InitWithUsernameReq,
    Promise<UserDetails>
>("initUserWithUsername", async (request, logger) => {
    const data = request.data;

    await validateTurnstile(
        process.env["TURNSTILE_LOGIN_PRIV_KEY"],
        data.turnstileResp,
        request.rawRequest,
        logger
    );

    logger.debug("Username:", data.username);
    if (!data.username.match(VALID_USERNAME)) {
        throw new HttpsError("invalid-argument", "Username is invalid");
    }

    const db = smartDatabase();

    // TODO: check username for bad words
    // BAD_WORDS.forEach(word => {
    //     if (data.username.toLowerCase().includes(word)) {
    //         throw new HttpsError("invalid-argument", "Invalid username");
    //     }
    // });

    const usernameExists = (
        await db.ref(`userName/${data.username.toLowerCase()}`).get()
    ).val();

    if (usernameExists != undefined) {
        logger.error("Username already exists");
        throw new HttpsError("already-exists", "Username already exists");
    }

    const maybeUserData = db.ref(`userDetails/${data.uid}`);

    // if ((await maybeUserData.get()) != null) {
    //     logger.error("User data already exists");
    //     throw new HttpsError("already-exists", "User data already exists");
    // }

    const isBanned = (await db.ref(`bannedUsers/${data.uid}`).get()).val();
    if (isBanned === true) {
        throw new HttpsError("permission-denied", "Banned");
    }

    const user: UserDetails = {
        username: data.username,
        epochNextPlace: 0,
        epochNextDelete: 0,
        epochNextReport: 0,
        epochNextGradient: 0,
        moderator: false,
        hasDonated: false,
    };

    logger.info("User created sucessfully");

    // make new user
    maybeUserData.set(user);

    db.ref(`userName/${data.username.toLowerCase()}`).set({
        uid: data.uid,
        displayColor: "white",
    });

    db.ref("userCount").transaction(count => {
        return count + 1;
    });

    return user;
});

export const reportUser = onCallAuthLogger<ReportUserReq>(
    "reportUser",
    async (request, logger) => {
        const data = request.data;

        await validateTurnstile(
            process.env["TURNSTILE_GENERAL_PRIV_KEY"],
            data.turnstileResp,
            request.rawRequest,
            logger
        );

        const db = smartDatabase();

        const { userDetails, userDetailsRef } = await getCheckedUserDetails(
            db,
            request.auth.uid
        );

        let transactionResult1 = await userDetailsRef
            .child("epochNextReport")
            .transaction(nextReport => {
                if (Date.now() < nextReport ?? 0) {
                    throw new HttpsError(
                        "permission-denied",
                        "Cannot report before timer expired"
                    );
                }

                return Date.now() + REPORT_COOLDOWN_SECONDS * 1000;
            });
        if (!transactionResult1.committed) {
            logger.debug("Transaction 1 not committed");
            return;
        }

        const userToReport = (
            await db.ref(`userName/${data.username.toLowerCase()}`).get()
        ).val();

        if (userToReport == undefined) {
            throw new HttpsError("invalid-argument", "Invalid username");
        }

        if (data.x < 0 || data.x > LEVEL_WIDTH_UNITS) {
            throw new HttpsError("invalid-argument", "Invalid X");
        }
        if (data.y < 0 || data.y > LEVEL_HEIGHT_UNITS) {
            throw new HttpsError("invalid-argument", "Invalid Y");
        }

        // const reported = db.ref("reportedUsers");

        let transactionResult2 = await db
            .ref("reportedUsers")
            .child(userToReport.uid)
            .transaction(reportData => {
                if (reportData == undefined) {
                    return {
                        username: data.username,
                        timestamp: Date.now(),
                        count: 1,
                        avg_x: data.x,
                        avg_y: data.y,
                    };
                } else {
                    return {
                        username: data.username,
                        timestamp: Date.now(),
                        count: reportData.count + 1,
                        avg_x:
                            (reportData.avg_x * reportData.count + data.x) /
                            (reportData.count + 1),
                        avg_y:
                            (reportData.avg_y * reportData.count + data.y) /
                            (reportData.count + 1),
                    };
                }
            });
        if (!transactionResult2.committed) {
            logger.debug("Transaction 2 not committed");
            return;
        }
        logger.info(`Reported user ${data.username}`);
    }
);

const banUserInner = async (
    db: ReturnType<typeof smartDatabase>,
    requesterUid: string,
    userToBanUid: string
) => {
    const isBanned = (await db.ref(`bannedUsers/${requesterUid}`).get()).val();
    if (isBanned === true) {
        throw new HttpsError("permission-denied", "Banned");
    }

    const userToBanDetailsRef = db.ref(`userDetails/${userToBanUid}`);
    const userToBanDetails = (await userToBanDetailsRef.get()).val();
    if (userToBanDetails == null) {
        throw new HttpsError("invalid-argument", "Invalid user UID");
    }

    if (!DEV_UIDS.includes(requesterUid)) {
        const reportedUserIsMod = (
            await userToBanDetailsRef.child("moderator").get()
        ).val();

        if (reportedUserIsMod) {
            throw new HttpsError(
                "permission-denied",
                "Cannot ban a moderator. Please contact a developer if there is an issue."
            );
        }
    }

    db.ref(`bannedUsers/${userToBanUid}`).set(true);
    db.ref("userCount").transaction(count => {
        return count - 1;
    });
};

// projects/gd-place-2023/topics/clearOldReports
export const clearReports = onMessagePublished("clearOldReports", _ => {
    const now = Date.now();
    const db = smartDatabase();

    db.ref("reportedUsers")
        .orderByChild("timestamp")
        .endAt(now - 15 * 60 * 1000)
        .get()
        .then(snapshot => {
            snapshot.forEach(child => {
                child.ref().remove();
            });
        });
});

export const reportedUserOperation = onCallAuth<ReportedUserOperationReq>(
    async request => {
        const db = smartDatabase();

        const modData = (
            await db.ref(`userDetails/${request.auth.uid}`).get()
        ).val();
        if (modData == null) {
            throw new HttpsError("invalid-argument", "Invalid mod UID");
        }

        if (!modData.moderator) {
            throw new HttpsError(
                "permission-denied",
                "User is not permitted to perform this operation"
            );
        }

        const data = request.data;

        if (data.operation == "ban") {
            await banUserInner(db, request.auth.uid, data.reportedUserUid);
        } else if (data.operation == "ignore") {
            await db.ref(`reportedUsers/${data.reportedUserUid}`).remove();
        } else {
            throw new HttpsError("invalid-argument", "Unknown operation");
        }
    }
);

export const banUser = onCallAuthLogger<BanReq>(
    "banUser",
    async (request, logger) => {
        const db = smartDatabase();

        const modData = (
            await db.ref(`userDetails/${request.auth.uid}`).get()
        ).val();
        if (modData == null) {
            throw new HttpsError("invalid-argument", "Invalid mod UID");
        }

        if (!modData.moderator) {
            logger.debug("User is not mod");
            throw new HttpsError(
                "permission-denied",
                "User is not permitted to perform this operation"
            );
        }

        const data = request.data;

        logger.info("Username:", data.username, data.username.toLowerCase());

        const userToBanUid = (
            await db.ref(`userName/${data.username.toLowerCase()}`).get()
        ).val()?.uid;

        if (userToBanUid == null) {
            throw new HttpsError("invalid-argument", "Unknown user");
        }

        logger.info("User id:", userToBanUid);

        await banUserInner(db, request.auth.uid, userToBanUid);
    }
);
