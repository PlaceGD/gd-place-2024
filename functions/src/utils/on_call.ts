import {
    CallableRequest,
    onCall as onCallF,
    CallableFunction,
    HttpsError,
} from "firebase-functions/v2/https";
import { Level, LogGroup } from "./logger";

type WithRequired<T, K extends keyof T> = T & { [P in K]-?: T[P] };

type AuthedCallableRequest<T> = WithRequired<CallableRequest<T>, "auth">;

export const onCallAuth = <T, Return = Promise<any>>(
    handler: (request: AuthedCallableRequest<T>) => Return
): CallableFunction<T, Return> => {
    return onCallF(request => {
        if (!request.auth) {
            throw new HttpsError(
                "unauthenticated",
                "User is not authenticated"
            );
        }
        return handler(request as AuthedCallableRequest<T>);
    });
};

export const onCallAuthLogger = <T, Return = Promise<any>>(
    id: string,
    handler: (request: AuthedCallableRequest<T>, logger: LogGroup) => Return
): CallableFunction<T, Return> => {
    const logger = new LogGroup(id);
    return onCallF(request => {
        if (!request.auth) {
            logger.finish(Level.ERROR);
            throw new HttpsError(
                "unauthenticated",
                "User is not authenticated"
            );
        }

        try {
            const ret = handler(request as AuthedCallableRequest<T>, logger);
            logger.finish();
            return ret;
        } catch (e: any) {
            logger.finish(Level.ERROR);
            throw e;
        }
    });
};
