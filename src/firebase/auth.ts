import { toast } from "@zerodevx/svelte-toast";
import type { FirebaseApp } from "firebase/app";
import {
    getAuth,
    onAuthStateChanged,
    GoogleAuthProvider,
    signInWithPopup,
    signInWithRedirect,
    type User,
    signOut as logOut,
    type UserCredential,
    GithubAuthProvider,
    signInWithEmailAndPassword,
    signInWithCredential,
    AuthCredential,
    signInWithCustomToken,
    EmailAuthCredential,
    EmailAuthProvider,
    type AuthProvider,
    TwitterAuthProvider,
    createUserWithEmailAndPassword,
    inMemoryPersistence,
    setPersistence,
    sendSignInLinkToEmail,
    browserLocalPersistence,
    isSignInWithEmailLink,
    signInWithEmailLink,
    onIdTokenChanged,
    type Unsubscribe,
} from "firebase/auth";

import { get, getDatabase, onValue, ref, set } from "firebase/database";
import { getFirestore, doc } from "firebase/firestore";
import type { HttpsCallableResult } from "firebase/functions";

// import { toastErrorTheme } from "../const";
import { derived, writable, type Writable } from "svelte/store";
import { auth, db } from "./firebase";
import { initUserWithUsername } from "./cloud_functions";

setPersistence(auth, inMemoryPersistence);

const googleProvider = new GoogleAuthProvider();
const githubProvider = new GithubAuthProvider();
const twitterProvider = new TwitterAuthProvider();

export type UserProperties = {
    username: string;
    lastPlaced: number;
    lastDeleted: number;
    placeTimer: number | null;
    deleteTimer: number | null;
};

export type UserData = {
    user: User;
    data: UserProperties | null; // no user data
};

export const currentUserData: Writable<UserData | null> = writable(null);
// export const currentUserDisplayColor: Writable<string> = writable(null);

export const signInGoogle = () => signInWithPopup(auth, googleProvider);
export const signInGithub = () => signInWithPopup(auth, githubProvider);
export const signInTwitter = () => signInWithPopup(auth, twitterProvider);

export const signOut = () => logOut(auth);

export const initUserData = (uid: string, username: string) => {
    return initUserWithUsername({ uid, username }) as Promise<
        HttpsCallableResult<UserProperties>
    >;
};

let userDataListener: Unsubscribe | null = null;
// let userDisplayColorListener: Unsubscribe | null = null;

onAuthStateChanged(auth, async user => {
    if (user != null) {
        let userDataValue = {
            user,
            data: null,
        };

        currentUserData.set(userDataValue);

        if (userDataListener != null) {
            userDataListener();
        }

        userDataListener = onValue(
            ref(db, `userData/${user.uid}`),
            snapshot => {
                userDataValue.data = snapshot.val();
                currentUserData.set(userDataValue);

                if (userDataValue.data !== null) {
                    setPersistence(auth, browserLocalPersistence);
                }
            }
        );
    } else {
        if (userDataListener != null) {
            userDataListener();
        }

        currentUserData.set(null);
    }
});
