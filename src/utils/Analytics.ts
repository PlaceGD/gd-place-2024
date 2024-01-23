import { getAnalytics, logEvent as fbLogEvent } from "firebase/analytics";
import { app } from "../firebase/Firebase";

const analytics = getAnalytics(app);

export enum AnalyticEvent {
    PAGE_VIEW = "page_view",
    PAGE_LOAD_TIME = "page_load_time",
}

// any event that requires no data
interface Bare {
    [AnalyticEvent.PAGE_VIEW]: undefined;
}

// any event that requires data
interface Payloads {
    [AnalyticEvent.PAGE_LOAD_TIME]: { time: number };
}

interface All extends Bare, Payloads {}

export function logEvent<T extends keyof Payloads>(
    event: T,
    payload: Payloads[T]
): void;
export function logEvent<T extends keyof Bare>(event: T): void;

export function logEvent<T extends keyof All>(
    event: T,
    payload: All[T] = undefined as never
): void {
    fbLogEvent(analytics, event as string, payload);
}
