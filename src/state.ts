import * as wasm from "wasm-lib";

type Callback = (s: wasm.StateWrapper) => void;

const callbacks: Callback[] = [];

export const addCallback = (f: Callback) => {
    callbacks.push(f);
};
export const runCallbacks = (state: wasm.StateWrapper) => {
    for (let i of callbacks) {
        i(state);
    }
};
