import {
    CallableRequest,
    onCall,
    CallableFunction,
    HttpsError,
    onRequest,
} from "firebase-functions/v2/https";
import { LogGroup } from "./utils/logger";
import { KofiTxId, VALID_KOFI_TRANSACTION_ID } from "shared-lib/kofi";
import { database } from "firebase-admin";
import { ref } from "./utils/database";

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

        const verification_token = jsonData.verification_token;

        if (verification_token !== process.env["KOFI_VERIFICATION_TOKEN"]) {
            logger.error(`Invalid verification token: ${verification_token}`);
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
        const tx_id = jsonData.kofi_transaction_id;

        if (!VALID_KOFI_TRANSACTION_ID.test(tx_id)) {
            logger.error(`Invalid transaction id: ${tx_id}`);
            logger.finish();

            response.status(400);
        }

        const db = database();

        ref(db, "activeDonations").push(tx_id);

        // TODO: why does kofi call this function twice
        response.status(200);
    }
);
