import { HttpsError, onRequest } from "firebase-functions/v2/https";
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

// #region onKofiDonation
export const onKofiDonation = onRequest(
    { cors: ["ko-fi.com"] },
    (request, response) => {
        const logger = new LogGroup("onKofiDonation");

        if (
            request.headers["content-type"] !==
            "application/x-www-form-urlencoded"
        ) {
            logger.error("Incorrect content-type header");
            logger.finish();

            response.status(400);
            return;
        }

        const data = request.body.data;

        if (!data) {
            logger.error("Missing data in body");
            logger.finish();

            response.status(400);
            return;
        }

        let jsonData: KofiDonation | null = null;

        try {
            jsonData = JSON.parse(data);
        } catch (e) {
            logger.error(`JSON failed to parse: ${e}`);
            logger.finish();

            response.status(400);
            return;
        }

        if (jsonData == null) {
            logger.error("JSON data is null");
            logger.finish();

            response.status(400);
            return;
        }

        const verificationToken = jsonData.verification_token;

        if (verificationToken !== process.env["KOFI_VERIFICATION_TOKEN"]) {
            logger.error(`Invalid verification token: ${verificationToken}`);
            logger.finish();

            response.status(400);
            return;
        }

        const timestamp = Date.parse(jsonData.timestamp);

        if (isNaN(timestamp) || timestamp > Date.now()) {
            logger.error(`Invalid timestamp: ${timestamp}`);
            logger.finish();

            response.status(400);
            return;
        }

        // TODO: should this be message id?
        const txId = jsonData.kofi_transaction_id;

        if (!VALID_KOFI_TRANSACTION_ID.test(txId)) {
            logger.error(`Invalid transaction id: ${txId}`);
            logger.finish();

            response.status(400);
        }

        const db = smartDatabase();
        db.ref(`activeDonations/${txId}`).set(1);

        // TODO: why does kofi call this function twice
        response.status(200);
    }
);

// #region submitKofiTxId
export const submitKofiTxId = onCallAuthLogger<KofiReq>(
    "submitKofiTxId",
    async (request, logger) => {
        const db = smartDatabase();
        const txId = request.data.txId;

        const { userDetailsRef } = await getCheckedUserDetails(
            db,
            request.auth.uid
        );

        if (!VALID_KOFI_TRANSACTION_ID.test(txId)) {
            logger.error(`Invalid transaction id format: ${txId}`);
            throw new HttpsError(
                "invalid-argument",
                "Invalid transaction ID format"
            );
        }

        const data = (await db.ref(`activeDonations/${txId}`).get()).val();
        if (data == null || data !== 1) {
            logger.error(`Invalid transaction id: ${txId}`);
            throw new HttpsError("invalid-argument", "Invalid transaction ID");
        }

        userDetailsRef.child("hasDonated").set(true);
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

        const { userDetails, userDetailsRef } = await getCheckedUserDetails(
            db,
            request.auth.uid
        );

        const timeNextGradient = userDetails.epochNextGradient;
        if (Date.now() < timeNextGradient ?? 0) {
            throw new HttpsError(
                "permission-denied",
                "Cannot update before timer expired"
            );
        }

        if (!userDetails.hasDonated) {
            throw new HttpsError(
                "permission-denied",
                "Cannot change gradient of user without donation"
            );
        }

        const grad = data.grad;

        if (!VALID_GRADIENT.test(grad)) {
            throw new HttpsError("permission-denied", "Invalid gradient");
        }

        let nextGradient = Date.now();
        nextGradient += GRADIENT_COOLDOWN_SECONDS * 1000;
        userDetailsRef.child("epochNextGradient").set(nextGradient);

        db.ref(
            `userName/${userDetails.username.toLowerCase()}/displayColor`
        ).set(grad);
    }
);
