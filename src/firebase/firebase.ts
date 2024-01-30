import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import {
    getDatabase,
    ref,
    onValue,
    onChildAdded,
    onChildRemoved,
    push,
    DataSnapshot,
    type Unsubscribe,
    set,
    remove,
    get,
} from "firebase/database";
import Toast from "../utils/toast";
import { getAuth } from "firebase/auth";

const firebaseConfig = {
    apiKey: "AIzaSyB9PSVZzg5WOp26PuCkVrrSTVrWg-XJMgg",
    authDomain: "gd-place-2023.firebaseapp.com",
    databaseURL: "https://gd-place-2023-default-rtdb.firebaseio.com",
    projectId: "gd-place-2023",
    storageBucket: "gd-place-2023.appspot.com",
    messagingSenderId: "358180840785",
    appId: "1:358180840785:web:c0c2c306234f2fe9de5f70",
    measurementId: "G-05Q7TVNRLM",
};

export const app = initializeApp(firebaseConfig);
export const auth = getAuth(app);
// const analytics = getAnalytics(app);
export const db = getDatabase(app);

// console.log((await get(ref(db, "dog"))).val());
