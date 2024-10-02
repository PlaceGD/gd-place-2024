import { HttpsError, onRequest } from "firebase-functions/v2/https";
import { onInit } from "firebase-functions/v2/core";
import Mailjet, { SendEmailV3_1 } from "node-mailjet";
import { LogGroup } from "./utils/logger";
import {
    KofiTxId,
    MAX_GRADIENT_STOPS,
    VALID_KOFI_TRANSACTION_ID,
} from "shared-lib/kofi";
import { onCallAuthLogger } from "./utils/on_call";
import { GradientReq, KofiReq } from "shared-lib/cloud_functions";
import { GRADIENT_COOLDOWN_SECONDS } from "shared-lib/user";
import { smartDatabase } from "src";
import { getCheckedUserDetails } from "./utils/utils";
import Error from "./utils/errors";

let mailjetClient: Mailjet;
onInit(() => {
    mailjetClient = new Mailjet({
        apiKey: process.env["MAILJET_API_KEY"]!,
        apiSecret: process.env["MAILJET_API_SECRET"]!,
    });
});

export type KofiDonation = {
    verification_token: string;
    message_id: string;
    timestamp: string;
    is_public: boolean;
    from_name: string;
    message: string;
    amount: string;
    url: string;
    email: string;
    currency: string;
    is_subscription_payment: boolean;
    is_first_subscription_payment: boolean;
    kofi_transaction_id: KofiTxId;
} & (
    | {
          type: "Donation";
          shop_items: null;
          tier_name: null;
          shipping: null;
      }
    | {
          type: "Subscription";
          shop_items: null;
          tier_name: string;
          shipping: null;
      }
    | {
          type: "Shop Order";
          shop_items: {
              direct_link_code: string;
              variation_name: string;
              quantity: number;
          }[];
          tier_name: string;
          shipping: {
              full_name: string;
              street_address: string;
              city: string;
              state_or_province: string;
              postal_code: string;
              country: string;
              country_code: string;
              telephone: string;
          };
      }
);

const EMAIL_BODY_HTML = `
<h1>Thank you for donating to GD Place ❤</h1>
<p><strong>You should now be able to change the colors of your username in the settings menu!</strong></p>
<p>If the website asks you for a code, use this:</p>
<h2 style="width: min-content; background: #00000016; padding: 10px; border-radius: 5px; user-select: all;">00000000-1111-2222-3333-444444444444</h2>
`;

// #region onKofiDonation
export const onKofiDonation = onRequest(
    { cors: ["ko-fi.com"] },
    async (request, response) => {
        const logger = new LogGroup("onKofiDonation");

        if (
            request.headers["content-type"] !==
            "application/x-www-form-urlencoded"
        ) {
            logger.error("Incorrect content-type header");
            logger.finish();

            response.status(400).send();
            return;
        }

        const data = request.body.data;

        logger.debug(data);

        if (!data) {
            logger.error("Missing data in body");
            logger.finish();

            response.status(400).send();
            return;
        }

        let jsonData: KofiDonation | null = null;

        try {
            jsonData = JSON.parse(data);
        } catch (e) {
            logger.error(`JSON failed to parse: ${e}`);
            logger.finish();

            response.status(400).send();
            return;
        }

        if (jsonData == null) {
            logger.error("JSON data is null");
            logger.finish();

            response.status(400).send();
            return;
        }

        const verificationToken = jsonData.verification_token;

        if (verificationToken !== process.env["KOFI_VERIFICATION_TOKEN"]) {
            logger.error(`Invalid verification token: ${verificationToken}`);
            logger.finish();

            response.status(400).send();
            return;
        }

        const timestamp = Date.parse(jsonData.timestamp);

        if (isNaN(timestamp) || timestamp > Date.now()) {
            logger.error(`Invalid timestamp: ${timestamp}`);
            logger.finish();

            response.status(400).send();
            return;
        }

        const txId = jsonData.kofi_transaction_id;

        if (!VALID_KOFI_TRANSACTION_ID.test(txId)) {
            logger.error(`Invalid transaction id: ${txId}`);
            logger.finish();

            response.status(400).send();
            return;
        }

        const db = smartDatabase();

        // try first by checking the username of the kofi donator against our database
        const kofiDonatorUsername = jsonData.from_name.toLowerCase();
        const kofiDonatorMaybeAccountData = (
            await db.ref(`/userName/${kofiDonatorUsername}`).get()
        ).val();

        // if the user exists then we (most likely) found a match, so make them a donator
        // its very possible (though extremely unlikely) that the donator could have a username that points
        // to a different user in the database...
        if (kofiDonatorMaybeAccountData != null) {
            db.ref(
                `/userDetails/${kofiDonatorMaybeAccountData.uid}/hasDonated`
            ).set(true);

            // in case someones kofi username matches someone elses gd place account, we want to keep track of the account that
            // recieved the perks so as to disable them later when the real donator claims the transaction id
            db.ref(`/claimedDonations/${kofiDonatorMaybeAccountData.uid}`).set(
                txId
            );
        }

        // set it to an empty string if no account match was found
        await db
            .ref(`activeDonations/${txId}`)
            .set(kofiDonatorMaybeAccountData?.uid ?? "");

        // ...in that case, we still send an email to the original donator with their transaction id.
        // most likely, the user that incorrectly got marked as a donator would not be expecting it and therefore would not notice their new powers.
        // once the real donator then inputs the transaction id we can remove the donator status from the incorrect user.
        // in the case where the kofi usernames matches the _correct_ gd place user, once they change their gradient, we remove the transaction id
        // from the database so the donation can't set more than 1 user to donator status
        // there *is* still edge cases in this but they are going to be so rare and theres not much we can do given the limited info from kofi
        const emailBody: SendEmailV3_1.Body = {
            Messages: [
                {
                    From: {
                        Email: "donations-noreply@place.gd",
                        Name: "GD Place",
                    },
                    To: [
                        {
                            Email: jsonData.email,
                        },
                    ],
                    Subject: `GD Place Donation! (#${jsonData.message_id.split("-")[1]})`,
                    HTMLPart: EMAIL_BODY_HTML,
                },
            ],
        };

        const emailRequest = mailjetClient
            .post("send", { version: "v3.1" })
            .request(emailBody);

        emailRequest.catch((e: SendEmailV3_1.ResponseError) => {
            logger.error(
                `Failed to send email to: ${jsonData.email}. ${e.ErrorMessage} (${e.ErrorCode}).`
            );
        });

        response.status(200).send();
    }
);

// #region submitKofiTxId
export const submitKofiTxId = onCallAuthLogger<KofiReq>(
    "submitKofiTxId",
    async (request, logger) => {
        const db = smartDatabase();
        const txId = request.data.txId;

        const userDetails = await getCheckedUserDetails(db, request.auth.uid);

        if (!VALID_KOFI_TRANSACTION_ID.test(txId)) {
            logger.error(`Invalid transaction id format: ${txId}`);
            throw Error.code(104, "invalid-argument");
        }

        const donatorUserIdRef = db.ref(`activeDonations/${txId}`);
        const donatorUserId = (await donatorUserIdRef.get()).val();
        // null means the txid doesnt exist, empty string means no user match
        if (donatorUserId == null) {
            logger.error(`Invalid transaction id: ${txId}`);
            throw Error.code(104, "invalid-argument");
        }

        // if the wrong user claimed the donation, this will reset their status.
        // technically, if the the donator put in a username that matched their gd place account and then
        // also claimed the transaction id,
        // this would run and set their reset their status but they wouldnt have anything to reset so its not an issue
        if (donatorUserId != request.auth.uid && donatorUserId !== "") {
            await db.ref(`/userDetails/${donatorUserId}/hasDonated`).set(false);
        }

        await userDetails.ref.child("hasDonated").set(true);

        // remove the data associated
        await donatorUserIdRef.remove();
        await db.ref(`/claimedDonations/${donatorUserId}`).remove();
    }
);

const VALID_GRADIENT = new RegExp(
    `linear-gradient\\(\\d+deg((,\\s*)?#[a-fA-F0-9]{6} \\d{1,3}%){2,${MAX_GRADIENT_STOPS}}\\)`
);

// #region changeNameGradient
export const changeNameGradient = onCallAuthLogger<GradientReq>(
    "changeNameGradient",
    async (request, logger) => {
        const db = smartDatabase();
        const data = request.data;

        // await validateTurnstile(
        //     process.env["TURNSTILE_LOGIN_PRIV_KEY"],
        //     data.turnstileResp,
        //     request.rawRequest,
        //     logger
        // );

        const userDetails = await getCheckedUserDetails(db, request.auth.uid);

        const timeNextGradient = userDetails.val.epochNextGradient;
        if (Date.now() < timeNextGradient) {
            throw Error.code(205, "permission-denied");
        }

        if (!userDetails.val.hasDonated) {
            throw Error.code(207, "permission-denied");
        }

        // remove the associated donation info once the user has updated their gradient
        // for safety
        const txIdRef = db.ref(`/claimedDonations/${request.auth.uid}`);
        const txId = (await txIdRef.get()).val();
        if (txId != null) {
            await txIdRef.remove();
            await db.ref(`/activeDonations/${txId}`).remove();
        }

        const grad = data.grad;

        if (!VALID_GRADIENT.test(grad)) {
            throw Error.code(105, "invalid-argument");
        }

        let nextGradient = Date.now();
        nextGradient += GRADIENT_COOLDOWN_SECONDS * 1000;

        await Promise.all([
            userDetails.ref.child("epochNextGradient").set(nextGradient),
            db
                .ref(
                    `userName/${userDetails.val.username.toLowerCase()}/displayColor`
                )
                .set(grad),
        ]);
    }
);
