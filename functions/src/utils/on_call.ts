import {
    CallableRequest,
    onCall as onCallF,
    CallableFunction,
    HttpsError,
    CallableOptions,
} from "firebase-functions/v2/https";
import { Level, LogGroup } from "./logger";
import Error from "./errors";

type WithRequired<T, K extends keyof T> = T & { [P in K]-?: T[P] };

type AuthedCallableRequest<T> = WithRequired<CallableRequest<T>, "auth">;

const ON_CALL_OPTIONS = //{};
    {
        minInstances: 3,
        memory: "1GiB",
        cpu: 2,
        concurrency: 900,
        region: ["us-central1", "europe-central2", "asia-southeast1"],
    } satisfies CallableOptions;

// TOOD: set multiple regions, memory, min instances?
export const onCallAuth = <T, Return = Promise<void>>(
    handler: (request: AuthedCallableRequest<T>) => Return
): CallableFunction<T, Return> => {
    return onCallF(ON_CALL_OPTIONS, request => {
        if (!request.auth) {
            throw Error.code(210, "permission-denied");
        }
        return handler(request as AuthedCallableRequest<T>);
    });
};

export const onCallAuthLogger = <T, Return = Promise<void>>(
    id: string,
    handler: (request: AuthedCallableRequest<T>, logger: LogGroup) => Return
): CallableFunction<T, Return> => {
    const logger = new LogGroup(id);

    return onCallF(ON_CALL_OPTIONS, request => {
        if (!request.auth) {
            logger.finish(Level.ERROR);
            throw Error.code(210, "permission-denied");
        }

        try {
            const ret = handler(request as AuthedCallableRequest<T>, logger);
            logger.finish();
            return ret;
        } catch (e: any) {
            logger.error("Captured exception", e.details.message ?? e);
            logger.finish(Level.ERROR);
            throw e;
        }
    });
};
