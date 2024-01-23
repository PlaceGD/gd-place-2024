import { LogSeverity, write } from "firebase-functions/logger";

export enum Level {
    DEBUG = "🐛",
    INFO = "ℹ️",
    ERROR = "❌",
}

export class LogGroup {
    private logs: { message: any; level: Level }[] = [];

    constructor() {}

    debug(...messages: any[]) {
        this.logs = [
            ...this.logs,
            ...messages.map(m => ({ message: m, level: Level.DEBUG })),
        ];
    }

    error(...messages: any[]) {
        this.logs = [
            ...this.logs,
            ...messages.map(m => ({ message: m, level: Level.ERROR })),
        ];
    }

    finish(level?: Level) {
        let l = level;
        if (!l) l = Level.INFO;
        write({
            severity: Object.keys(Level)[
                Object.values(Level).indexOf(l)
            ] as LogSeverity,
            message: this.logs
                .map(log => `[${log.level}]: ${log.message}`)
                .join("\n"),
        });
    }
}
