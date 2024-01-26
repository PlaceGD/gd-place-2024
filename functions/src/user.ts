import { database } from "firebase-admin";
import { HttpsError, onCall } from "firebase-functions/v2/https";
import { VALID_USERNAME } from "shared-lib";
import { Level, LogGroup } from "./logger";

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

    const usernameExists = db.ref(`/userName/${data.username.toLowerCase()}`);

    // TODO: check username for bad words
    // BAD_WORDS.forEach(word => {
    //     if (data.username.toLowerCase().includes(word)) {
    //         throw new HttpsError("invalid-argument", "Invalid username");
    //     }
    // });

    const val = (await usernameExists.get()).val();
    if (val != null) {
        logger.error("User already exists");
        logger.finish(Level.ERROR);
        throw new HttpsError("already-exists", "Username already exists");
    }

    let user = {
        username: data.username,
        lastPlaced: 0,
        lastDeleted: 0,
    };

    logger.info("User created sucessfully");

    // make new user
    db.ref(`/userData/${data.uid}`).set(user);

    db.ref(`/userName/${data.username.toLowerCase()}`).set({
        uid: data.uid,
    });

    db.ref("/userCount").transaction(count => {
        return count + 1;
    });

    logger.finish();

    return user;
});
