import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import { connectDatabaseEmulator, getDatabase } from "firebase/database";
import Toast from "../utils/toast";
import { getAuth } from "firebase/auth";
import { convertDatabase } from "@smart-firebase/client";
import { type DatabaseSchema } from "shared-lib/database";

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

const db_ = getDatabase(app);
if (typeof window !== "undefined" && __USE_DB === "local") {
    connectDatabaseEmulator(db_, "127.0.0.1", 9000);
}

export const db = convertDatabase<DatabaseSchema>(db_);
