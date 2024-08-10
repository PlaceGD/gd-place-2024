export enum TokenStatus {
    NoToken,
    Error,
}

let turnstileToken: string | TokenStatus = TokenStatus.NoToken;
let turnstilePromiseFuncs: { res: (t: string) => void; rej: () => void }[] = [];

export const setTurnstileToken = (v: string | TokenStatus) => {
    turnstileToken = v;
};
// export const getTurnstileTokenOrStatus = () => turnstileToken;
export const getNewTurnstileToken: () => Promise<string> = () =>
    new Promise((res, rej) => {
        turnstilePromiseFuncs.push({ res, rej });
    });

export const setTurnstileResetFunction = (f: () => void) => {
    const resetTurnstile = () => {
        turnstileToken = TokenStatus.NoToken;
        f();
    };

    setInterval(() => {
        if (turnstileToken === TokenStatus.Error) {
            let fs = turnstilePromiseFuncs.pop();
            if (fs == undefined) {
                return;
            }
            fs.rej();
            resetTurnstile();
        } else if (typeof turnstileToken == "string") {
            let fs = turnstilePromiseFuncs.pop();
            if (fs == undefined) {
                return;
            }
            fs.res(turnstileToken);
            resetTurnstile();
        }
    }, 100);
};
