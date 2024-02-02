import { database } from "firebase-admin";
import { HttpsError, onCall } from "firebase-functions/v2/https";
import { VALID_USERNAME } from "shared-lib";
import { Level, LogGroup } from "./utils/logger";
import { ref } from "./utils/database";
import { DEV_UIDS } from ".";
import { Database } from "firebase-admin/database";

type RequestData = {
    username: string;
    uid: string;
};

export const initUserWithUsername = onCall<RequestData>(async request => {
    const logger = new LogGroup("initUserWithUsername");
    logger.info(
        "Is user authed?",
        request.auth != null,
        `UID: ${request.auth?.uid}`
    );

    if (!request.auth) {
        logger.finish(Level.ERROR);
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const data = request.data;

    logger.debug("Username:", data.username);

    if (!data.username.match(VALID_USERNAME)) {
        logger.finish(Level.ERROR);
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
        logger.error("User already exists");
        logger.finish(Level.ERROR);
        throw new HttpsError("already-exists", "Username already exists");
    }

    const user = {
        username: data.username,
        lastPlaced: 0,
        lastDeleted: 0,
        moderator: false,
    };

    logger.info("User created sucessfully");

    // make new user
    ref(db, `userData/${data.uid}`).set(user);

    ref(db, `userName/${data.username.toLowerCase()}`).set({
        uid: data.uid,
    });

    ref(db, "userCount").transaction(count => {
        return count + 1;
    });

    logger.finish();

    return user;
});

type ReportData = {
    username: string;
};

export const reportUser = onCall<ReportData>(async request => {
    const logger = new LogGroup("reportUser");
    logger.info(
        "Is user authed?",
        request.auth != null,
        `UID: ${request.auth?.uid}`
    );

    if (!request.auth) {
        logger.finish(Level.ERROR);
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const db = database();
    const data = request.data;

    const userToReport = (
        await ref(db, `userName/${data.username.toLowerCase()}`).get()
    ).val();

    if (userToReport == undefined) {
        throw new HttpsError("invalid-argument", "Invalid username");
    }

    const reported = ref(db, `reportedUsers/${userToReport.uid}`);

    reported.transaction(data => {
        logger.debug(data);
        if (data == undefined) {
            logger.info("User has been reported 1 time");
            return {
                username: request.data.username,
                count: 1,
            };
        } else {
            data.count += 1;
            logger.info(`User has been reported ${data.count} times`);
            return data;
        }
    });

    logger.finish();
});

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

    ref(db, `userData/${userToBanUid}`).remove();
    ref(db, "userCount").transaction(count => {
        return count - 1;
    });
};

type OperationData = {
    operation: "ignore" | "ban";
    reportedUserUid: string;
};

export const reportedUserOperation = onCall<OperationData>(async request => {
    if (!request.auth) {
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const db = database();

    const modData = (await ref(db, `userData/${request.auth.uid}`).get()).val();
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

    ref(db, `reportedUsers/${data.reportedUserUid}`).remove();
});

type BanRequestData = {
    username: string;
};

export const banUser = onCall<BanRequestData>(async request => {
    if (!request.auth) {
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const logger = new LogGroup("banUser");

    const db = database();

    const modData = (await ref(db, `userData/${request.auth.uid}`).get()).val();
    if (modData == null) {
        logger.finish(Level.ERROR);
        throw new HttpsError("invalid-argument", "Invalid mod UID");
    }

    if (!modData.moderator) {
        logger.debug("User is not mod");
        logger.finish(Level.ERROR);
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

    logger.info("User id:", userToBanUid);

    if (userToBanUid == null) {
        logger.finish(Level.ERROR);
        throw new HttpsError("invalid-argument", "Unknown user");
    }

    await banUserInner(db, request.auth.uid, userToBanUid);

    logger.finish();
});
