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
import { loginData } from "../stores";
import Toast from "../utils/toast";
import type { DatabaseSchema } from "shared-lib";

setPersistence(auth, inMemoryPersistence);

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

export const initUserData = (uid: string, username: string) => {
    return initUserWithUsername({ uid, username }) as Promise<
        HttpsCallableResult<PlaceData>
    >;
};

let userDataUnsub: Unsubscribe | null = null;
// let userDisplayColorListener: Unsubscribe | null = null;

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

        userDataUnsub = onValue(ref(db, `userData/${user.uid}`), snapshot => {
            const placeData = snapshot.val();

            console.log(placeData);

            loginData.update(data => {
                if (data.currentUserData !== null) {
                    data.isLoggedIn = true;
                    data.currentUserData.placeData = placeData;
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

            if (placeData !== null) {
                setPersistence(auth, browserLocalPersistence);
            }
        });
    } else {
        if (userDataUnsub != null) {
            userDataUnsub();
        }

        loginData.update(data => {
            data.currentUserData = null;
            return data;
        });
    }
});
