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

import { onValue, ref } from "firebase/database";
import type { HttpsCallableResult } from "firebase/functions";
import { writable, type Writable } from "svelte/store";
import { auth, db } from "./firebase";
import { initUserWithUsername } from "./cloud_functions";

setPersistence(auth, inMemoryPersistence);

const googleProvider = new GoogleAuthProvider();
const githubProvider = new GithubAuthProvider();
const twitterProvider = new TwitterAuthProvider();

export type PlaceData = {
    username: string;
    lastPlaced: number;
    lastDeleted: number;
    placeTimer: number | null;
    deleteTimer: number | null;
};

export type UserData = {
    userData: User;
    placeData: PlaceData | null; // no user data
};

export const currentUserData: Writable<UserData | null> = writable(null);
// export const currentUserDisplayColor: Writable<string> = writable(null);

export const signInGoogle = () => signInWithPopup(auth, googleProvider);
export const signInGithub = () => signInWithPopup(auth, githubProvider);
export const signInTwitter = () => signInWithPopup(auth, twitterProvider);

export const signOut = () => logOut(auth);

export const initUserData = (uid: string, username: string) => {
    return initUserWithUsername({ uid, username }) as Promise<
        HttpsCallableResult<PlaceData>
    >;
};

let userDataUnsub: Unsubscribe | null = null;
// let userDisplayColorListener: Unsubscribe | null = null;

onAuthStateChanged(auth, async user => {
    if (user != null) {
        let userDataValue = {
            user,
            data: null,
        };

        currentUserData.set(userDataValue);

        if (userDataUnsub != null) {
            userDataUnsub();
        }

        userDataUnsub = onValue(ref(db, `userData/${user.uid}`), snapshot => {
            userDataValue.data = snapshot.val();
            currentUserData.set(userDataValue);

            if (userDataValue.data !== null) {
                setPersistence(auth, browserLocalPersistence);
            }
        });
    } else {
        if (userDataUnsub != null) {
            userDataUnsub();
        }

        currentUserData.set(null);
    }
});
