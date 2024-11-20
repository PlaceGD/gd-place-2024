import { db } from "./firebase";

let signupDateCache: Record<string, number | undefined> = {};

export const getSignupTimestamp = (
    username: string,
    cb: (date: number | undefined) => void
) => {
    if (signupDateCache[username] != null) {
        cb(signupDateCache[username]);
    } else {
        db.ref(`userName/${username}`)
            .get()
            .then(s => {
                let uid = s.val()?.uid;

                db.ref(`userDetails/${uid}/signupdate`)
                    .get()
                    .then(s => {
                        let date = s.val();
                        signupDateCache[username] = date;
                        cb(date);
                    });
            });
    }
};
