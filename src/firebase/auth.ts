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
    signInWithRedirect,
} from "firebase/auth";

import type { HttpsCallableResult } from "firebase/functions";
import { writable, type Writable } from "svelte/store";
import { auth, db } from "./firebase";
import { initUserWithUsername } from "./cloud_functions";
import { loginData } from "../stores";
import Toast from "../utils/toast";
import type { DatabaseSchema, UserDetails } from "shared-lib/database";

const googleProvider = new GoogleAuthProvider();
const githubProvider = new GithubAuthProvider();
const twitterProvider = new TwitterAuthProvider();

export type UserData = {
    user: User;
    userDetails: UserDetails | null; // no user data
};

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
    });
};

let userDataUnsub: Unsubscribe | null = null;

onAuthStateChanged(auth, async user => {
    if (typeof window !== "undefined") {
        localStorage.setItem(
            "authState",
            user === null ? "-1" : Math.random().toString()
        );
    }

    if (user != null) {
        let userDataValue: UserData = {
            user: user,
            userDetails: null,
        };

        loginData.update(data => {
            data.currentUserData = userDataValue;
            return data;
        });

        if (userDataUnsub != null) {
            userDataUnsub();
        }

        userDataUnsub = db
            .ref(`userDetails/${user.uid}`)
            .on("value", snapshot => {
                const placeData = snapshot.val();

                loginData.update(data => {
                    if (data.currentUserData != null) {
                        data.currentUserData.userDetails = placeData ?? null;
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
            data.currentUserData = null;
            return data;
        });
    }
});
