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

export type LoginData = {
    isLoggedIn: boolean;
    showLoginUI: boolean;
};

export enum LoginMethod {
    Google = "Google",
    GitHub = "GitHub",
    X = "X",
}

export type Component = new (...args: any[]) => SvelteComponent;
export type ComponentWithProps = {
    component: Component;
    props?: any;
    showBackButton?: boolean;
    showCloseButton?: boolean;
};

export type SliderMethods = {
    previous: () => void;
    addSlideAndMove: (slide: ComponentWithProps) => void;
    setInteractability: (interact: boolean) => void;
};

let currentUser: UserData | null = null;

const logInSuccess = (user: any): boolean => {
    get(ref(db, `userData/${user.user.uid}`))
        .then(snapshot => {
            currentUser = snapshot.val();

            Toast.showSuccessToast("Signed in successfully!");

            if (currentUser === null) {
            } else {
                // todo
            }
        })
        .catch(err => {
            Toast.showErrorToast(`Failed to get user data! (${err})`);
        });

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
