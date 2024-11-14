import { convertDatabase } from "@smart-firebase/admin";
import { getAuth } from "firebase-admin/auth";
import { onInit } from "firebase-functions/v2/core";
import Mailjet from "node-mailjet";
import { DatabaseSchema } from "shared-lib/database";
import pkg from "firebase-admin";
import { initializeApp } from "firebase-admin/app";
const { database } = pkg;

initializeApp();

export const auth = getAuth();

export const smartDatabase = () => convertDatabase<DatabaseSchema>(database());

// if (process.env["CLOUD_FUNCTIONS_ENV"] == "dev") {
//     emulate
// }

export let mailjetClient: Mailjet;
onInit(() => {
    mailjetClient = new Mailjet({
        apiKey: process.env["MAILJET_API_KEY"]!,
        apiSecret: process.env["MAILJET_API_SECRET"]!,
    });
});
