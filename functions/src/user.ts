import { database } from "firebase-admin";
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
import { ref } from "./utils/database";
import { DEV_UIDS, LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from ".";
import { Database } from "firebase-admin/database";
import { onCallAuth, onCallAuthLogger } from "./utils/on_call";
import { UserData } from "shared-lib/database";

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
    Promise<UserData>
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

    const db = database();

    // TODO: check username for bad words
    // BAD_WORDS.forEach(word => {
    //     if (data.username.toLowerCase().includes(word)) {
    //         throw new HttpsError("invalid-argument", "Invalid username");
    //     }
    // });

    const usernameExists = (
        await ref(db, `userName/${data.username.toLowerCase()}`).get()
    ).val();

    if (usernameExists != undefined) {
        logger.error("Username already exists");
        throw new HttpsError("already-exists", "Username already exists");
    }

    const maybeUserData = ref(db, `userData/${data.uid}`);

    if ((await maybeUserData.get()) != null) {
        logger.error("User data already exists");
        throw new HttpsError("already-exists", "User data already exists");
    }

    const isBanned = (await ref(db, `bannedUsers/${data.uid}`).get()).val();
    if (isBanned === 1) {
        throw new HttpsError("permission-denied", "Banned");
    }

    const user: UserData = {
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

    ref(db, `userName/${data.username.toLowerCase()}`).set({
        uid: data.uid,
        displayColor: "white",
    });

    ref(db, "userCount").transaction(count => {
        return count + 1;
    });

    return user;
});

export const reportUser = onCallAuthLogger<ReportUserReq>(
    "reportUser",
    async (request, logger) => {
        const data = request.data;

        await validateTurnstile(
            process.env["TURNSTILE_LOGIN_PRIV_KEY"],
            data.turnstileResp,
            request.rawRequest,
            logger
        );

        const db = database();

        const isBanned = (
            await ref(db, `bannedUsers/${request.auth.uid}`).get()
        ).val();
        if (isBanned === 1) {
            throw new HttpsError("permission-denied", "Banned");
        }

        const timeNextReport = (
            await ref(db, `userData/${request.auth.uid}/epochNextReport`).get()
        ).val();

        if (timeNextReport == undefined) {
            throw new HttpsError("invalid-argument", "Missing report timer");
        }

        if (Date.now() < timeNextReport) {
            throw new HttpsError(
                "permission-denied",
                "Cannot report before timer expired"
            );
        }

        const userToReport = (
            await ref(db, `userName/${data.username.toLowerCase()}`).get()
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

        const reported = ref(db, `reportedUsers`);

        logger.info(`Reported user ${data.username}`);

        reported.push({
            uid: userToReport.uid,
            username: data.username,
            timestamp: Date.now(),
            x: data.x,
            y: data.y,
        });

        let nextReport = Date.now();
        nextReport += REPORT_COOLDOWN_SECONDS * 1000;

        ref(db, `userData/${request.auth.uid}/epochNextReport`).set(nextReport);
    }
);

const banUserInner = async (
    db: Database,
    requesterUid: string,
    userToBanUid: string
) => {
    if (!DEV_UIDS.includes(requesterUid)) {
        const reportedUserIsMod = (
            await ref(db, `userData/${userToBanUid}/moderator`).get()
        ).val();

        if (reportedUserIsMod) {
            throw new HttpsError(
                "permission-denied",
                "Cannot ban a moderator. Please contact a developer if there is an issue."
            );
        }
    }

    const userData = (await ref(db, `userData/${userToBanUid}`).get()).val();

    if (userData == null) {
        throw new HttpsError("invalid-argument", "Invalid user UID");
    }

    ref(db, `bannedUsers/${userToBanUid}`).set(1);
    ref(db, "userCount").transaction(count => {
        return count - 1;
    });
};

// TODO: schedule delete old reports!!
// const clearReportsOfUser = (db: Database, reportedUserUid: string) => {

// };

// // projects/gd-place-2023/topics/clearOldReports
// export const clearReports = onMessagePublished("clearOldReports", event => {

// });

export const reportedUserOperation = onCallAuth<ReportedUserOperationReq>(
    async request => {
        const db = database();

        const modData = (
            await ref(db, `userData/${request.auth.uid}`).get()
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
        } else if (data.operation != "ignore") {
            throw new HttpsError("invalid-argument", "Unknown operation");
        }

        // TODO: clear reports
        //ref(db, `reportedUsers/${data.reportedUserUid}`).remove();
    }
);

export const banUser = onCallAuthLogger<BanReq>(
    "banUser",
    async (request, logger) => {
        const db = database();

        const modData = (
            await ref(db, `userData/${request.auth.uid}`).get()
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
            await ref(db, `userName/${data.username.toLowerCase()}`).get()
        ).val()?.uid;

        if (userToBanUid == null) {
            throw new HttpsError("invalid-argument", "Unknown user");
        }

        logger.info("User id:", userToBanUid);

        await banUserInner(db, request.auth.uid, userToBanUid);
    }
);
