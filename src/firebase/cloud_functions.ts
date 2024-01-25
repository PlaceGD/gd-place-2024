import { getFunctions, httpsCallable } from "firebase/functions";

const functions = getFunctions();
export const placeObject = httpsCallable(functions, "placeObject");
export const deleteObject = httpsCallable(functions, "deleteObject");
export const initUserWithUsername = httpsCallable(
    functions,
    "initUserWithUsername"
);
