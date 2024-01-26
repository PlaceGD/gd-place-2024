import type { SvelteComponent } from "svelte";
import {
    signInGithub,
    signInGoogle,
    signInTwitter,
    type UserData,
} from "../firebase/auth";
import Toast from "../utils/toast";
import { get, ref } from "firebase/database";
import { db } from "../firebase/firebase";
import { loginData } from "../stores";

export enum SlideIds {
    LoginMethod = "LoginMethod",
    CreateUser = "CreateUser",
    TOS = "TOS",
}

export enum LoginMethod {
    Google = "Google",
    GitHub = "GitHub",
    X = "X",
}

let currentUser: UserData | null = null;

const logInSuccess = (user: any): boolean => {
    Toast.showSuccessToast("Signed in successfully!");
    // get(ref(db, `userData/${user.user.uid}`))
    //     .then(snapshot => {
    //         const placeData = snapshot.val();

    //         if(placeData !== null) {
    //             loginData.update((data) => {
    //                 data.currentUserData?.placeData = placeData;
    //                 return data
    //             })
    //         }

    //     })
    //     .catch(err => {
    //         Toast.showErrorToast(`Failed to get user data! (${err})`);
    //     });

    return true;
};

const logInFailed = (err: any): boolean => {
    console.error(err);
    Toast.showErrorToast(`Failed to login!`);

    return false;
};

export const handleSignOut = () => {};

export const handleSignIn = async (method: LoginMethod): Promise<boolean> => {
    switch (method) {
        case LoginMethod.Google:
            return signInGoogle().then(logInSuccess).catch(logInFailed);
        case LoginMethod.GitHub:
            return signInGithub().then(logInSuccess).catch(logInFailed);
        case LoginMethod.X:
            return signInTwitter().then(logInSuccess).catch(logInFailed);
    }
};
