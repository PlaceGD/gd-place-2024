import * as wasm from "wasm-lib";

let state: wasm.StateWrapper | null = null;

type Callback = (s: wasm.StateWrapper) => void;

let callbacks: Record<number, Callback> = {};
let callbackCount = 0;
class CallbackID {
    constructor(public id: number) {}
    remove() {
        delete callbacks[this.id];
    }
}

export const addCallback = (f: Callback) => {
    callbacks[callbackCount] = f;
    callbackCount += 1;
    return new CallbackID(callbackCount - 1);
};
export const runCallbacks = () => {
    if (state != null) {
        for (let i of Object.values(callbacks)) {
            i(state);
        }
    }
};

export const loadState = (s: wasm.StateWrapper) => {
    state = s;
};

export const withState = <T>(f: (state: wasm.StateWrapper) => T): T => {
    if (state != null) {
        return f(state);
    }
    return undefined as any;
};
