import { onCall } from "firebase-functions/v2/https";

export const getServerTime = onCall<{}, Promise<number>>(async () => {
    return Date.now();
});
