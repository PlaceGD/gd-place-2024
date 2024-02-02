import { database } from "firebase-admin";
import { HttpsError, onCall } from "firebase-functions/v2/https";
import { VALID_USERNAME } from "shared-lib";
import { Level, LogGroup } from "./utils/logger";
import { ref } from "./utils/database";

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
        banned: false,
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

    const userUid = (
        await ref(db, `userName/${data.username.toLowerCase()}`).get()
    ).val();

    if (userUid == undefined) {
        throw new HttpsError("invalid-argument", "Invalid username");
    }

    const reported = ref(db, `reportedUsers/${userUid.uid}`);

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

    if (data.operation == "ignore") {
        ref(db, `reportedUsers/${request.auth.uid}`).remove();
    } else if (data.operation == "ban") {
        ref(db, `userData/${request.auth.uid}/banned`).set(true);
    } else {
        throw new HttpsError("invalid-argument", "Unknown operation");
    }
});
