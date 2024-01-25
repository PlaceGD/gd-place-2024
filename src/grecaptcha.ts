export const SITE_KEY = "6LfeoFspAAAAAFDlFGtTLIt6Xs_2WYVspTwo9640";

declare global {
    const grecaptcha: ReCaptchaInstance;

    interface Window {
        grecaptcha: ReCaptchaInstance;
    }
}

export interface ReCaptchaInstance {
    ready: (callback: () => void) => void;

    execute: (siteKey: string, options: ExecuteOptions) => Promise<string>;

    enterprise: Omit<ReCaptchaInstance, "enterprise">;
}

export declare interface ExecuteOptions {
    action?: string;
}
