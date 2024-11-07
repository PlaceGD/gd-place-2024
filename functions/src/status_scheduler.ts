import { onCall } from "firebase-functions/https";

export const getServerTime = onCall<{}, Promise<number>>(async () => {
    return Date.now();
});
