import { getFunctions, httpsCallable } from "firebase/functions";

const functions = getFunctions();
export const helloWorld = httpsCallable(functions, "helloWorld");
