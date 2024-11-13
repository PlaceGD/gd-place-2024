import { initializeApp, type FirebaseApp } from "firebase/app";
import {
    connectDatabaseEmulator,
    Database,
    getDatabase,
} from "firebase/database";
import { connectAuthEmulator, getAuth, type Auth } from "firebase/auth";
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
export const app = initializeApp(firebaseConfig, "gd-place-2023");

/*
`npm run vite:dev-local` -> spawn up dev server using firebase emulators
`npm run vite:dev-live` -> spawn up dev server using dev database (not emulated)

`npm run deploy` -> deploys cloud functions and hosting to dev project and uses dev database (gd-place-2023-dev, gd-place-2023-dev.web.app)
`npm run deploy:prod` -> deploys cloud functions and hosting to prod project and uses prod database (gd-place-2023, place.gd)

all auth is from gd-place-2023 project

`gd-place-2023.web.app` is no longer deployed to. `place.gd` is prod and `gd-place-2023-dev.web.app` is dev
*/

// the app used by the cloud functions
export let cfapp: FirebaseApp;
export let auth: Auth;

let db_: Database;
if (__CLOUD_FUNCTIONS_ENV === "dev") {
    // use dev cloud functions and database
    const firebaseConfig = {
        apiKey: "AIzaSyBfpykGgifWO82srOPieOU26IwdDV_XJ2E",
        authDomain: "gd-place-2023-dev.firebaseapp.com",
        databaseURL: "https://gd-place-2023-dev-default-rtdb.firebaseio.com",
        projectId: "gd-place-2023-dev",
        storageBucket: "gd-place-2023-dev.firebasestorage.app",
        messagingSenderId: "935041808954",
        appId: "1:935041808954:web:6c81d761fb93cb9701d303",
    };

    cfapp = initializeApp(firebaseConfig, "gd-place-2023-dev");
    db_ = getDatabase(cfapp);
    auth = getAuth(cfapp);
} else if (__CLOUD_FUNCTIONS_ENV === "prod") {
    // use prod cloud functions and database
    cfapp = app;
    db_ = getDatabase(app);
    auth = getAuth(app);
} else {
    throw new Error("__CLOUD_FUNCTIONS_ENV not set");
}

if (typeof window !== "undefined" && __RT_DB_ENV === "local") {
    connectDatabaseEmulator(db_, "127.0.0.1", 9000);
    connectAuthEmulator(auth, "http://127.0.0.1:8000");
}

export const db = convertDatabase<DatabaseSchema>(db_);
