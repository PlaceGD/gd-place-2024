import { database } from "firebase-admin";
import { HttpsError, onCall } from "firebase-functions/v2/https";

type RequestData = {
    username: string;
    uid: string;
};

export const initUserWithUsername = onCall<RequestData>(async request => {
    if (!request.auth) {
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const data = request.data;

    if (!data.username.match(/^[A-Za-z0-9_-]{3,16}$/)) {
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
        throw new HttpsError("already-exists", "Username already exists");
    }

    let user = {
        username: data.username,
        lastPlaced: 0,
        lastDeleted: 0,
    };

    // make new user
    db.ref(`/userData/${data.uid}`).set(user);

    db.ref(`/userName/${data.username.toLowerCase()}`).set({
        uid: data.uid,
    });

    db.ref("/userCount").transaction(count => {
        return count + 1;
    });

    return user;
});
