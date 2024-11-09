import { LogSeverity, write } from "firebase-functions/logger";

export enum Level {
    DEBUG = "🐛",
    INFO = "ℹ️",
    ERROR = "❌",
}

export class LogGroup {
    private readonly label: string;
    private logs: { message: string; level: Level }[] = [];

    constructor(label: string = "") {
        this.label = label;
    }

    private addNewLog(message: string, level: Level) {
        this.logs.push({ message, level });
    }

    info(...messages: { toString: () => string }[]) {
        this.addNewLog(messages.join(" "), Level.INFO);
    }

    debug(...messages: { toString: () => string }[]) {
        this.addNewLog(messages.join(" "), Level.DEBUG);
    }

    error(...messages: { toString: () => string }[]) {
        this.addNewLog(messages.join(" "), Level.ERROR);
    }

    finish(level?: Level) {
        let l = level;
        if (!l) l = Level.INFO;
        write({
            severity: Object.keys(Level)[
                Object.values(Level).indexOf(l)
            ] as LogSeverity,
            message: `--- ${this.label} ---\n${this.logs
                .map(log => `[${log.level}]: ${log.message}`)
                .join("\n")}`,
        });
        this.logs = [];
    }
}
