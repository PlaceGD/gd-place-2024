import { getAnalytics, logEvent as fbLogEvent } from "firebase/analytics";
import { app } from "../firebase/Firebase";

export enum AnalyticEvent {
    PAGE_VIEW = "page_view",
    PAGE_LOAD_TIME = "page_load_time",
}

type EventParamMap = {
    [AnalyticEvent.PAGE_LOAD_TIME]: {
        time: number;
    };
};

type EventsWithParams = keyof EventParamMap;

const analytics = getAnalytics(app);

// export const logEvent = (
//     event: T,
//     params: GetEventParamType<T>
// ) => {
//     fbLogEvent(analytics, event as string, params);
// };

export function logEvent<T extends EventsWithParams>(
    event: AnalyticEvent,
    params: EventParamMap[T]
): void;

export function logEvent(event: AnalyticEvent, params?: never): void;

export function logEvent(event: AnalyticEvent, params: any): void {
    fbLogEvent(analytics, event as string, params);
}

// export const logEvent = <T extends Events>(
//     event: T,
//     params: GetEventParamType<T>
// ) => {
//     fbLogEvent(analytics, event as string, params);
// };

logEvent(AnalyticEvent.PAGE_LOAD_TIME);
