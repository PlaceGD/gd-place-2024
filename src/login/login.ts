import type { SvelteComponent } from "svelte";
import {
    signInGithub,
    signInGoogle,
    signInTwitter,
    signOut,
} from "../firebase/auth";
import Toast from "../utils/toast";
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

const logInSuccess = (user: any): boolean => {
    Toast.showSuccessToast("Signed in successfully!");
    // get(ref(db, `userDetails/${user.user.uid}`))
    //     .then(snapshot => {
    //         const placeData = snapshot.val();

    //         if(placeData != null) {
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

export const handleSignOut = () => {
    if (confirm("Are you sure you want to sign out?")) {
        signOut()
            .then(() => {
                Toast.showSuccessToast("Successfully logged out!");
            })
            .catch(err => {
                console.error(err);
                Toast.showErrorToast("Failed to log out!");
            });
    }
};

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
