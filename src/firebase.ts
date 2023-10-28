import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import { getDatabase } from "firebase/database";

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

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);
const database = getDatabase(app);

console.log(database);
