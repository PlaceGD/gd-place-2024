import {
    onAuthStateChanged,
    GoogleAuthProvider,
    signInWithPopup,
    type User,
    signOut as logOut,
    GithubAuthProvider,
    TwitterAuthProvider,
    inMemoryPersistence,
    setPersistence,
    browserLocalPersistence,
    type Unsubscribe,
} from "firebase/auth";

import type { HttpsCallableResult } from "firebase/functions";
import { writable, type Writable } from "svelte/store";
import { auth, db } from "./firebase";
import { initUserWithUsername } from "./cloud_functions";
import { loginData } from "../stores";
import Toast from "../utils/toast";
import type { DatabaseSchema } from "shared-lib/database";

const googleProvider = new GoogleAuthProvider();
const githubProvider = new GithubAuthProvider();
const twitterProvider = new TwitterAuthProvider();

export type PlaceData = DatabaseSchema["userData"][""];

export type UserData = {
    userData: User;
    placeData: PlaceData | null; // no user data
};

// export const currentUserData: Writable<UserData | null> = writable(null);
// export const currentUserDisplayColor: Writable<string> = writable(null);

export const signInGoogle = () => signInWithPopup(auth, googleProvider);
export const signInGithub = () => signInWithPopup(auth, githubProvider);
export const signInTwitter = () => signInWithPopup(auth, twitterProvider);

export const signOut = () => logOut(auth);

export const initUserData = (
    uid: string,
    username: string,
    turnstile: string
) => {
    return initUserWithUsername({
        uid,
        username,
        turnstileResp: turnstile,
    }) as Promise<HttpsCallableResult<PlaceData>>;
};

let userDataUnsub: Unsubscribe | null = null;
// let userDisplayColorListener: Unsubscribe | null = null;

// window.addEventListener("storage", async e => {
//     if (e.key?.match(/firebase:authUser:.*/)) {
//         const strData = localStorage.getItem(e.key);

//         console.log(strData);
//         if (strData == null) return;

//         try {
//             const newUserData: User = JSON.parse(strData);

//             console.log(newUserData);

//             const placeData = (
//                 await get(ref(db, `userData/${newUserData.uid}`))
//             ).val();

//             console.log(placeData);

//             loginData.update(data => {
//                 if (data.currentUserData != null) {
//                     data.isLoggedIn = true;
//                     data.currentUserData.userData = newUserData;
//                     data.currentUserData.placeData = placeData;
//                 }
//                 return data;
//             });
//         } catch {
//             return;
//         }
//     }
// });

onAuthStateChanged(auth, async user => {
    if (user != null) {
        let userDataValue: UserData = {
            userData: user,
            placeData: null,
        };

        loginData.update(data => {
            data.currentUserData = userDataValue;
            return data;
        });

        if (userDataUnsub != null) {
            userDataUnsub();
        }

        userDataUnsub = db.ref(`userData/${user.uid}`).on("value", snapshot => {
            const placeData = snapshot.val();

            loginData.update(data => {
                if (data.currentUserData != null) {
                    data.isLoggedIn = true;
                    data.currentUserData.placeData = placeData ?? null;
                } else {
                    console.error(
                        "User data set before user was created! (`onAuthStateChanged`)"
                    );
                    Toast.showErrorToast(
                        "There was an issue signing in. Please try again."
                    );
                }
                return data;
            });

            if (placeData != null) {
                setPersistence(auth, browserLocalPersistence);
            }
        });
    } else {
        setPersistence(auth, inMemoryPersistence);

        if (userDataUnsub != null) {
            userDataUnsub();
        }

        loginData.update(data => {
            data.isLoggedIn = false;
            data.currentUserData = null;
            return data;
        });
    }
});
